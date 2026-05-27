# counterpoint-engine-rs

Rust port of [counterpoint-engine](https://github.com/SuperInstance/counterpoint-engine) — species counterpoint rule checker.

## Features

- **Interval classification**: consonance, dissonance, perfect/imperfect
- **First species rules**: dissonance detection, parallel 5ths/octaves, voice crossing
- **Leap checking**: configurable maximum leap size
- **Scoring**: consonance ratio minus violation penalties

## Usage

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

## License

MIT
