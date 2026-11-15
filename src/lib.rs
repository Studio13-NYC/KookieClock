//! Portable clock behavior and sound generation shared by native frontends.
//!
//! The shipping desktop frontend currently targets Windows. This library has
//! no windowing or playback dependencies and can be tested on macOS and Linux.
pub mod audio;
pub mod cast;
pub mod model;
