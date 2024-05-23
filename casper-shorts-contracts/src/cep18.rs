#[allow(unused_imports)]
use odra_modules;

pub use odra_modules::cep18::utils::Cep18Modality;
pub use odra_modules::cep18_token::*;

#[cfg(test)]
mod tests {
    use odra::{casper_types::U256, host::Deployer};
    use odra_modules::{
        cep18::utils::Cep18Modality,
        cep18_token::{Cep18HostRef, Cep18InitArgs},
    };

    #[test]
    fn works() {
        let env = odra_test::env();
        let init_args = Cep18InitArgs {
            name: "Cep18".to_string(),
            symbol: "MT".to_string(),
            decimals: 10,
            initial_supply: U256::from(1_000_000_000_000u64),
            minter_list: vec![],
            admin_list: vec![],
            modality: Some(Cep18Modality::MintAndBurn),
        };
        assert!(Cep18HostRef::try_deploy(&env, init_args).is_ok());
    }
}
