use urng_core::Rng;

use crate::{QuasiRng, Seek};

/// Uses a one-dimensional quasi-random sequence where a [`Rng`] is expected.
///
/// # Only for one value per sample
///
/// Successive values of a low-discrepancy sequence are strongly correlated. Code that
/// draws several values per sample (a 2-D point, Box–Muller, rejection sampling, a
/// shuffle) breaks silently: estimating π from pairs of successive van der Corput values
/// gives about 3.29. Wrap a sequence only when the consumer draws exactly one value per
/// sample, and use a `D`-dimensional [`QuasiRng`] otherwise.
///
/// ```
/// use urng_core::Rng;
/// use urng_qmc_core::{AsRng, QuasiRng};
///
/// struct Vdc(u32);
///
/// impl QuasiRng<1> for Vdc {
///     type Word = u32;
///
///     fn next_point(&mut self) -> [u32; 1] {
///         let x = self.0.reverse_bits();
///         self.0 = self.0.wrapping_add(1);
///         [x]
///     }
/// }
///
/// let mut rng = AsRng::new(Vdc(0));
/// assert_eq!(rng.nextf(), 0.0);
/// assert_eq!(rng.nextf(), 0.5);
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AsRng<Q> {
    inner: Q,
}

impl<Q> AsRng<Q> {
    /// Wraps `inner`.
    #[inline]
    pub const fn new(inner: Q) -> Self {
        Self { inner }
    }

    /// Returns the wrapped sequence.
    #[inline]
    pub fn into_inner(self) -> Q {
        self.inner
    }

    /// Returns a reference to the wrapped sequence.
    #[inline]
    pub const fn get_ref(&self) -> &Q {
        &self.inner
    }

    /// Returns a mutable reference to the wrapped sequence.
    #[inline]
    pub fn get_mut(&mut self) -> &mut Q {
        &mut self.inner
    }
}

impl<Q: QuasiRng<1>> Rng for AsRng<Q> {
    type Word = Q::Word;

    #[inline]
    fn nextu(&mut self) -> Q::Word {
        let [x] = self.inner.next_point();
        x
    }
}

impl<Q: Seek> Seek for AsRng<Q> {
    #[inline]
    fn seek(&mut self, index: u64) {
        self.inner.seek(index);
    }

    #[inline]
    fn index(&self) -> u64 {
        self.inner.index()
    }
}
