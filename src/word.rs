use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitXor, BitXorAssign};

use urng_core::{Rng, Word};

mod sealed {
    pub trait Sealed {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
}

/// A [`Word`] with the operations quasi-random sequences and their wrappers need.
///
/// [`Word`] is sealed and has no arithmetic, so generic code over any
/// [`QuasiRng`](crate::QuasiRng), such as a scrambling wrapper, could not add or XOR two
/// words. This trait fills that gap.
///
/// It is sealed and implemented only for [`u32`] and [`u64`], so new items can be added
/// without a breaking change.
pub trait QmcWord:
    Word
    + sealed::Sealed
    + Copy
    + Eq
    + Ord
    + Hash
    + Debug
    + Default
    + Send
    + Sync
    + 'static
    + BitXor<Output = Self>
    + BitXorAssign
{
    /// The width in bits (32 or 64).
    const BITS: u32;

    /// Addition modulo `2^BITS`.
    #[must_use]
    fn wrapping_add(self, rhs: Self) -> Self;

    /// `n mod 2^BITS`.
    #[must_use]
    fn from_index(n: u64) -> Self;

    /// Zero-extends to [`u64`].
    #[must_use]
    fn to_u64(self) -> u64;

    /// Draws one word from a generator of either width.
    ///
    /// A 64-bit word from a 32-bit generator joins two draws (the first one is the high
    /// half). A 32-bit word from a 64-bit generator keeps the high half of one draw.
    ///
    /// ```
    /// use urng_core::Rng;
    /// use urng_qmc_core::QmcWord;
    ///
    /// struct Counter(u32);
    /// impl Rng for Counter {
    ///     type Word = u32;
    ///     fn nextu(&mut self) -> u32 {
    ///         self.0 += 1;
    ///         self.0
    ///     }
    /// }
    ///
    /// assert_eq!(u64::from_rng(&mut Counter(0)), 0x0000_0001_0000_0002);
    /// ```
    #[must_use]
    fn from_rng<R: Rng + ?Sized>(rng: &mut R) -> Self
    where
        R::Word: QmcWord;
}

impl QmcWord for u32 {
    const BITS: u32 = 32;

    #[inline]
    fn wrapping_add(self, rhs: Self) -> Self {
        u32::wrapping_add(self, rhs)
    }

    #[inline]
    fn from_index(n: u64) -> Self {
        n as u32
    }

    #[inline]
    fn to_u64(self) -> u64 {
        self as u64
    }

    #[inline]
    fn from_rng<R: Rng + ?Sized>(rng: &mut R) -> Self
    where
        R::Word: QmcWord,
    {
        (rng.nextu().to_u64() >> (<R::Word as QmcWord>::BITS - 32)) as u32
    }
}

impl QmcWord for u64 {
    const BITS: u32 = 64;

    #[inline]
    fn wrapping_add(self, rhs: Self) -> Self {
        u64::wrapping_add(self, rhs)
    }

    #[inline]
    fn from_index(n: u64) -> Self {
        n
    }

    #[inline]
    fn to_u64(self) -> u64 {
        self
    }

    #[inline]
    fn from_rng<R: Rng + ?Sized>(rng: &mut R) -> Self
    where
        R::Word: QmcWord,
    {
        let hi = rng.nextu().to_u64();
        if <R::Word as QmcWord>::BITS == 64 {
            hi
        } else {
            (hi << 32) | rng.nextu().to_u64()
        }
    }
}
