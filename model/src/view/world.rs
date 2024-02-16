use crate::{identity::*, timeframe::*, view::area::*};

pub struct WorldView {
    identity: Identity,
    frame: Frame,
    area: AreaView,
}

