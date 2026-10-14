use crate::QuasiRng;

/// Marker for sequences that are base-2 digital nets or sequences, such as Sobol and
/// van der Corput.
///
/// XOR-ing every point with the same word (a digital shift) keeps the `(t, m, s)`-net
/// property of such a sequence, so scrambling wrappers that XOR accept only this trait.
///
/// Implement it only if bit `k` (counting from the top) of each coordinate is the
/// `k`-th base-2 digit of the point. Nothing unsafe depends on it, but a wrong
/// implementation silently loses the equidistribution the wrappers promise.
pub trait DigitalNet<const D: usize>: QuasiRng<D> {}
