#![cfg_attr(not(feature = "std"), no_std)]
#[derive(Debug, Copy, Clone)]
use ink_lang as ink;
use std::io;

#[ink::contract]
 mod Errc20 {

    use ink_storage::collections::HashMap;
    use ink_env::bal{}ance::Balance;
    use ink_prelude::string::String;
    use ink_constructor::construct;
    use ink_message::Message;
    use ink_test::Test;

    #[ink(storage)]
    pub struct Erc20 {
        total_supply: Balance,
        balances: HashMap<AccountId, Balance>,
        allowances: HashMap<(AccountId, AccountId), Balance>,
        name: String,
        symbol: String,
        decimals: u8,
    }

    #[ink(event)]
    pub struct Transfer {
        from: AccountId,
        to: AccountId,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        owner: AccountId,
        spender: AccountId,
        value: Balance,
    }

    pub fn new(initial_supply: Balance, name: String, symbol: String, decimals: u8) -> Self {
        let caller = self.env().caller();
        let mut balances = HashMap::new();
        balances.insert(caller, initial_supply);

        Erc20 {
            total_supply: initial_supply,
            balances,
            allowances: HashMap::new(),
            name,
            symbol,
            decimals,
        }
    }
    #[ink::test]
    pub fn test(&mut  self, initial_supply: Balance, ) {
        initial_supply += 40;
        initial_supply.transfer_from_to(20);
        assert_eq(initial_supply.get_balance(),  20);


    }
    

   pub  fn transfer_from_to(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<(), String> {
        let from_balance = self.balances.get_mut(&from).ok_or("Invalid 'from' account")?;
        let to_balance = self.balances.get_mut(&to).unwrap_or_else(|| {
        self.balances.insert(to.clone(), 0);
        0
    });

    if *from_balance < value {
        return Err("Insufficient balance".to_string());
    }

    *from_balance -= value;
    *to_balance += value;

    self.env().emit_event(Transfer {
        from,
        to,
        value,
    });
    Ok(())
    }

    pub fn get_balance(&self ) -> Result<Self, Self::Error>  {
        Balance.unwrap().get_balance();
        Ok(())
    }
    
    pub  fn retire(&mut self, total_supply: Balance, other_mane: String,  valeur : Balance ) -> Result<Self, Self::Error> {
    total_supply -= valeur;
    println!("{:#?}", total_supply);
   }

    pub  fn ajoute(&mut self, valeur : Balance, total_supply: Balance, other_mane: String, ) -> Result< Self, Self::Error> {
    total_supply += valeur;
    println!("{:#?}", total_supply); 
   }

}

fn  main() {

}