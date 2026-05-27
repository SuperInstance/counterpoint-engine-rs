//! Species counterpoint rules.

use crate::interval::{is_consonant, OCTAVE};

pub fn no_parallel_fifths(a:&[i32],b:&[i32])->bool{no_parallel(a,b,7)}
pub fn no_parallel_octaves(a:&[i32],b:&[i32])->bool{no_parallel(a,b,0)}

pub fn proper_resolution(v:&[i32],tonic:i32)->bool{
    if v.len()<2{return true}
    for i in 1..v.len(){let p=v[i-1]%OCTAVE;let c=v[i]%OCTAVE;if p==(tonic+11)%OCTAVE&&c!=tonic%OCTAVE{return false}}
    true
}

pub fn max_leap_seventh(v:&[i32])->bool{v.windows(2).all(|w|(w[1]-w[0]).abs()<=12)}
pub fn consonant_check(a:i32,b:i32)->bool{is_consonant((a-b).abs())}

pub fn voice_independence(a:&[i32],b:&[i32],t:f64)->bool{
    if a.len()<3||a.len()!=b.len(){return true}
    let s=(1..a.len()).filter(|&i|{let da=(a[i]-a[i-1]).signum();let db=(b[i]-b[i-1]).signum();da!=0&&da==db}).count();
    (s as f64/(a.len()-1)as f64)<=t
}

pub fn contrary_motion_score(a:&[i32],b:&[i32])->f64{
    if a.len()<2||a.len()!=b.len(){return 0.0}
    let mut c=0;let mut t=0;
    for i in 1..a.len(){let da=(a[i]-a[i-1]).signum();let db=(b[i]-b[i-1]).signum();if da!=0&&db!=0{t+=1;if da!=db{c+=1}}}
    if t==0{0.0}else{c as f64/t as f64}
}

pub fn voice_range_invariant(v:&[i32],lo:i32,hi:i32)->bool{v.iter().all(|&n|n>=lo&&n<=hi)}

fn no_parallel(a:&[i32],b:&[i32],iv:i32)->bool{
    if a.len()<2||a.len()!=b.len(){return true}
    for i in 1..a.len(){
        let pi=(a[i-1]-b[i-1]).abs()%OCTAVE;let ci=(a[i]-b[i]).abs()%OCTAVE;
        if pi==iv&&ci==iv{let da=(a[i]-a[i-1]).signum();let db=(b[i]-b[i-1]).signum();if da!=0&&da==db{return false}}
    }
    true
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]fn t_par5_ok(){assert!(no_parallel_fifths(&[60,62],&[64,65]));}
    #[test]fn t_par5_bad(){assert!(!no_parallel_fifths(&[60,62],&[67,69]));}
    #[test]fn t_leap(){assert!(max_leap_seventh(&[60,67,60]));assert!(!max_leap_seventh(&[60,73]));}
    #[test]fn t_range(){assert!(voice_range_invariant(&[60,62],55,75));assert!(!voice_range_invariant(&[60,80],55,75));}
    #[test]fn t_contrary(){assert!(contrary_motion_score(&[60,62,60],&[67,65,67])>0.5);}
}
