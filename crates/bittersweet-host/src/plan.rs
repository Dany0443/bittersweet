//! Turn a host probe into a concrete tuning plan.
//!
//! The plan is data: an ordered list of environment variables, a VRAM
//! budget, and advisories. `setup.sh` writes it to `bittersweet.env`; the
//! launcher sources that file. Nothing in this module touches a game binary
//! or writes to the kernel.

use crate::gpu::{
    self, AmdFamily, Vendor, max_wave_lanes, mesh_pipelines_safe, mesh_refusal,
    needs_wave64_emulation,
};
use crate::kernel::{self, KernelAdvice};
use crate::probe::HostReport;
use crate::vram::{self, VramBudget};
use std::fmt::Write as _;
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level {
    Info,
    Warn,
    Critical,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Advisory {
    pub level: Level,
    pub text: String,
}

#[derive(Clone, Debug)]
pub struct TuningPlan {
    pub profile: HostReport,
    pub budget: VramBudget,
    pub mesh_safe: bool,
    pub mesh_refusal: Option<&'static str>,
    pub wave64_emulation: bool,
    pub proton: bool,
    pub env: Vec<(String, String)>,
    pub advisories: Vec<Advisory>,
    pub cache_root: String,
}

impl TuningPlan {
    pub fn env_file(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            out,
            "# bittersweet host plan — generated; re-run `bittersweet plan --write`"
        );
        let _ = writeln!(
            out,
            "# host: {} device 0x{:04x} ({}), {} MiB VRAM",
            self.profile.vendor.label(),
            self.profile.device,
            amd_family_label(self.profile.amd_family),
            self.profile.vram_mb
        );
        let _ = writeln!(out, "# cache: {}", self.cache_root);
        for (key, value) in &self.env {
            let _ = writeln!(out, "export {key}={}", shell_quote(value));
        }
        out
    }

    pub fn advisories_text(&self) -> String {
        let mut out = String::new();
        for a in &self.advisories {
            let tag = match a.level {
                Level::Info => "info",
                Level::Warn => "warn",
                Level::Critical => "critical",
            };
            let _ = writeln!(out, "[{tag}] {}", a.text);
        }
        out
    }

    pub fn to_json(&self) -> String {
        let mut out = String::new();
        out.push('{');
        let _ = write!(
            out,
            "\"host\":{{\"vendor\":\"{}\",\"device\":{},\"vram_mb\":{},\"amd_family\":\"{}\"}}",
            self.profile.vendor.label(),
            self.profile.device,
            self.profile.vram_mb,
            amd_family_label(self.profile.amd_family)
        );
        let _ = write!(
            out,
            ",\"vram\":{{\"texture_mb\":{},\"mip_bias\":{},\"pressure\":\"{:?}\"}}",
            self.budget.texture_mb, self.budget.mip_bias, self.budget.pressure
        );
        let _ = write!(
            out,
            ",\"shader\":{{\"mesh_pipelines\":{},\"wave64_emulation\":{},\"max_wave_lanes\":{}}}",
            self.mesh_safe,
            self.wave64_emulation,
            max_wave_lanes(self.profile.vendor, self.profile.amd_family)
        );
        out.push_str(",\"env\":{");
        for (i, (k, v)) in self.env.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            let _ = write!(out, "\"{}\":\"{}\"", json_escape(k), json_escape(v));
        }
        out.push('}');
        out.push_str(",\"advisories\":[");
        for (i, a) in self.advisories.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            let _ = write!(
                out,
                "{{\"level\":\"{:?}\",\"text\":\"{}\"}}",
                a.level,
                json_escape(&a.text)
            );
        }
        out.push_str("]}");
        out
    }
}

fn amd_family_label(f: AmdFamily) -> &'static str {
    match f {
        AmdFamily::Gcn => "gfx9",
        AmdFamily::Rdna1 => "gfx10.1",
        AmdFamily::Rdna2Plus => "gfx10.3+",
        AmdFamily::Unknown => "unknown AMD",
    }
}

fn shell_quote(value: &str) -> String {
    let escaped = value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('$', "\\$")
        .replace('`', "\\`");
    format!("\"{escaped}\"")
}

