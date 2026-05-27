//! Species counterpoint engine — first species rules, interval checking, voice leading.

/// Musical interval in semitones.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval(pub i32);

impl Interval {
    pub const UNISON: Interval = Interval(0);
    pub const MIN2: Interval = Interval(1);
    pub const MAJ2: Interval = Interval(2);
    pub const MIN3: Interval = Interval(3);
    pub const MAJ3: Interval = Interval(4);
    pub const PER4: Interval = Interval(5);
    pub const TRITONE: Interval = Interval(6);
    pub const PER5: Interval = Interval(7);
    pub const MIN6: Interval = Interval(8);
    pub const MAJ6: Interval = Interval(9);
    pub const MIN7: Interval = Interval(10);
    pub const MAJ7: Interval = Interval(11);
    pub const OCTAVE: Interval = Interval(12);

    /// Absolute interval between two MIDI notes.
    pub fn between(a: i32, b: i32) -> Self {
        Interval((a - b).abs())
    }

    /// Number of semitones.
    pub fn semitones(&self) -> i32 {
        self.0
    }

    /// Is this a perfect consonance (P1, P5, P8)?
    pub fn is_perfect_consonance(&self) -> bool {
        matches!(self.0, 0 | 7 | 12)
    }

    /// Is this an imperfect consonance (m3, M3, m6, M6)?
    pub fn is_imperfect_consonance(&self) -> bool {
        matches!(self.0, 3 | 4 | 8 | 9)
    }

    /// Is this any consonance?
    pub fn is_consonance(&self) -> bool {
        self.is_perfect_consonance() || self.is_imperfect_consonance()
    }

    /// Is this a dissonance?
    pub fn is_dissonance(&self) -> bool {
        !self.is_consonance()
    }

    /// Interval quality name.
    pub fn name(&self) -> &'static str {
        match self.0 % 12 {
            0 => "P1",
            1 => "m2",
            2 => "M2",
            3 => "m3",
            4 => "M3",
            5 => "P4",
            6 => "TT",
            7 => "P5",
            8 => "m6",
            9 => "M6",
            10 => "m7",
            11 => "M7",
            _ => "?",
        }
    }
}

/// A note with MIDI pitch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Note {
    pub midi: i32,
}

impl Note {
    pub fn new(midi: i32) -> Self {
        Self { midi }
    }
    pub fn frequency(&self) -> f64 {
        440.0 * 2.0_f64.powf((self.midi - 69) as f64 / 12.0)
    }
    pub fn interval_to(&self, other: &Note) -> Interval {
        Interval::between(self.midi, other.midi)
    }
}

/// A pair of voices (cantus firmus + counterpoint).
#[derive(Debug, Clone)]
pub struct VoicePair {
    pub cf: Vec<Note>,
    pub cp: Vec<Note>,
}

impl VoicePair {
    pub fn new(cf: Vec<i32>, cp: Vec<i32>) -> Self {
        Self {
            cf: cf.into_iter().map(Note::new).collect(),
            cp: cp.into_iter().map(Note::new).collect(),
        }
    }

    /// Check all intervals between the voices.
    pub fn intervals(&self) -> Vec<Interval> {
        self.cf
            .iter()
            .zip(self.cp.iter())
            .map(|(c, p)| c.interval_to(p))
            .collect()
    }

    /// Count consonant intervals.
    pub fn consonance_count(&self) -> usize {
        self.intervals()
            .iter()
            .filter(|i| i.is_consonance())
            .count()
    }

    /// Count dissonant intervals.
    pub fn dissonance_count(&self) -> usize {
        self.intervals().len() - self.consonance_count()
    }
}

/// Types of counterpoint rule violations.
#[derive(Debug, Clone, PartialEq)]
pub enum Violation {
    Dissonance { index: usize, interval: Interval },
    ParallelFifths { index: usize },
    ParallelOctaves { index: usize },
    DirectFifth { index: usize },
    DirectOctave { index: usize },
    LargeLeap { index: usize, leap: i32 },
    VoiceCrossing { index: usize },
}

/// Result of checking counterpoint rules.
#[derive(Debug, Clone)]
pub struct CheckResult {
    pub violations: Vec<Violation>,
    pub score: f64,
}

impl CheckResult {
    pub fn is_valid(&self) -> bool {
        self.violations.is_empty()
    }
}

/// First species counterpoint checker.
#[derive(Debug, Clone)]
pub struct CounterpointChecker {
    pub allow_imperfect: bool,
    pub max_leap: i32,
}

impl Default for CounterpointChecker {
    fn default() -> Self {
        Self {
            allow_imperfect: true,
            max_leap: 8,
        }
    }
}

