//! The prelude file imports various `CollectDispatch*` traits.
//!
//! This allows one
//! to import everything just by writing `use edisp::prelude::*;`, and get
//! access for everything needed.

pub use edisp_core::prelude::*;

#[cfg(default_features)]
pub use edisp_derive::Dispatch;