fn json_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Build a plan from a probe. `cache_root` is where shader and pipeline
/// caches live; `proton` selects Proton/GE-specific variables.
pub fn build(profile: &HostReport, cache_root: &Path, proton: bool) -> TuningPlan {
    let budget = vram::for_vram(profile.vram_mb);
    let vendor = profile.vendor;
    let mesh_safe = mesh_pipelines_safe(vendor, profile.device);
    let mesh_refusal = mesh_refusal(vendor, profile.device);
    let wave64_emulation = needs_wave64_emulation(vendor, profile.amd_family);

    let cache = cache_root.display().to_string();
    let mut env: Vec<(String, String)> = Vec::new();
    // Accept &str, String and format!() alike; `to_string` is cheap here.
    macro_rules! put {
        ($k:expr, $v:expr $(,)?) => {
            env.push(($k.to_owned(), $v.to_string()))
        };
    }

    // Keep Wine quiet; the runtime has its own log.
    put!("WINEDEBUG", "-all");
    put!("WINE_LARGE_ADDRESS_AWARE", "1");
    if proton {
        put!("PROTON_LOCAL_SHADER_CACHE", "1");
        put!("WINEESYNC", "1");
        put!("WINEFSYNC", "1");
    }

    // Shader and pipeline caches, one directory each so a corrupted cache
    // can be removed without touching game data.
    put!("DXVK_STATE_CACHE", "1");
    put!("DXVK_STATE_CACHE_PATH", format!("{cache}/dxvk-state"));
    put!("DXVK_LOG_PATH", format!("{cache}/dxvk-log"));
    put!("VKD3D_SHADER_CACHE_PATH", format!("{cache}/vkd3d"));
    put!("MESA_SHADER_CACHE_DIR", format!("{cache}/mesa"));
    put!("MESA_SHADER_CACHE_MAX_SIZE", "12G");

    // The runtime reads these; they are policy, not driver options.
    put!("BITTERSWEET_VRAM_BUDGET_MB", budget.texture_mb.to_string());
    put!("BITTERSWEET_MIP_BIAS", budget.mip_bias.to_string());
    put!(
        "BITTERSWEET_PIPELINE_KEY_CAP",
        crate::pipeline::EARLY_MAX.to_string()
    );
    put!("BITTERSWEET_SHADER_WARM_BUDGET_SECS", "45");
    put!(
        "BITTERSWEET_WAVE64",
        if wave64_emulation {
            "emulate"
        } else {
            "native"
        },
    );
    put!(
        "BITTERSWEET_MESH_PIPELINES",
        if mesh_safe { "1" } else { "0" },
    );
    // Ray tracing is an experiment on every tracked build in the table.
    put!("BITTERSWEET_ENABLE_RT", "0");

    // A card whose texture budget is well below its own size loses time to
    // eviction and long waits. These are the settings the 8 GB port
    // (Linox-IDK/LNX-victory, commit acea637) runs with; the inherited
    // runtime reads all of them.
    if matches!(budget.pressure, crate::vram::Pressure::High | crate::vram::Pressure::Severe) {
        put!("VICTORY_TEXTURE_NO_WAIT", "1");
        put!("VICTORY_PIPELINE_WAIT_MS", "50");
        put!("VICTORY_SHADER_LOOP_LIMIT", "1024");
        put!("VICTORY_CARD_REHASH_RUNS", "1");
        put!("VICTORY_JOIN_COPIES", "1");
    }

    match vendor {
        Vendor::Amd => {
            put!("AMD_VULKAN_ICD", "RADV");
            // Graphics pipeline library: a slower first compile, much less
            // stutter once the cache is warm.
            put!("RADV_PERFTEST", "gpl");
            if budget.texture_mb < profile.vram_mb * 7 / 10 {
                // Do not park uploads in host-visible memory on cards that
                // are already tight; vkd3d-proton honours this.
                put!("VKD3D_CONFIG", "no_upload_hvv");
            }
            if let Some(family) = mesh_refusal {
                put!("BITTERSWEET_MESH_REFUSAL", family);
            }
        }
        Vendor::Nvidia => {
            put!("PROTON_ENABLE_NVAPI", "1");
            put!("DXVK_ENABLE_NVAPI", "1");
            put!("__GL_SHADER_DISK_CACHE", "1");
            put!("__GL_SHADER_DISK_CACHE_PATH", format!("{cache}/nvidia"));
            put!("__GL_SHADER_DISK_CACHE_SKIP_CLEANUP", "1");
            put!(
                "__GL_SHADER_DISK_CACHE_SIZE",
                (budget.texture_mb * 1024 * 1024 / 2).to_string(),
            );
        }
        Vendor::Intel => {
            put!("ANV_QUEUE_THREAD_DISABLE", "0");
        }
        _ => {}
    }

    let mut advisories = Vec::new();
    match kernel::assess(profile.max_map_count) {
        KernelAdvice::TooLow(v) => {
            advisories.push(Advisory {
                level: Level::Critical,
                text: format!(
                    "{} — the runtime maps many small views of title memory; \
                     apply the sysctl below before a long session.",
                    KernelAdvice::TooLow(v).summary()
                ),
            });
            if let Some(line) = kernel::assess(profile.max_map_count).suggested_sysctl() {
                advisories.push(Advisory {
                    level: Level::Info,
                    text: format!("suggested: /etc/sysctl.d/99-bittersweet.conf -> {line}"),
                });
            }
        }
        KernelAdvice::Acceptable(v) => advisories.push(Advisory {
            level: Level::Warn,
            text: KernelAdvice::Acceptable(v).summary(),
        }),
        other => advisories.push(Advisory {
            level: Level::Info,
            text: other.summary(),
        }),
    }

    if !mesh_safe {
        advisories.push(Advisory {
            level: Level::Warn,
            text: format!(
                "mesh pipelines disabled for {}: RADV does not expose VK_EXT_mesh_shader \
                 on this family (and aborts on Vega). The title's non-mesh path runs; \
                 draws that name a refused pipeline are skipped.",
                mesh_refusal.unwrap_or("this AMD family")
            ),
        });
    }
    if wave64_emulation {
        advisories.push(Advisory {
            level: Level::Info,
            text: "64-lane compute will be lowered to 32-lane waves; expect extra \
                   shader-compile time on the first run."
                .to_owned(),
        });
    }
    if budget.pressure != vram::Pressure::Low {
        advisories.push(Advisory {
            level: Level::Warn,
            text: format!(
                "{} MiB VRAM: texture budget capped at {} MiB. Lower output resolution \
                 before raising it; ray tracing is off.",
                budget.total_mb, budget.texture_mb
            ),
        });
    }
    if profile.vendor == Vendor::Nvidia {
        advisories.push(Advisory {
            level: Level::Info,
            text: "NVIDIA: the proprietary driver ignores MESA_*; the __GL_* cache path \
                   above is the one that applies."
                .to_owned(),
        });
    }

    TuningPlan {
        profile: profile.clone(),
        budget,
        mesh_safe,
        mesh_refusal,
        wave64_emulation,
        proton,
        env,
        advisories,
        cache_root: cache,
    }
}

