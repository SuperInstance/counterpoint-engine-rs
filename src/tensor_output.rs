//! Tensor-MIDI event encoding.

use serde::{Deserialize,Serialize};

#[derive(Debug,Clone,Copy,PartialEq,Eq,Serialize,Deserialize)]
pub struct TensorMidiEvent { pub cos_int8:i8, pub sin_int8:i8, pub beat_k:u8, pub state_byte:u8 }

impl TensorMidiEvent {
    pub fn new(cos:i8,sin:i8,beat:u8,state:u8)->Self{
        Self{cos_int8:cos.clamp(-128,127),sin_int8:sin.clamp(-128,127),beat_k:beat.min(255),state_byte:state}
    }
    pub fn from_pitch_interval(pitch:i32,interval:i32,beat:i32,side:i32)->Self{
        let angle=(interval.rem_euclid(12)as f64)*std::f64::consts::PI/6.0;
        let cos=(angle.cos()*60.0)as i8; let sin=(angle.sin()*60.0)as i8;
        let state=(((side&0x0F)as u8)<<4)|(pitch.rem_euclid(12)as u8);
        Self::new(cos,sin,beat.rem_euclid(256)as u8,state)
    }
    pub fn to_bytes(&self)->[u8;4]{[self.cos_int8 as u8,self.sin_int8 as u8,self.beat_k,self.state_byte]}
    pub fn from_bytes(b:[u8;4])->Self{Self{cos_int8:b[0]as i8,sin_int8:b[1]as i8,beat_k:b[2],state_byte:b[3]}}
}

pub fn voice_leading_side_state(voice:&[i32],beat:usize)->i32{
    if beat==0{return 1}
    let prev=voice[beat-1];let curr=voice[beat];let leap=(curr-prev).rem_euclid(12);
    if prev.rem_euclid(12)==11&&curr.rem_euclid(12)==0{return 3}
    if leap<=2{0}else if [3,4,7,8,9].contains(&leap){1}else{2}
}

pub fn voices_to_tensor_events(voices:&[Vec<i32>])->Vec<TensorMidiEvent>{
    if voices.is_empty(){return vec![]}
    let n=voices[0].len();let mut events=Vec::new();
    for beat in 0..n{
        let bass=voices[0][beat];
        for(vi,v)in voices.iter().enumerate(){
            if beat>=v.len(){continue}
            let pitch=v[beat];let interval=if vi==0{0}else{(pitch-bass).rem_euclid(12)};
            let side=voice_leading_side_state(v,beat);
            events.push(TensorMidiEvent::from_pitch_interval(pitch,interval,beat as i32,side));
        }
    }
    events
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]fn t_roundtrip(){let e=TensorMidiEvent::new(42,-42,100,200);assert_eq!(e,TensorMidiEvent::from_bytes(e.to_bytes()));}
    #[test]fn t_voices(){assert_eq!(voices_to_tensor_events(&[vec![60,62],vec![67,65]]).len(),4);}
    #[test]fn t_side(){assert_eq!(voice_leading_side_state(&[60,62],0),1);}
}
