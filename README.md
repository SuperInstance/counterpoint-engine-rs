# counterpoint-engine

**Species counterpoint engine — first species rules, interval checking, voice leading.**

A no-std-friendly, zero-dependency Rust library for checking first-species counterpoint. Feed it a cantus firmus and a counterpoint voice, and it tells you whether the voice leading obeys the classical rules: no dissonances, no parallel fifths or octaves, no voice crossing, and no excessive leaps.

---

## What This Does

Implements the core rules of **first species counterpoint** (note-against-note) as defined in classical music theory:

1. **Consonance checking** — Every harmonic interval must be a perfect or imperfect consonance
2. **Parallel motion prohibition** — No parallel perfect fifths or octaves
3. **Leap limiting** — Counterpoint voice must not leap beyond a configurable maximum (default: 8 semitones)
4. **Voice crossing** — Counterpoint must not cross below the cantus firmus

Each violation is reported with its position and type. An overall score (0.0–1.0) reflects consonance ratio minus violation penalties.

---

## Key Idea

Counterpoint rules are **constraints on pairs of melodic lines**. This engine models those constraints as composable checks over a `VoicePair` (cantus firmus + counterpoint). Every interval between the two voices is classified, every motion between consecutive intervals is examined, and every leap in the counterpoint is bounded. The result is a `CheckResult` with a full violation list and a numerical score.

The design is intentionally minimal: no MIDI parsing, no playback, no GUI. Just the math of voice leading, expressed as pure functions on `Note` and `Interval`.

---

## Install

```toml
[dependencies]
counterpoint-engine = "0.1"
```

Or via git:

```toml
[dependencies]
counterpoint-engine = { git = "https://github.com/SuperInstance/counterpoint-engine-rs" }
```

Requires **Rust 2021 edition** (1.56+). **Zero runtime dependencies.**

---

## Quick Start

```rust
use counterpoint_engine::*;

// Define cantus firmus and counterpoint as MIDI note numbers
let pair = VoicePair::new(
    vec![60, 62, 64, 65, 67, 69, 71],  // C D E F G A B
    vec![64, 65, 67, 69, 71, 72, 74],  // E F G A B C D
);

// Check first species rules
let checker = CounterpointChecker::default();
let result = checker.check(&pair);

println!("Score: {:.2}", result.score);       // 0.0–1.0
println!("Valid: {}", result.is_valid());      // no violations?
for v in &result.violations {
    println!("Violation at index {:?}: {:?}", v, v);
}
```

---

## API Reference

### `Interval`

A musical interval measured in semitones.

```rust
let fifth = Interval::between(60, 67); // C to G
assert_eq!(fifth, Interval::PER5);
assert!(fifth.is_perfect_consonance());
assert_eq!(fifth.name(), "P5");
```

**Named constants:**

| Constant | Semitones | Name |
|----------|-----------|------|
| `UNISON` | 0 | P1 |
| `MIN2` | 1 | m2 |
| `MAJ2` | 2 | M2 |
| `MIN3` | 3 | m3 |
| `MAJ3` | 4 | M3 |
| `PER4` | 5 | P4 |
| `TRITONE` | 6 | TT |
| `PER5` | 7 | P5 |
| `MIN6` | 8 | m6 |
| `MAJ6` | 9 | M6 |
| `MIN7` | 10 | m7 |
| `MAJ7` | 11 | M7 |
| `OCTAVE` | 12 | P8 |

**Classification:**

| Method | Returns `true` for |
|--------|--------------------|
| `is_perfect_consonance()` | P1 (0), P5 (7), P8 (12) |
| `is_imperfect_consonance()` | m3 (3), M3 (4), m6 (8), M6 (9) |
| `is_consonance()` | Perfect + imperfect |
| `is_dissonance()` | Everything else (m2, M2, P4, TT, m7, M7) |

**Methods:**

| Method | Returns | Description |
|--------|---------|-------------|
| `between(a, b)` | `Interval` | Absolute interval between two MIDI pitches |
| `semitones()` | `i32` | Number of semitones |
| `name()` | `&'static str` | Interval quality name (P1, m3, TT, etc.) |

---

### `Note`

A note with a MIDI pitch.

```rust
let note = Note::new(69);  // A4
assert!((note.frequency() - 440.0).abs() < 0.01);
let interval = note.interval_to(&Note::new(60)); // A4 to C4 = M6
```

| Method | Returns | Description |
|--------|---------|-------------|
| `new(midi)` | `Note` | Create from MIDI number |
| `frequency()` | `f64` | Frequency in Hz (A4 = 440 Hz reference) |
| `interval_to(other)` | `Interval` | Interval to another note |

---

### `VoicePair`

A cantus firmus + counterpoint pair.

```rust
let pair = VoicePair::new(
    vec![60, 62, 64],  // CF: C, D, E
    vec![64, 67, 69],  // CP: E, G, A
);
```