impl CounterpointChecker {
    /// Check first species counterpoint rules.
    pub fn check(&self, pair: &VoicePair) -> CheckResult {
        let mut violations = Vec::new();
        let intervals = pair.intervals();

        // Check consonances
        for (i, interval) in intervals.iter().enumerate() {
            if interval.is_dissonance() {
                violations.push(Violation::Dissonance {
                    index: i,
                    interval: *interval,
                });
            }
        }

        // Check parallel motion
        for i in 1..intervals.len() {
            let prev = intervals[i - 1];
            let curr = intervals[i];

            // Parallel 5ths
            if prev == Interval::PER5 && curr == Interval::PER5 {
                let prev_dir = pair.cp[i - 1].midi - pair.cp[i].midi;
                let curr_dir = pair.cf[i - 1].midi - pair.cf[i].midi;
                if prev_dir.signum() == curr_dir.signum() {
                    violations.push(Violation::ParallelFifths { index: i });
                }
            }

            // Parallel octaves
            if prev == Interval::UNISON && curr == Interval::UNISON {
                violations.push(Violation::ParallelOctaves { index: i });
            }
            if prev == Interval::OCTAVE && curr == Interval::OCTAVE {
                violations.push(Violation::ParallelOctaves { index: i });
            }
        }

        // Check leaps in counterpoint
        for i in 1..pair.cp.len() {
            let leap = (pair.cp[i].midi - pair.cp[i - 1].midi).abs();
            if leap > self.max_leap {
                violations.push(Violation::LargeLeap { index: i, leap });
            }
        }

        // Voice crossing (CP below CF in two-part)
        for i in 0..pair.cf.len() {
            if pair.cp[i].midi < pair.cf[i].midi {
                violations.push(Violation::VoiceCrossing { index: i });
            }
        }

        let n = intervals.len() as f64;
        let cons_ratio = pair.consonance_count() as f64 / n;
        let penalty = violations.len() as f64 * 0.1;
        let score = (cons_ratio - penalty).max(0.0);

        CheckResult { violations, score }
    }

    /// Score a voice pair (higher = better).
    pub fn score(&self, pair: &VoicePair) -> f64 {
        self.check(pair).score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval_consonance() {
        assert!(Interval::UNISON.is_perfect_consonance());
        assert!(Interval::PER5.is_perfect_consonance());
        assert!(Interval::OCTAVE.is_perfect_consonance());
        assert!(Interval::MIN3.is_imperfect_consonance());
        assert!(Interval::MAJ3.is_imperfect_consonance());
        assert!(Interval::TRITONE.is_dissonance());
        assert!(Interval::MIN2.is_dissonance());
    }

    #[test]
    fn test_interval_between() {
        assert_eq!(Interval::between(60, 67), Interval::PER5);
        assert_eq!(Interval::between(67, 60), Interval::PER5);
        assert_eq!(Interval::between(60, 60), Interval::UNISON);
    }

    #[test]
    fn test_note_frequency() {
        assert!((Note::new(69).frequency() - 440.0).abs() < 0.01);
        assert!((Note::new(60).frequency() - 261.63).abs() < 0.1);
    }

    #[test]
    fn test_consonant_counterpoint() {
        // C major: C-E, D-F, E-G, F-A, G-B, A-C, B-D (mostly consonant)
        let pair = VoicePair::new(
            vec![60, 62, 64, 65, 67, 69, 71], // C D E F G A B
            vec![64, 65, 67, 69, 71, 72, 74], // E F G A B C D
        );
        let checker = CounterpointChecker::default();
        let result = checker.check(&pair);
        assert!(result.score > 0.5);
    }

    #[test]
    fn test_dissonant_counterpoint() {
        // Tritones everywhere
        let pair = VoicePair::new(
            vec![60, 62, 64],
            vec![66, 68, 70], // tritone, tritone, tritone
        );
        let checker = CounterpointChecker::default();
        let result = checker.check(&pair);
        assert!(!result.violations.is_empty());
    }

    #[test]
    fn test_parallel_fifths() {
        // C-G followed by D-A: parallel 5ths
        let pair = VoicePair::new(
            vec![60, 62], // C, D
            vec![67, 69], // G, A (5th above each)
        );
        let checker = CounterpointChecker::default();
        let result = checker.check(&pair);
        assert!(result
            .violations
            .iter()
            .any(|v| matches!(v, Violation::ParallelFifths { .. })));
    }

    #[test]
    fn test_large_leap() {
        let pair = VoicePair::new(
            vec![60, 62],
            vec![60, 72], // octave leap
        );
        let checker = CounterpointChecker {
            max_leap: 7,
            ..Default::default()
        };
        let result = checker.check(&pair);
        assert!(result
            .violations
            .iter()
            .any(|v| matches!(v, Violation::LargeLeap { .. })));
    }

    #[test]
    fn test_voice_crossing() {
        let pair = VoicePair::new(
            vec![72, 72], // CF high
            vec![60, 60], // CP low (crossing)
        );
        let checker = CounterpointChecker::default();
        let result = checker.check(&pair);
        assert!(result
            .violations
            .iter()
            .any(|v| matches!(v, Violation::VoiceCrossing { .. })));
    }

    #[test]
    fn test_interval_names() {
        assert_eq!(Interval::UNISON.name(), "P1");
        assert_eq!(Interval::PER5.name(), "P5");
        assert_eq!(Interval::TRITONE.name(), "TT");
    }
}
