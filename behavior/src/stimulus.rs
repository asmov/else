use crate::timeframe::*;

pub enum Stimulus<'s> {
    /// Perceive the passafe of time(frame).
    Time(&'s TimeFrame),
    /// Perceive a guided action 
    Interface(/*TODO*/)
}