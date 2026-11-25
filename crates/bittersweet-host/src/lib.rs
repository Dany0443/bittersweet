//! Linux/Proton host policy for bittersweet.
//!
//! This crate is deliberately dependency-free (std only) so it can be
//! built and unit-tested on a Linux host without the Windows runtime. It
//! answers four questions:
//!
//! * what GPU is here, and which shader features may be used
//!   ([`gpu`], [`probe`]);
//! * how much VRAM may textures use ([`vram`]);
//! * which environment variables should the launcher set ([`plan`]);
//! * is the kernel configured for console memory-page aliasing
//!   ([`kernel`]).
//!
//! It never opens a game binary and never writes outside the paths it is
//! given.

pub mod env;
pub mod gpu;
pub mod kernel;
pub mod logging;
pub mod paths;
pub mod pipeline;
pub mod plan;
pub mod probe;
pub mod vram;
pub mod warm;

use probe::HostReport;

/// A human-readable doctor report: host, capability and advisories.
pub fn doctor(report: &HostReport, cache_root: &std::path::Path) -> (String, bool) {
    let plan = plan::build(report, cache_root, true);
    let mut out = String::new();
    out.push_str("bittersweet host check\n");
    out.push_str("-----------------------\n");
    out.push_str(&format!(
        "GPU:     {}\n",
        plan::capability_line(report)
    ));
    if let Some(name) = &report.gpu_name {
        out.push_str(&format!("Name:    {name}\n"));
    }
    out.push_str(&format!("VRAM:    {}\n", plan.budget.summary()));
    match &report.distro {
        Some(d) => out.push_str(&format!("Distro:  {d}\n")),
        None => out.push_str("Distro:  unknown\n"),
    }
    if let Some(k) = &report.kernel {
        out.push_str(&format!("Kernel:  {k}\n"));
    }
    out.push('\n');
    out.push_str(&plan.advisories_text());
    let critical = plan
        .advisories
        .iter()
        .any(|a| a.level == plan::Level::Critical);
    (out, critical)
}
