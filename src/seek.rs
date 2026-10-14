/// Jumping to any index of a sequence, for parallel use or for resuming.
///
/// A sequence of `B`-bit words has period `2^B`, and its index always stays in
/// `[0, 2^B)`.
pub trait Seek {
    /// Makes the next call to `next_point` (or `nextu`) return point number `index`.
    ///
    /// An `index` of `2^B` or more is reduced modulo `2^B`.
    fn seek(&mut self, index: u64);

    /// The index of the point the next call returns.
    #[must_use]
    fn index(&self) -> u64;
}
