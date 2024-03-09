use crate::{interface::*, identity::*};
use super::*;

pub struct UniverseAction;
impl UniverseAction {
    pub fn authenticate(universe: &mut Universe, interface_uid: UID, auth: Auth) -> Result<Modification<UniverseBuilder>> {
        match auth {
            Auth::Password(encrypted_password) => {
                const TODO_PASSWORD: &str = "sh4d0wf4x"; //todo: fetch this from an auth db
                if encrypted_password != TODO_PASSWORD {
                    return Err(Error::AuthenticationFailed);
                }
            }
        }

        let mut universe_editor = universe.edit_self();
        universe_editor.add_active_interface_uid(interface_uid)?;
        let modification = universe_editor.modify(universe)?;

        Ok(modification)
    }
}

#[cfg(test)]
mod tests {
    use crate::testing;
    use super::*;

    #[test]
    fn test_authenticate() {
        let (mut universe, _world, interface) = testing::create_universe();
        let auth = Auth::Password("sh4d0wf4x".to_string());

        UniverseAction::authenticate(&mut universe, interface.uid(), auth).unwrap();
        assert_eq!(universe.active_interface_uids().last().unwrap(), &interface.uid());
    }
}