//! The prelude file imports various `CollectDispatch*` traits.
//!
//! This allows one
//! to import everything just by writing `use edisp::prelude::*;`, and get
//! access for everything needed.

pub use edisp_core::prelude::*;

#[cfg(feature = "default")]
pub use edisp_derive::Dispatch;
