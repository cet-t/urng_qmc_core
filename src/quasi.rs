use urng_core::Word;

use crate::{Float, QmcWord};

/// A `D`-dimensional low-discrepancy sequence.
///
/// Each coordinate is a fixed-point number in `[0, 1)`: the word `x` stands for
/// `x / 2^BITS`. The high bits must carry the uniformity, because the conversions in
/// [`urng_core::Word`] read words from the top.
///
/// A point is returned whole, so the coordinates of one point cannot be mixed up with
/// those of the next by calling a scalar method twice.
///
/// ```
/// use urng_qmc_core::QuasiRng;
///
/// /// Base-2 van der Corput sequence.
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
/// let mut q = Vdc(0);
/// assert_eq!(q.next_point_f(), [0.0]);
/// assert_eq!(q.next_point_f(), [0.5]);
/// assert_eq!(q.next_point_f(), [0.25]);
/// ```
pub trait QuasiRng<const D: usize> {
    /// The word every coordinate is stored in.
    type Word: QmcWord;

    /// Returns the current point and advances the index by one.
    #[must_use]
    fn next_point(&mut self) -> [Self::Word; D];

    /// Writes the point [`next_point`](Self::next_point) would return into `out`.
    ///
    /// Sequences with a large `D` can override this to avoid moving the point through the
    /// stack.
    #[inline]
    fn fill_point(&mut self, out: &mut [Self::Word; D]) {
        *out = self.next_point();
    }

    /// Returns the next point with each coordinate converted to a float in `[0, 1)`.
    #[must_use]
    #[inline]
    fn next_point_f(&mut self) -> [Float<Self::Word>; D] {
        self.next_point().map(Word::to_float)
    }

    /// Returns the next point with coordinate `i` mapped to `[min[i], max[i])`.
    ///
    /// Like [`Word::randf`], this does not check `min[i] <= max[i]`.
    #[must_use]
    #[inline]
    fn next_point_in(
        &mut self,
        min: [Float<Self::Word>; D],
        max: [Float<Self::Word>; D],
    ) -> [Float<Self::Word>; D] {
        let p = self.next_point();
        core::array::from_fn(|i| p[i].randf(min[i], max[i]))
    }
}
