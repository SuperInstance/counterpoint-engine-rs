# counterpoint-engine-rs

Species counterpoint rule checker — interval classification, first-species rules (parallel 5ths/octaves, voice crossing, leap limits), and consonance scoring.

## What This Gives You

- **Interval classification** — Consonance, dissonance, perfect/imperfect intervals
- **First species rules** — Dissonance detection, parallel 5ths/octaves, voice crossing
- **Leap checking** — Configurable maximum leap size
- **Scoring** — Consonance ratio minus violation penalties

## Quick Start

```rust
use counterpoint_engine::{VoicePair, CounterpointChecker};

let pair = VoicePair::new(
    vec![60, 62, 64, 65, 67],  // C D E F G (cantus firmus)
    vec![64, 65, 67, 69, 71],  // E F G A B (counterpoint)
);

let checker = CounterpointChecker::default();
let result = checker.check(&pair);

if result.is_valid() {
    println!("Valid counterpoint! Score: {:.2}", result.score);
} else {
    for v in &result.violations {
        println!("Violation: {:?}", v);
    }
}
```

## API Reference

### `VoicePair`

```rust
VoicePair::new(cantus_firmus, counterpoint)  // Two pitch sequences (MIDI note numbers)
```

### `CounterpointChecker`

```rust
CounterpointChecker::default()               // Standard first-species rules
checker.check(&pair) -> CheckResult          // Evaluate all rules
```

### `CheckResult`

| Field | Description |
|-------|-------------|
| `is_valid()` | Passed all rules |
| `score` | Consonance ratio minus penalties |
| `violations` | Vec of specific rule violations |

## How It Fits

- **[counterpoint-engine-c](https://github.com/SuperInstance/counterpoint-engine-c)** — C99 port of this library
- **[constraint-instrument](https://github.com/SuperInstance/constraint-instrument)** — Counterpoint rules as constraint surfaces for interactive music
- **[flux-algebra-rs](https://github.com/SuperInstance/flux-algebra-rs)** — Harmonic ring and tuning systems for interval computation

## Testing

9 tests covering interval classification, consonance detection, parallel motion rules, voice crossing, and scoring.

```bash
cargo test
```

## Installation

```toml
[dependencies]
counterpoint-engine = { git = "https://github.com/SuperInstance/counterpoint-engine-rs" }
```

```bash
git clone https://github.com/SuperInstance/counterpoint-engine-rs.git
cd counterpoint-engine-rs
cargo build
```

## License

MIT

Part of the [SuperInstance OpenConstruct](https://github.com/SuperInstance) ecosystem.
