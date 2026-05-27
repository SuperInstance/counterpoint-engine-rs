//! Interval arithmetic and pitch class math.

pub const OCTAVE: i32 = 12;
pub const CONSONANT_INTERVALS: &[i32] = &[0, 3, 4, 5, 7, 8, 9, 12];
pub const PERFECT_INTERVALS: &[i32] = &[0, 5, 7, 12];
pub const PITCH_NAMES: [&str; 12] = ["C","C#","D","D#","E","F","F#","G","G#","A","A#","B"];

pub fn is_consonant(s: i32) -> bool { CONSONANT_INTERVALS.contains(&(s.abs() % OCTAVE)) }
pub fn is_perfect(s: i32) -> bool { PERFECT_INTERVALS.contains(&(s.abs() % OCTAVE)) }
pub fn pitch_name(n: i32) -> &'static str { PITCH_NAMES[(n.rem_euclid(OCTAVE)) as usize] }
pub fn pitch_class(n: i32) -> i32 { n.rem_euclid(OCTAVE) }
pub fn semitone_interval(from: i32, to: i32) -> i32 { let d=(to-from).rem_euclid(OCTAVE); if d>6{d-OCTAVE}else{d} }
pub fn consonant_interval(a:i32,b:i32)->bool{is_consonant((a-b).abs())}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn t_consonance(){assert!(is_consonant(0));assert!(is_consonant(7));assert!(!is_consonant(6));}
    #[test] fn t_pc(){assert_eq!(pitch_class(60),0);assert_eq!(pitch_class(64),4);}
    #[test] fn t_name(){assert_eq!(pitch_name(60),"C");}
    #[test] fn t_interval(){assert_eq!(semitone_interval(0,4),4);assert_eq!(semitone_interval(0,8),-4);}
}