/// One-line capability summary for `doctor`.
pub fn capability_line(profile: &HostReport) -> String {
    format!(
        "{} device 0x{:04x} ({}), {} MiB VRAM, mesh pipelines {}, wave64 {}",
        profile.vendor.label(),
        profile.device,
        amd_family_label(profile.amd_family),
        profile.vram_mb,
        if mesh_pipelines_safe(profile.vendor, profile.device) {
            "enabled"
        } else {
            "refused"
        },
        if needs_wave64_emulation(profile.vendor, profile.amd_family) {
            "emulated"
        } else {
            "native"
        }
    )
}

/// Exposed for callers that already have ids and no sysfs.
pub fn plan_for_ids(vendor_id: u16, device_id: u16, vram_mb: u64, cache: &Path) -> TuningPlan {
    let profile = HostReport {
        vendor: Vendor::from_pci(vendor_id),
        device: device_id,
        amd_family: gpu::amd_family(device_id),
        vram_mb,
        ..HostReport::default()
    };
    build(&profile, cache, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_8gb_nvidia_plan_turns_on_nvapi_and_caps_textures() {
        let plan = plan_for_ids(0x10de, 0x2684, 8192, Path::new("/tmp/bs"));
        let get = |k: &str| {
            plan.env
                .iter()
                .find(|(key, _)| key == k)
                .map(|(_, v)| v.as_str())
        };
        assert_eq!(get("PROTON_ENABLE_NVAPI"), Some("1"));
        assert_eq!(get("BITTERSWEET_VRAM_BUDGET_MB"), Some("4915"));
        assert_eq!(get("BITTERSWEET_WAVE64"), Some("emulate"));
        assert_eq!(get("BITTERSWEET_MESH_PIPELINES"), Some("1"));
        assert!(plan.budget.pressure == vram::Pressure::High);
    }

    #[test]
    fn an_rx_9070_xt_keeps_mesh_pipelines_and_radv() {
        let plan = plan_for_ids(0x1002, 0x7550, 16384, Path::new("/tmp/bs"));
        let get = |k: &str| {
            plan.env
                .iter()
                .find(|(key, _)| key == k)
                .map(|(_, v)| v.as_str())
        };
        assert_eq!(get("AMD_VULKAN_ICD"), Some("RADV"));
        assert_eq!(get("RADV_PERFTEST"), Some("gpl"));
        assert_eq!(get("BITTERSWEET_MESH_PIPELINES"), Some("1"));
        assert_eq!(get("BITTERSWEET_WAVE64"), Some("native"));
        assert!(!get("VKD3D_CONFIG").is_some_and(|v| v.contains("no_upload_hvv")));
    }

    #[test]
    fn rdna3_and_rdna1_keep_their_vram_and_mesh_policies() {
        for (device, vram, mesh, budget, bias) in [
            (0x744c, 24576, "1", 18432, 0),
            (0x7480, 8192, "1", 4915, 1),
            (0x731f, 8192, "0", 4915, 1),
        ] {
            let plan = plan_for_ids(0x1002, device, vram, Path::new("/tmp/bs"));
            let get = |key: &str| {
                plan.env
                    .iter()
                    .find(|(k, _)| k == key)
                    .map(|(_, v)| v.as_str())
            };
            assert_eq!(get("BITTERSWEET_MESH_PIPELINES"), Some(mesh));
            assert_eq!(get("BITTERSWEET_WAVE64"), Some("native"));
            assert_eq!(get("MESA_SHADER_CACHE_MAX_SIZE"), Some("12G"));
            assert_eq!(get("AMD_VULKAN_ICD"), Some("RADV"));
            assert_eq!(plan.budget.texture_mb, budget);
            assert_eq!(plan.budget.mip_bias, bias);
        }
    }

    #[test]
    fn a_vega_plan_refuses_mesh_and_lowers_vram_uploads() {
        let plan = plan_for_ids(0x1002, 0x687f, 8192, Path::new("/tmp/bs"));
        let get = |k: &str| {
            plan.env
                .iter()
                .find(|(key, _)| key == k)
                .map(|(_, v)| v.as_str())
        };
        assert_eq!(get("BITTERSWEET_MESH_PIPELINES"), Some("0"));
        assert_eq!(get("BITTERSWEET_MESH_REFUSAL"), Some("gfx9 (Vega)"));
        assert_eq!(get("VKD3D_CONFIG"), Some("no_upload_hvv"));
        assert!(
            plan.advisories
                .iter()
                .any(|a| a.text.contains("mesh pipelines disabled"))
        );
    }

    #[test]
    fn an_8gb_plan_uses_the_low_vram_runtime_settings() {
        let plan = plan_for_ids(0x1002, 0x731f, 8192, Path::new("/tmp/bs"));
        let get = |k: &str| {
            plan.env
                .iter()
                .find(|(key, _)| key == k)
                .map(|(_, v)| v.as_str())
        };
        assert_eq!(get("VICTORY_TEXTURE_NO_WAIT"), Some("1"));
        assert_eq!(get("VICTORY_PIPELINE_WAIT_MS"), Some("50"));
        assert_eq!(get("VICTORY_SHADER_LOOP_LIMIT"), Some("1024"));
        assert_eq!(get("VICTORY_CARD_REHASH_RUNS"), Some("1"));
        assert_eq!(get("VICTORY_JOIN_COPIES"), Some("1"));

        let roomy = plan_for_ids(0x1002, 0x7550, 16384, Path::new("/tmp/bs"));
        assert!(!roomy.env.iter().any(|(k, _)| k.starts_with("VICTORY_")));
    }

    #[test]
    fn the_env_file_is_shell_safe() {
        let plan = plan_for_ids(0x10de, 0x2684, 8192, Path::new("/tmp/bs dir"));
        let text = plan.env_file();
        assert!(text.contains("export DXVK_STATE_CACHE=\"1\""));
        assert!(text.contains("/tmp/bs dir/dxvk-state"));
    }

    #[test]
    fn json_carries_the_policy_decision() {
        let plan = plan_for_ids(0x1002, 0x687f, 8192, Path::new("/tmp/bs"));
        let json = plan.to_json();
        assert!(json.contains("\"mesh_pipelines\":false"));
        assert!(json.contains("\"wave64_emulation\":false"));
        assert!(json.contains("\"texture_mb\":4915"));
    }
}
