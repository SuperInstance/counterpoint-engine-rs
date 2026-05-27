//! Voice leading analysis.

use crate::interval::OCTAVE;

#[derive(Debug, Clone)]
pub struct VoiceLeadingScore {
    pub smoothness: i32, pub parallel_fifths: i32, pub parallel_octaves: i32,
    pub contrary_motion: i32, pub similar_motion: i32, pub oblique_motion: i32, pub total_intervals: i32,
}

impl VoiceLeadingScore {
    pub fn quality(&self) -> f64 {
        if self.total_intervals == 0 { return 1.0; }
        let mut s = 1.0;
        s -= (self.parallel_fifths as f64 * 0.2).min(0.5);
        s -= (self.parallel_octaves as f64 * 0.3).min(0.5);
        let m = (self.contrary_motion + self.similar_motion + self.oblique_motion).max(1);
        s += (self.contrary_motion as f64 / m as f64) * 0.2;
        s.clamp(0.0, 1.0)
    }
}

pub fn analyze_voice_leading(a: &[i32], b: &[i32]) -> VoiceLeadingScore {
    let mut s = VoiceLeadingScore { smoothness:0, parallel_fifths:0, parallel_octaves:0, contrary_motion:0, similar_motion:0, oblique_motion:0, total_intervals:0 };
    if a.len() != b.len() || a.len() < 2 { return s; }
    for i in 1..a.len() {
        let da = (a[i]-a[i-1]).signum(); let db = (b[i]-b[i-1]).signum();
        let ci = (a[i]-b[i]).abs()%OCTAVE; let pi = (a[i-1]-b[i-1]).abs()%OCTAVE;
        s.smoothness += (a[i]-a[i-1]).abs() + (b[i]-b[i-1]).abs();
        s.total_intervals += 1;
        if da != 0 && da == db {
            if pi==7 && ci==7 { s.parallel_fifths += 1; }
            if pi==0 && ci==0 { s.parallel_octaves += 1; }
            s.similar_motion += 1;
        } else if da==0 || db==0 { s.oblique_motion += 1; }
        else { s.contrary_motion += 1; }
    }
    s
}

pub fn voices_cross(upper: &[i32], lower: &[i32]) -> bool {
    upper.iter().zip(lower.iter()).any(|(&u, &l)| u < l)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn t_analysis() { let s = analyze_voice_leading(&[60,62,64,65], &[67,65,67,69]); assert!(s.total_intervals > 0); }
    #[test] fn t_no_cross() { assert!(!voices_cross(&[67,69], &[60,62])); }
    #[test] fn t_cross() { assert!(voices_cross(&[60,62], &[67,65])); }
}
