#![doc = include_str!("../crates-readme.md")]
#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod adapter;
mod net;
mod quasi;
mod seek;
mod word;

pub use crate::adapter::AsRng;
pub use crate::net::DigitalNet;
pub use crate::quasi::QuasiRng;
pub use crate::seek::Seek;
pub use crate::word::QmcWord;

/// The float type a word converts to: [`f32`] for [`u32`], [`f64`] for [`u64`].
pub type Float<W> = <W as urng_core::Word>::Float;
