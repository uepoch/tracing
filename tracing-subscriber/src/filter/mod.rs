//! [Subscribers](crate::subscribe) that control which spans and events are
//! enabled by the wrapped collector.
//!
//! This module contains a number of types that provide implementations of
//! various strategies for filtering which spans and events are enabled. For
//! details on filtering spans and events using [`Subscribe`] implementations,
//! see the [`subscribe` module documentation].
//!
//! [`subscribe` module documentation]: crate::subscribe#filtering-with-subscribers
//! [`Subscribe`]: crate::subscribe
mod filter_fn;
mod level;

feature! {
    #![all(feature = "env-filter", feature = "std")]
    mod env;
    pub use self::env::*;
}

feature! {
    #![all(feature = "registry", feature = "std")]
    mod subscriber_filters;
    pub use self::subscriber_filters::*;
}

pub use self::filter_fn::*;
#[cfg(not(feature = "registry"))]
pub(crate) use self::has_plf_stubs::*;

pub use self::level::{LevelFilter, ParseError as LevelParseError};

#[cfg(not(all(feature = "registry", feature = "std")))]
pub(crate) use self::has_plf_stubs::*;

feature! {
    #![any(feature = "std", feature = "alloc")]
    pub mod targets;
    pub use self::targets::Targets;

    mod directive;
    pub use self::directive::ParseError;
}

/// Stub implementations of the per-layer-fitler detection functions for when the
/// `registry` feature is disabled.
#[cfg(not(all(feature = "registry", feature = "std")))]
mod has_plf_stubs {
    pub(crate) fn is_plf_downcast_marker(_: core::any::TypeId) -> bool {
        false
    }

    /// Does a type implementing `Collect` contain any per-layer filters?
    pub(crate) fn collector_has_plf<C>(_: &C) -> bool
    where
        C: tracing_core::Collect,
    {
        false
    }

    /// Does a type implementing `Layer` contain any per-layer filters?
    pub(crate) fn subscriber_has_plf<S, C>(_: &S) -> bool
    where
        S: crate::Subscribe<C>,
        C: tracing_core::Collect,
    {
        false
    }
}
