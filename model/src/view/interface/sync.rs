use super::*;
use crate::{error::*, modeling::*, sync::*};

impl SynchronizedDomainBuilder<InterfaceView> for InterfaceViewBuilder {
    fn synchronize(self, interface_view: &mut InterfaceView) -> Result<Modification<Self::BuilderType>> {
        self.modify(interface_view)
    }
}