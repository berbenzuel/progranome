use std::rc::Rc;
use std::time::Duration;
use slint::Timer;
use crate::MetronomeUnit;

#[derive(Clone)]
pub struct TimerUnit {
    accent: bool,
    duration: Duration,
    metronome_unit_index: usize,
    metronome_unit_beat: usize

}



impl TimerUnit {
    pub fn new(duration: Duration, accent: bool, metronome_unit_index: usize, metronome_unit_beat: usize) -> Self {
        Self {
            accent,
            duration,
            metronome_unit_index,
            metronome_unit_beat
        }
    }

    pub fn duration(&self) -> Duration {self.duration}
    pub fn accent(&self) -> bool {self.accent}
    pub fn beat(&self) -> usize { self.metronome_unit_beat}
    pub fn index(&self) -> usize { self.metronome_unit_index}

}