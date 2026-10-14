use urng_core::Rng;
use urng_qmc_core::{AsRng, DigitalNet, QmcWord, QuasiRng, Seek};

/// Returns 1, 2, 3, ... as words of type `W`.
struct Counter<W>(W);

impl Rng for Counter<u32> {
    type Word = u32;
    fn nextu(&mut self) -> u32 {
        self.0 += 1;
        self.0
    }
}

impl Rng for Counter<u64> {
    type Word = u64;
    fn nextu(&mut self) -> u64 {
        self.0 += 1;
        self.0
    }
}

/// Base-2 van der Corput sequence with a 32-bit word.
#[derive(Debug, PartialEq)]
struct Vdc {
    index: u64,
}

impl QuasiRng<1> for Vdc {
    type Word = u32;
    fn next_point(&mut self) -> [u32; 1] {
        let x = (self.index as u32).reverse_bits();
        self.index = (self.index + 1) & 0xFFFF_FFFF;
        [x]
    }
}

impl Seek for Vdc {
    fn seek(&mut self, index: u64) {
        self.index = index & 0xFFFF_FFFF;
    }
    fn index(&self) -> u64 {
        self.index
    }
}

impl DigitalNet<1> for Vdc {}

/// Hammersley-like 2-D sequence: (n / 16, vdc(n)).
struct Pair<W>(W);

impl QuasiRng<2> for Pair<u32> {
    type Word = u32;
    fn next_point(&mut self) -> [u32; 2] {
        let n = self.0;
        self.0 = self.0.wrapping_add(1);
        [n << 28, n.reverse_bits()]
    }
}

impl QuasiRng<2> for Pair<u64> {
    type Word = u64;
    fn next_point(&mut self) -> [u64; 2] {
        let n = self.0;
        self.0 = self.0.wrapping_add(1);
        [n << 60, n.reverse_bits()]
    }
}

#[test]
fn word_constants_and_conversions() {
    assert_eq!(<u32 as QmcWord>::BITS, 32);
    assert_eq!(<u64 as QmcWord>::BITS, 64);
    assert_eq!(u32::from_index(0x1_2345_6789), 0x2345_6789);
    assert_eq!(u64::from_index(u64::MAX), u64::MAX);
    assert_eq!(QmcWord::wrapping_add(u32::MAX, 2u32), 1);
    assert_eq!(QmcWord::wrapping_add(u64::MAX, 2u64), 1);
    assert_eq!(u32::MAX.to_u64(), 0xFFFF_FFFF);
}

#[test]
fn from_rng_handles_both_widths() {
    // Same width: one draw, unchanged.
    assert_eq!(u32::from_rng(&mut Counter(0u32)), 1);
    assert_eq!(u64::from_rng(&mut Counter(0u64)), 1);
    // 64-bit word from a 32-bit generator: two draws, first one high.
    let mut r = Counter(0u32);
    assert_eq!(u64::from_rng(&mut r), 0x0000_0001_0000_0002);
    assert_eq!(r.0, 2);
    // 32-bit word from a 64-bit generator: high half of one draw.
    let mut r = Counter(0x1234_5678_0000_0000u64);
    assert_eq!(u32::from_rng(&mut r), 0x1234_5678);
    assert_eq!(r.0, 0x1234_5678_0000_0001);
}

#[test]
fn from_rng_works_through_dyn() {
    let mut r = Counter(0u64);
    let dynr: &mut dyn Rng<Word = u64> = &mut r;
    assert_eq!(u32::from_rng(dynr), 0);
}

#[test]
fn provided_methods() {
    let mut q = Vdc { index: 0 };
    assert_eq!(q.next_point_f(), [0.0]);
    assert_eq!(q.next_point_f(), [0.5]);
    assert_eq!(q.next_point_in([10.0], [14.0]), [11.0]);

    let mut out = [0u32; 1];
    q.fill_point(&mut out);
    assert_eq!(out, [0xC000_0000]);

    let mut p = Pair(0u32);
    let _ = p.next_point();
    let [x, y]: [f32; 2] = p.next_point_f();
    assert_eq!((x, y), (1.0 / 16.0, 0.5));
    let [a, b] = p.next_point_in([-1.0, 0.0], [1.0, 4.0]);
    assert_eq!((a, b), (-1.0 + 2.0 / 8.0, 1.0));
}

#[test]
fn provided_methods_u64() {
    let mut p = Pair(0u64);
    let _ = p.next_point();
    let [x, y]: [f64; 2] = p.next_point_f();
    assert_eq!((x, y), (1.0 / 16.0, 0.5));
    let [a, b] = p.next_point_in([-1.0, 0.0], [1.0, 4.0]);
    assert_eq!((a, b), (-1.0 + 2.0 / 8.0, 1.0));
}

#[test]
fn as_rng_delegates() {
    let mut rng = AsRng::new(Vdc { index: 0 });
    assert_eq!(rng.nextu(), 0);
    assert_eq!(rng.nextu(), 0x8000_0000);
    assert_eq!(rng.nextf(), 0.25);
    assert_eq!(rng.index(), 3);
    rng.seek(1);
    assert_eq!(rng.nextu(), 0x8000_0000);
    assert_eq!(rng.get_ref().index, 2);
    rng.get_mut().index = 0;
    assert_eq!(rng.into_inner(), Vdc { index: 0 });
}

fn uses_digital_net<Q: DigitalNet<D>, const D: usize>(q: &mut Q) -> [Q::Word; D] {
    q.next_point()
}

#[test]
fn digital_net_is_a_quasi_rng() {
    assert_eq!(uses_digital_net(&mut Vdc { index: 1 }), [0x8000_0000]);
}
