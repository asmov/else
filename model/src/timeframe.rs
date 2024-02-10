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

    pub fn tick(&mut self) {
        self.frame += 1;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap()
            .as_secs();


        self.timestamp = now;
    }

    pub fn frame(&self) -> Frame {
        self.frame
    }
}