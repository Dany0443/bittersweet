//! Linux/Proton host policy for bittersweet.
//!
//! Dependency-free (std only) so the host half builds and tests on Linux
//! without the Windows runtime.

pub mod gpu;

pub mod vram;

pub mod paths;

pub mod logging;

pub mod pipeline;

pub mod warm;

pub mod kernel;

pub mod env;

pub mod probe;
