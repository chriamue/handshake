#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod azns_router {
    use scale_info::prelude::string::String;
    use scale_info::prelude::vec::Vec;
    use openbrush::traits::AccountId;
    
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Caller is not allowed to call privileged calls.
        NotAdmin,
        /// Not a contract address
        InvalidRegistryAddress,
        /// Given TLD already points to a registry
        TldAlreadyInUse(String),
        /// Given Tld not found
        TldNotFound(String),
        /// Cannot find the resolved address
        CouldNotResolveDomain,
        /// Domain does not contain valid name and/or tld
        InvalidDomainName,
    }

    #[ink::trait_definition]
    pub trait AznsContract {
        #[ink(message, selector = 0xe6da7bf0)]
        fn get_all_registries(&self) -> Vec<AccountId>;
     
        #[ink(message, selector = 0x15a5d20a)]
        fn get_registry(&self, tld: String) -> Option<AccountId>;
     
        #[ink(message, selector = 0xd259f7ba)]
        fn get_address(&self, domain: String) -> Result<AccountId, Error>;
     
        #[ink(message, selector = 0xdf3a358e)]
        fn get_primary_domains(
            &self,
            account: AccountId,
            tld: Option<String>,
        ) -> Vec<(AccountId, String)>;
    }
}

#[openbrush::implementation(PSP22, PSP22Mintable)]
#[openbrush::contract]
pub mod handshake {
    use openbrush::traits::Storage;
    use openbrush::traits::String;

    use crate::azns_router::{
        AznsContract,
        Error as AznsRouterError,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Handshake {
        #[storage_field]
        psp22: psp22::Data,
        accounts: ink::prelude::vec::Vec<AccountId>,
        #[storage_field]
        handshakes: ink::prelude::vec::Vec<(AccountId, AccountId)>,
    }

    impl Handshake {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();

            psp22::Internal::_mint_to(&mut instance, Self::env().caller(), total_supply)
                .expect("Should mint");

            instance
        }

        #[ink(message)]
        pub fn signup(&mut self) -> Result<(), PSP22Error> {
            let caller = Self::env().caller();
            self.accounts.push(caller);
            Ok(())
        }

        #[ink(message)]
        pub fn num_accounts(&self) -> Result<u32, PSP22Error> {
            Ok(self.accounts.len().try_into().unwrap())
        }

        #[ink(message)]
        pub fn handshake(&mut self, other: AccountId) -> Result<(), PSP22Error> {
            let executive = Self::env().caller();
            self.handshakes.push((executive, other));
            Ok(())
        }

        #[ink(message)]
        pub fn num_handshakes(&self) -> Result<u32, PSP22Error> {
            Ok(self.handshakes.len().try_into().unwrap())
        }

        #[ink(message)]
        pub fn open_handshakes(&self) -> Result<Vec<AccountId>, PSP22Error> {
            let caller = Self::env().caller();
            let other_handshakes: Vec<AccountId> = self
                .handshakes
                .iter()
                .filter(|(_, other)| other == &caller)
                .map(|(executive, _)| *executive)
                .collect();
            Ok(other_handshakes)
        }

        #[ink(message)]
        pub fn get_primary_domains(
            &self,
            router_addr: AccountId,
            account: AccountId,
            tld: Option<String>,
        ) -> Vec<(AccountId, String)> {
            let router: ink::contract_ref!(AznsContract) = router_addr.into();
        
            router.get_primary_domains(account, tld)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[ink::test]
        fn total_supply_works() {
            let token = Handshake::new(100);
            assert_eq!(PSP22Impl::total_supply(&token), 100);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod e2e_tests {
        use super::*;
        use ink::primitives::AccountId;
        use ink_e2e::subxt::tx::Signer;
        use ink_e2e::subxt::utils::AccountId32;
        use ink_e2e::{build_message, Keypair, PolkadotConfig};
        use openbrush::contracts::psp22::psp22_external::PSP22;

        type ContractRef = HandshakeRef;

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        fn address_of(keypair: &Keypair) -> AccountId {
            let address: AccountId32 = <Keypair as Signer<PolkadotConfig>>::account_id(keypair);
            address.0.into()
        }

        async fn balance_of(
            client: &mut ink_e2e::Client<PolkadotConfig, ink_env::DefaultEnvironment>,
            address: ink::primitives::AccountId,
            account: ink::primitives::AccountId,
        ) -> Balance {
            let _msg = build_message::<ContractRef>(address.clone())
                .call(|contract| contract.balance_of(account));
            let result = client
                .call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                .await
                .return_value();
            result
        }

        #[ink_e2e::test]
        async fn assigns_initial_balance(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("handshake", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.balance_of(address_of(&ink_e2e::alice())));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            };
            assert!(matches!(result.return_value(), 100));
            Ok(())
        }

        #[ink_e2e::test]
        async fn transfer_adds_amount_to_destination_account(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("handshake", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of(&ink_e2e::bob()), 50, vec![]));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("transfer failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let balance_of_alice =
                balance_of(&mut client, address, address_of(&ink_e2e::alice())).await;

            let balance_of_bob =
                balance_of(&mut client, address, address_of(&ink_e2e::bob())).await;

            assert_eq!(balance_of_bob, 50, "Bob should have 50 tokens");
            assert_eq!(balance_of_alice, 50, "Alice should have 50 tokens");

            Ok(())
        }

        #[ink_e2e::test]
        async fn account_signup(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("handshake", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.signup());
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("signup failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let num_accounts = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.num_accounts());
                client
                    .call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .return_value()
            };

            assert_eq!(num_accounts.unwrap(), 1, "Should have 1 account");

            Ok(())
        }

        #[ink_e2e::test]
        async fn count_handshakes(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("handshake", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.handshake(address_of(&ink_e2e::bob())));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("handshake failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let num_accounts = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.num_handshakes());
                client
                    .call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .return_value()
            };

            assert_eq!(num_accounts.unwrap(), 1, "Should have 1 account");

            Ok(())
        }

        #[ink_e2e::test]
        async fn cannot_transfer_above_the_amount(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("handshake", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of(&ink_e2e::bob()), 101, vec![]));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            };

            assert!(matches!(
                result.return_value(),
                Err(PSP22Error::InsufficientBalance)
            ));

            Ok(())
        }
    }
}
