use crate::timeframe::*;
use serde;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum Stimulus {
    /// Perceive the passafe of time(frame).
    Time(TimeFrame),
    /// Perceive a guided action 
    Interface(/*TODO*/)
}