#![cfg_attr(feature = "nightly",
    feature(external_doc)
)]
#![cfg_attr(feature = "nightly",
    doc(include = "../README.md")
)]
#![cfg_attr(not(feature = "nightly"),
    doc = "See [crates.io](https://crates.io/crates/join-lazy-fmt)"
)]
#![cfg_attr(not(feature = "nightly"),
    doc = "for more info about this crate."
)]

pub use self::lazy_format::LazyFormat;
mod lazy_format;

pub use self::join::{Join, DisplayableJoin};
mod join;
