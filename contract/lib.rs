#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod azns_router {
    use openbrush::traits::AccountId;
    use scale_info::prelude::string::String;
    use scale_info::prelude::vec::Vec;

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

#[openbrush::implementation(PSP34)]
#[openbrush::contract]
pub mod handshake {
    use openbrush::traits::Storage;
    use openbrush::traits::String;

    use crate::azns_router::{AznsContract, Error as AznsRouterError};

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Handshake {
        #[storage_field]
        psp34: psp34::Data,
        next_id: u8,
        ticket_price: Balance,
        accounts: ink::prelude::vec::Vec<AccountId>,
        #[storage_field]
        handshakes: ink::prelude::vec::Vec<(AccountId, AccountId)>,
    }

    impl Handshake {
        #[ink(constructor)]
        pub fn new(ticket_price: Balance) -> Self {
            Self {
                ticket_price,
                ..Default::default()
            }
        }

        #[ink(message, payable)]
        pub fn signup(&mut self) -> Result<(), PSP34Error> {
            assert!(
                self.ticket_price <= Self::env().transferred_value(),
                "payment was not the ticket price"
            );
            let caller = Self::env().caller();
            psp34::Internal::_mint_to(self, caller, Id::U8(self.next_id))?;
            self.next_id += 1;
            self.accounts.push(caller);
            Ok(())
        }

        #[ink(message)]
        pub fn num_accounts(&self) -> Result<u32, PSP34Error> {
            Ok(self.accounts.len().try_into().unwrap())
        }

        #[ink(message)]
        pub fn handshake(&mut self, other: AccountId) -> Result<(), PSP34Error> {
            let executive = Self::env().caller();
            self.handshakes.push((executive, other));
            Ok(())
        }

        #[ink(message)]
        pub fn num_handshakes(&self) -> Result<u32, PSP34Error> {
            Ok(self.handshakes.len().try_into().unwrap())
        }

        #[ink(message)]
        pub fn open_handshakes(&self) -> Result<Vec<AccountId>, PSP34Error> {
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
        fn new_contract_works() {
            let contract = Handshake::new(7);
            assert_eq!(contract.num_accounts(), Ok(0));
            assert_eq!(contract.num_handshakes(), Ok(0));
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod e2e_tests {
        use super::*;
        use ink::primitives::AccountId;
        use ink_e2e::subxt::tx::Signer;
        use ink_e2e::subxt::utils::AccountId32;
        use ink_e2e::{build_message, Keypair, PolkadotConfig};

        type ContractRef = HandshakeRef;

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        fn address_of(keypair: &Keypair) -> AccountId {
            let address: AccountId32 = <Keypair as Signer<PolkadotConfig>>::account_id(keypair);
            address.0.into()
        }

        #[ink_e2e::test]
        async fn account_signup(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(7);
            let address = client
                .instantiate("handshake", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.signup());
                client
                    .call(&ink_e2e::alice(), _msg, 7, None)
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
            let constructor = ContractRef::new(7);
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
        async fn cannot_signup_without_payment(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(7);
            let address = client
                .instantiate("handshake", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.signup());
                client.call(&ink_e2e::alice(), _msg, 0, None).await
            };

            if let Err(ink_e2e::Error::CallDryRun(dry_run)) = result {
                let debug_message = String::from_utf8_lossy(&dry_run.debug_message);
                assert!(debug_message.contains("payment was not the ticket price"))
            } else {
                panic!("Signup without payment should fail")
            }

            Ok(())
        }
    }
}
