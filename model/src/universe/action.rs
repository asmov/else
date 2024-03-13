use crate::{identity::*, interface::*, AuthRequestMsg};
use super::*;

pub struct UniverseAction;
impl UniverseAction {
    pub fn request_authentication(
        universe: &mut Universe,
        auth_request: AuthRequestMsg
    ) -> Result<Modification<UniverseBuilder>> {
        match auth_request {
            AuthRequestMsg::Solana(solana_auth_request) => {
                let mut universe_editor = universe.edit_self();
                universe_editor.add_auth_request(request)?;
                let modification = universe_editor.modify(universe)?;
                Ok(modification)
            }
        }
        //todo: make an actual lookup to an auth db or auth service
        let mut universe_editor = universe.edit_self();
        universe_editor.add_active_interface_uid(interface_uid)?;
        let modification = universe_editor.modify(universe)?;

        Ok(modification)
    }
}

#[cfg(test)]
mod tests {
    use crate::{testing, AuthRequestMsg};
    use super::*;

}