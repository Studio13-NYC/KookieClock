//! Portable clock behavior and sound generation shared by native frontends.
//!
//! The library has no windowing or playback dependencies.
pub mod audio;
pub mod cast;
#[cfg(target_os = "macos")]
mod macos;
pub mod model;
