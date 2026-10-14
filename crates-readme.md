# urng_qmc_core

Traits for quasi-random (low-discrepancy) sequences on top of [`urng_core`](https://crates.io/crates/urng_core). `no_std`, no `alloc`, no `unsafe`.

This crate holds only the contracts. The sequences themselves (Weyl, R_d, van der Corput, Sobol, ...) live in [`urng_qmc`](https://crates.io/crates/urng_qmc), which re-exports everything here. Depend on this crate alone when you implement your own sequence.

| Item | Purpose |
| --- | --- |
| `QuasiRng<D>` | A `D`-dimensional sequence. `next_point` returns a whole point `[Word; D]` of fixed-point coordinates in `[0, 1)` |
| `Seek` | Jump to any index (period `2^B`) |
| `DigitalNet<D>` | Marker for base-2 digital nets, the sequences an XOR scramble keeps equidistributed |
| `QmcWord` | `urng_core::Word` plus the arithmetic generic wrappers need (`wrapping_add`, XOR, drawing from an `Rng` of either width) |
| `AsRng<Q>` | Uses a one-dimensional sequence as a `urng_core::Rng` |

## Usage

```rust
use urng_qmc_core::{QuasiRng, Seek};

/// Base-2 van der Corput sequence.
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

let mut q = Vdc { index: 0 };
let _: [f32; 1] = q.next_point_f();            // [0, 1)
let _: [f32; 1] = q.next_point_in([-1.0], [1.0]); // [-1, 1)
q.seek(1 << 20);
```

## Notes

- **Not suitable for cryptographic use.**
- **`AsRng` is for one value per sample only.** Successive values of a low-discrepancy sequence are strongly correlated, so code that draws several values per sample from one `Rng` breaks silently. Use a `D`-dimensional sequence instead.
- `urng_core` is a public dependency: its `Word` and `Rng` appear in this crate's API.
