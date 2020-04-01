//! The prelude file imports various `CollectDispatch*` traits.
//!
//! This allows one
//! to import everything just by writing `use resep::prelude::*;`, and get
//! access for everything needed.

pub use crate::{dispatchers::*, implement_dispatch, implement_dispatcher_trait};
