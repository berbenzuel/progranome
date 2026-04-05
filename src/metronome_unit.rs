use crate::MetronomeUnit;

// #[derive(Clone)]
// pub struct MetronomeUnit {
//     numerator: u8,//that's what is above in signature
//     denominator: u8,//that's what is below in signature
//     tempo: u16,
//     active: bool,
// }

impl MetronomeUnit {
    pub fn new(numerator: i32, denominator: i32, tempo: i32, count: i32) -> Self {
        Self {
            numerator,
            denominator,
            tempo,
            count,
            active: false
        }
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn get_numerator(&self) -> i32 {
        self.numerator
    }
    pub fn get_denominator(&self) -> i32 {
        self.denominator
    }

    pub fn get_tempo(&self) -> i32 {
        self.tempo
    }
}
