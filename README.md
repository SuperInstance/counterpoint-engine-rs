# counterpoint-engine-rs

A species counterpoint engine providing interval arithmetic, voice leading, constraint-based composition, Laman graph rigidity, and holonomy-based harmony analysis — ported from the Python [counterpoint-engine](https://github.com/SuperInstance/counterpoint-engine) library.

## Modules

- **interval** — Pitch class math, consonance checks, semitone intervals
- **rules** — Species counterpoint constraints (no parallel fifths/octaves, proper resolution, max leap, voice independence)
- **voice_leading** — Voice leading analysis and quality scoring
- **generator** — Backtracking search counterpoint generation
- **laman** — Laman graph construction for constraint rigidity
- **tensor_output** — Tensor-MIDI event encoding
- **harmony** — Tonal graphs, holonomy cycle checking, Roman numeral analysis

## Usage

```rust
use counterpoint_engine::generator::{CounterpointGenerator, Species, VoiceRange, Scale};

let gen = CounterpointGenerator::new(
    Species::First,
    VoiceRange::new(48, 72),
    Scale::major(0),
);
let result = gen.generate(&[60, 62, 64, 65, 67]).unwrap();
assert!(result.success);
```

## Relation to Python Version

This is a pure Rust port of [SuperInstance/counterpoint-engine](https://github.com/SuperInstance/counterpoint-engine). The core algorithms have been translated to idiomatic Rust with no Python FFI — all types use `i32` for pitch classes and MIDI notes.

## License

MIT
