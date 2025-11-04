#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String};

/// Escrow Contract
///
/// This contract holds user guarantees (20% down payment) and releases funds to merchants
/// when conditions are met. It manages the escrow lifecycle for BNPL transactions.
#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    /// Returns the version of this contract
    pub fn version(_env: Env) -> String {
        String::from_str(&_env, "trustup-v1.0")
    }

    // TODO: Implement escrow deposit functionality
    // TODO: Implement escrow release to merchant
    // TODO: Implement refund functionality
    // TODO: Implement escrow status queries
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    #[allow(deprecated)]
    fn test_version() {
        let env = Env::default();
        let contract_id = env.register_contract(None, EscrowContract);
        let client = EscrowContractClient::new(&env, &contract_id);

        let version = client.version();
        assert_eq!(version, String::from_str(&env, "trustup-v1.0"));
    }

    // TODO: Add tests for escrow deposit
    // TODO: Add tests for escrow release
    // TODO: Add tests for refund
}
