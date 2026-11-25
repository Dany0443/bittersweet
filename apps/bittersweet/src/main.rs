//! `bittersweet` — host checker and tuning planner.
//!
//! No subcommand starts Wine, Proton or a game binary. `doctor` reads
//! sysfs/procfs; `plan` writes an environment file; the rest are pure
//! policy checks.

use bittersweet_host as host;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const HELP: &str = "\
bittersweet — Linux/Proton host checker

USAGE:
    bittersweet doctor [--json]
    bittersweet plan   [--json] [--write DIR] [--no-proton]
                       [--vendor amd|nvidia|intel] [--device 0xID] [--vram MB]
    bittersweet check  [--warm] [--cache]
    bittersweet pipeline [--cap N] KEY...
    bittersweet help

COMMANDS:
    doctor     Probe the host (read-only) and print capabilities + advice.
    plan       Print the environment plan, or write it to DIR/bittersweet.env.
    check      Validate shader warm-up vs pipeline pre-build mode selection.
    pipeline   Apply the early-pipeline key cap and report what was dropped.

Nothing here opens, copies or runs a game executable.
";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        None | Some("help") | Some("-h") | Some("--help") => {
            print!("{HELP}");
            ExitCode::SUCCESS
        }
        Some("doctor") => doctor(&args[1..]),
        Some("plan") => plan(&args[1..]),
        Some("check") => check(&args[1..]),
        Some("pipeline") => pipeline(&args[1..]),
        Some(other) => {
            eprintln!("bittersweet: unknown command `{other}`");
            eprint!("{HELP}");
            ExitCode::from(2)
        }
    }
}

fn flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn value<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .map(String::as_str)
}

fn cache_root() -> PathBuf {
    host::paths::cache_root().unwrap_or_else(|_| PathBuf::from("./bittersweet-cache"))
}

fn doctor(args: &[String]) -> ExitCode {
    let report = host::probe::probe();
    let plan = host::plan::build(&report, &cache_root(), true);
    if flag(args, "--json") {
        println!("{}", plan.to_json());
        return ExitCode::SUCCESS;
    }
    let (text, critical) = host::doctor(&report, &cache_root());
    print!("{text}");
    if critical {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}

fn overridden_report(args: &[String]) -> host::probe::HostReport {
    let mut report = host::probe::probe();
    if let Some(vendor) = value(args, "--vendor") {
        let (vendor, default_device) = match vendor {
            "amd" | "radeon" => (host::gpu::Vendor::Amd, 0x7550),
            "nvidia" => (host::gpu::Vendor::Nvidia, 0x2684),
            "intel" => (host::gpu::Vendor::Intel, 0xe20b),
            other => {
                eprintln!("bittersweet: unknown --vendor `{other}`");
                std::process::exit(2);
            }
        };
        report.vendor = vendor;
        report.device = default_device;
    }
    if let Some(device) = value(args, "--device") {
        report.device = u16::from_str_radix(
            device.trim_start_matches("0x").trim_start_matches("0X"),
            16,
        )
        .unwrap_or(report.device);
    }
    if let Some(vram) = value(args, "--vram") {
        report.vram_mb = vram.parse().unwrap_or(report.vram_mb);
    }
    report.amd_family = host::gpu::amd_family(report.device);
    report
}

fn plan(args: &[String]) -> ExitCode {
    let report = overridden_report(args);
    let proton = !flag(args, "--no-proton");
    let plan = host::plan::build(&report, &cache_root(), proton);
    if flag(args, "--json") {
        println!("{}", plan.to_json());
        return ExitCode::SUCCESS;
    }
    if let Some(dir) = value(args, "--write") {
        let dir = Path::new(dir);
        if let Err(err) = std::fs::create_dir_all(dir) {
            eprintln!("bittersweet: cannot create {}: {err}", dir.display());
            return ExitCode::from(1);
        }
        let path = dir.join("bittersweet.env");
        if let Err(err) = std::fs::write(&path, plan.env_file()) {
            eprintln!("bittersweet: cannot write {}: {err}", path.display());
            return ExitCode::from(1);
        }
        println!("wrote {}", path.display());
        return ExitCode::SUCCESS;
    }
    print!("{}", plan.env_file());
    ExitCode::SUCCESS
}

fn check(args: &[String]) -> ExitCode {
    let warm = flag(args, "--warm");
    let cache = flag(args, "--cache");
    match host::warm::check_modes(warm, cache) {
        Ok(Some(mode)) => {
            println!("mode: {mode:?}");
            ExitCode::SUCCESS
        }
        Ok(None) => {
            println!("mode: none selected");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("bittersweet: {err}");
            ExitCode::from(1)
        }
    }
}

fn pipeline(args: &[String]) -> ExitCode {
    let cap = value(args, "--cap")
        .and_then(|v| v.parse().ok())
        .unwrap_or(host::pipeline::EARLY_MAX);
    let mut list = host::pipeline::EarlyList::new(cap);
    let mut keys = 0usize;
    let mut skipped_flags = 0usize;
    for arg in args {
        if arg == "--cap" {
            skipped_flags += 1;
            continue;
        }
        if skipped_flags > 0 {
            skipped_flags -= 1;
            continue;
        }
        if arg.starts_with("--") {
            continue;
        }
        keys += 1;
        list.note_used(arg);
    }
    let (listed, cap, dropped) = list.report();
    println!("keys:     {keys}");
    println!("listed:   {listed}");
    println!("cap:      {cap}");
    println!("dropped:  {dropped}");
    if list.was_truncated() {
        println!("truncated: yes (first keys kept; the tail is left to the title's own workers)");
    }
    ExitCode::SUCCESS
}
