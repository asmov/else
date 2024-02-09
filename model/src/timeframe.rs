use serde;

pub type Frame = u64;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct TimeFrame {
    frame: Frame,
    timestamp: u64,
}

impl TimeFrame {
    pub fn new(frame: Frame, timestamp: u64) -> Self {
        Self {
            frame,
            timestamp
        }
    }

    pub fn frame(&self) -> Frame {
        self.frame
    }
}