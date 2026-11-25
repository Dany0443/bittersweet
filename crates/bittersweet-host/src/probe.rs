//! Read-only Linux host probe.
//!
//! Reads sysfs, procfs and `/etc/os-release`. It never enumerates a game
//! directory, opens a title binary, or launches Wine/Proton. `probe_from`
//! takes a root so the same code can be tested against a fixture tree.

use crate::gpu::{AmdFamily, Vendor, amd_family};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HostReport {
    pub vendor: Vendor,
    pub device: u16,
    pub amd_family: AmdFamily,
    pub vram_mb: u64,
    pub gpu_name: Option<String>,
    pub max_map_count: Option<u64>,
    pub kernel: Option<String>,
    pub distro: Option<String>,
}

impl Default for HostReport {
    fn default() -> Self {
        Self {
            vendor: Vendor::Other(0),
            device: 0,
            amd_family: AmdFamily::Unknown,
            vram_mb: 0,
            gpu_name: None,
            max_map_count: None,
            kernel: None,
            distro: None,
        }
    }
}

fn read_trim(path: &Path) -> Option<String> {
    let text = std::fs::read_to_string(path).ok()?;
    let text = text.trim();
    if text.is_empty() {
        None
    } else {
        Some(text.to_owned())
    }
}

fn read_hex(path: &Path) -> Option<u16> {
    let text = read_trim(path)?;
    let text = text.trim_start_matches("0x").trim_start_matches("0X");
    u16::from_str_radix(text, 16).ok()
}

fn read_u64(path: &Path) -> Option<u64> {
    read_trim(path)?.parse().ok()
}

/// Prefer the card with the most reported VRAM, so an integrated GPU does
/// not hide a discrete adapter. Ties and unknown VRAM keep card-name order.
/// Connector entries (`card0-DP-1`) are skipped.
fn preferred_card(root: &Path) -> Option<PathBuf> {
    let drm = root.join("sys/class/drm");
    let mut cards: Vec<PathBuf> = std::fs::read_dir(&drm)
        .ok()?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| {
            let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
            name.strip_prefix("card")
                .is_some_and(|rest| !rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit()))
        })
        .collect();
    cards.sort();
    let mut best = None;
    let mut most_vram = 0;
    for card in cards.into_iter().filter(|p| p.join("device").is_dir()) {
        let vram = read_u64(&card.join("device/mem_info_vram_total")).unwrap_or(0);
        if best.is_none() || vram > most_vram {
            most_vram = vram;
            best = Some(card);
        }
    }
    best
}

fn parse_os_release(path: &Path) -> Option<String> {
    let text = std::fs::read_to_string(path).ok()?;
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("PRETTY_NAME=") {
            return Some(rest.trim().trim_matches('"').to_owned());
        }
    }
    None
}

/// Probe a root directory. Pass `/` for the live host.
pub fn probe_from(root: &Path) -> HostReport {
    let mut report = HostReport {
        max_map_count: read_u64(&root.join("proc/sys/vm/max_map_count")),
        kernel: read_trim(&root.join("proc/sys/kernel/osrelease")),
        distro: parse_os_release(&root.join("etc/os-release")),
        ..HostReport::default()
    };
    let Some(card) = preferred_card(root) else {
        return report;
    };
    let device_dir = card.join("device");
    if let Some(vendor) = read_hex(&device_dir.join("vendor")) {
        report.vendor = Vendor::from_pci(vendor);
    }
    if let Some(device) = read_hex(&device_dir.join("device")) {
        report.device = device;
    }
    report.amd_family = amd_family(report.device);
    report.vram_mb = read_u64(&device_dir.join("mem_info_vram_total"))
        .map(|bytes| bytes / (1024 * 1024))
        .unwrap_or(0);
    report.gpu_name = read_trim(&device_dir.join("product_name"))
        .or_else(|| read_trim(&device_dir.join("label")));
    report
}

pub fn probe() -> HostReport {
    probe_from(Path::new("/"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static N: AtomicU64 = AtomicU64::new(0);

    struct Fixture(PathBuf);

    impl Fixture {
        fn new() -> Self {
            let dir = std::env::temp_dir().join(format!(
                "bittersweet-probe-{}-{}",
                std::process::id(),
                N.fetch_add(1, Ordering::Relaxed)
            ));
            let _ = std::fs::remove_dir_all(&dir);
            std::fs::create_dir_all(&dir).unwrap();
            Fixture(dir)
        }

        fn write(&self, rel: &str, contents: &str) {
            let path = self.0.join(rel);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, contents).unwrap();
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn reads_an_amd_card_and_kernel_settings() {
        let f = Fixture::new();
        f.write("sys/class/drm/card0/device/vendor", "0x1002\n");
        f.write("sys/class/drm/card0/device/device", "0x7550\n");
        f.write(
            "sys/class/drm/card0/device/mem_info_vram_total",
            "17163091968\n",
        );
        f.write("sys/class/drm/card0/device/product_name", "Navi 48\n");
        f.write("sys/class/drm/card1-DP-1/status", "connected\n");
        f.write("proc/sys/vm/max_map_count", "1048576\n");
        f.write("proc/sys/kernel/osrelease", "6.12.1-test\n");
        f.write("etc/os-release", "NAME=Test\nPRETTY_NAME=\"Test Linux\"\n");

        let r = probe_from(&f.0);
        assert_eq!(r.vendor, Vendor::Amd);
        assert_eq!(r.device, 0x7550);
        assert_eq!(r.amd_family, AmdFamily::Rdna2Plus);
        assert_eq!(r.vram_mb, 16368);
        assert_eq!(r.gpu_name.as_deref(), Some("Navi 48"));
        assert_eq!(r.max_map_count, Some(1_048_576));
        assert_eq!(r.kernel.as_deref(), Some("6.12.1-test"));
        assert_eq!(r.distro.as_deref(), Some("Test Linux"));
    }

    #[test]
    fn prefers_the_larger_card_over_the_integrated_gpu() {
        let f = Fixture::new();
        f.write("sys/class/drm/card0/device/vendor", "0x1002");
        f.write("sys/class/drm/card0/device/device", "0x164e");
        f.write(
            "sys/class/drm/card0/device/mem_info_vram_total",
            "536870912",
        );
        f.write("sys/class/drm/card1/device/vendor", "0x1002");
        f.write("sys/class/drm/card1/device/device", "0x744c");
        f.write(
            "sys/class/drm/card1/device/mem_info_vram_total",
            "25769803776",
        );
        let r = probe_from(&f.0);
        assert_eq!(r.device, 0x744c);
        assert_eq!(r.vram_mb, 24576);
    }

    #[test]
    fn a_missing_card_does_not_panic() {
        let f = Fixture::new();
        let r = probe_from(&f.0);
        assert_eq!(r.vendor, Vendor::Other(0));
        assert_eq!(r.vram_mb, 0);
    }

    #[test]
    fn connector_entries_are_not_cards() {
        let f = Fixture::new();
        f.write("sys/class/drm/card0-DP-1/device/vendor", "0x1002\n");
        let r = probe_from(&f.0);
        assert_eq!(r.device, 0);
    }
}