| Method | Returns | Description |
|--------|---------|-------------|
| `intervals()` | `Vec<Interval>` | All harmonic intervals between voices |
| `consonance_count()` | `usize` | Number of consonant intervals |
| `dissonance_count()` | `usize` | Number of dissonant intervals |

---

### `Violation`

Rule violation types:

| Variant | Fields | Rule |
|---------|--------|------|
| `Dissonance` | `index`, `interval` | Non-consonant harmonic interval |
| `ParallelFifths` | `index` | Consecutive perfect fifths in same direction |
| `ParallelOctaves` | `index` | Consecutive unisons or octaves |
| `DirectFifth` | `index` | Similar motion into a perfect fifth |
| `DirectOctave` | `index` | Similar motion into an octave |
| `LargeLeap` | `index`, `leap` | CP leap exceeds `max_leap` |
| `VoiceCrossing` | `index` | CP pitch < CF pitch |

---

### `CheckResult`

Result of checking a voice pair:

| Field | Type | Description |
|-------|------|-------------|
| `violations` | `Vec<Violation>` | All rule violations found |
| `score` | `f64` | Quality score (0.0–1.0) |

| Method | Returns | Description |
|--------|---------|-------------|
| `is_valid()` | `bool` | `violations.is_empty()` |

**Score formula:**

```
score = max(0, consonance_ratio - 0.1 × violation_count)
```

Where `consonance_ratio = consonant_intervals / total_intervals`.

---

### `CounterpointChecker`

The main checker with configurable parameters:

```rust
let checker = CounterpointChecker {
    allow_imperfect: true,  // allow imperfect consonances
    max_leap: 8,            // max semitone leap in CP (default 8 = minor 6th)
};
let result = checker.check(&pair);
let score = checker.score(&pair); // shorthand
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `allow_imperfect` | `bool` | `true` | Whether imperfect consonances are allowed |
| `max_leap` | `i32` | `8` | Maximum CP leap in semitones |

---

## How It Works

### Checking Pipeline

```
VoicePair (CF + CP)
       │
       ▼
  ┌─────────────────────────────────┐
  │     1. Compute intervals         │
  │     CF[i] ↔ CP[i] for all i     │
  └──────────────┬──────────────────┘
                 │
       ▼         ▼         ▼         ▼
  ┌─────────┐ ┌────────┐ ┌───────┐ ┌──────────┐
  │Consonance│ │Parallel│ │ Leaps │ │  Voice    │
  │  Check   │ │Motion  │ │ Check │ │ Crossing  │
  │         │ │ Check   │ │       │ │  Check    │
  └────┬────┘ └───┬────┘ └───┬───┘ └─────┬────┘
       │          │          │            │
       └──────────┴──────────┴────────────┘
                  │
                  ▼
           CheckResult
       (violations + score)
```

### Rules Checked

1. **Dissonance**: Every interval must be a consonance (P1, m3, M3, P5, m6, M6, P8). P4 is treated as dissonant (classical convention).

2. **Parallel fifths**: Two consecutive intervals of P5 where both voices move in the same direction.

3. **Parallel octaves**: Two consecutive intervals of P1 or P8.

4. **Large leaps**: Any step in the CP voice larger than `max_leap` semitones.

5. **Voice crossing**: CP pitch lower than CF pitch at any position.

---

## The Math

### Frequency from MIDI

```
f = 440 × 2^((midi - 69) / 12)
```

Standard equal-temperament tuning with A4 (MIDI 69) = 440 Hz.

### Interval Classification

Intervals are classified by their absolute semitone count modulo octave equivalence:

| Semitones | Interval | Classification |
|-----------|----------|----------------|
| 0 | P1 (Unison) | Perfect consonance |
| 1 | m2 | Dissonance |
| 2 | M2 | Dissonance |
| 3 | m3 | Imperfect consonance |
| 4 | M3 | Imperfect consonance |
| 5 | P4 | Dissonance* |
| 6 | TT (Tritone) | Dissonance |
| 7 | P5 | Perfect consonance |
| 8 | m6 | Imperfect consonance |
| 9 | M6 | Imperfect consonance |
| 10 | m7 | Dissonance |
| 11 | M7 | Dissonance |
| 12 | P8 (Octave) | Perfect consonance |

*P4 is classified as dissonant in two-part first species counterpoint (it creates a "suspension" quality against the bass).

### Scoring

```
consonance_ratio = consonant_count / total_intervals
penalty = 0.1 × violation_count
score = max(0, consonance_ratio - penalty)
```

- Perfect compliance: score → 1.0
- All dissonant with violations: score → 0.0

---

## Testing

9 tests covering intervals, notes, consonant and dissonant counterpoint, parallel fifths, large leaps, voice crossing, and interval naming:

```bash
cargo test
```

---

## Zero Dependencies

This crate has **no runtime dependencies** — it uses only `std`. Suitable for embedded, WASM, or any environment where dependency count matters.

---

## License

MIT © SuperInstance
