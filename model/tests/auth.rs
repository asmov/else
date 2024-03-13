use auth::*;

#[cfg(test)]
mod tests {
    use asmov_else_model::auth;

    #[test]
    fn test_authenticate() {
        let (mut universe, _world, interface) = testing::create_universe();
        let solana_auth = auth::SolanaAuthRequest {
            client_public_key: [0; 32],
            client_challenge: [0; 32]
        };

        let AuthRequestMsg = AuthRequestMsg::Solana();

        UniverseAction::authenticate(&mut universe, interface.uid(), auth).unwrap();
        assert_eq!(universe.active_interface_uids().last().unwrap(), &interface.uid());
    }
}
