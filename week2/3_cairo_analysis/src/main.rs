use starknet::ContractAddress;

/**
 * Simple ERC4626 vault contract
 * Use the erc20 interface ,and simple vault interface to implement the contract
 * constructor write the token to use for the vault 
 * Allow users to deposit and withdraw tokens from the vault
 * Allow users to get the balance of their deposits and the total supply of tokens minted in the vault
 */


//E write an interface for the contract to interact with the ERC20 token
#[starknet::interface]
pub trait IERC20<TContractState> {
    //E function to get name of token in felt252 format
    fn get_name(self: @TContractState) -> felt252;
    //E function to get symbol of token in felt252 format
    fn get_symbol(self: @TContractState) -> felt252;
    //E function to get decimals of token in u8 format
    fn get_decimals(self: @TContractState) -> u8;
    //E function to get total supply of token in felt252 format
    fn get_total_supply(self: @TContractState) -> felt252;
    //E function to get balance of an account in felt252 format
    fn balance_of(self: @TContractState, account: ContractAddress) -> felt252;
    //E function to get allowance of an owner for a spender in felt252 format
    fn allowance(
        self: @TContractState, owner: ContractAddress, spender: ContractAddress
    ) -> felt252;
    //E function to transfer a token to an account
    fn transfer(ref self: TContractState, recipient: ContractAddress, amount: felt252);
    //E function to transfer a token from an owner to a recipient
    fn transfer_from(
        ref self: TContractState,
        sender: ContractAddress,
        recipient: ContractAddress,
        amount: felt252
    );
    //E function to give allowance to a spender for a certain amount of tokens
    fn approve(ref self: TContractState, spender: ContractAddress, amount: felt252);
    //E function to increase allowance of a spender for a certain amount of tokens
    fn increase_allowance(ref self: TContractState, spender: ContractAddress, added_value: felt252);
    //E function to decrease allowance of a spender for a certain amount of tokens
    fn decrease_allowance(
        ref self: TContractState, spender: ContractAddress, subtracted_value: felt252
    );
}

//E interface for a vault
#[starknet::interface]
pub trait ISimpleVault<TContractState> {
    //E deposit function
    fn deposit(ref self: TContractState, amount: u256);
    //E withdraw function
    fn withdraw(ref self: TContractState, shares: u256);
    //E get balance deposited of a user in the vault
    fn user_balance_of(ref self: TContractState, account: ContractAddress) -> u256;
    //E fetch supply of tokens minted in the vault
    fn contract_total_supply(ref self: TContractState) -> u256;
}

//E simpleVault contract implementation
#[starknet::contract]
pub mod SimpleVault {
    use super::{IERC20Dispatcher, IERC20DispatcherTrait};

    //E use the starknet library to get the caller address and the contract address
    use starknet::{ContractAddress, get_caller_address, get_contract_address};

    //E create storage for the vault to store global datas
    #[storage]
    struct Storage {
        token: IERC20Dispatcher,
        total_supply: u256,
        balance_of: LegacyMap<ContractAddress, u256>
    }

    //E constructor of the contract
    #[constructor]
    fn constructor(ref self: ContractState, token: ContractAddress) {
        //E write in the struct storage the token address
        self.token.write(IERC20Dispatcher { contract_address: token });
    }

    //E create a trait for private functions for the token of the vault
    #[generate_trait]
    impl PrivateFunctions of PrivateFunctionsTrait {
        //E mint function
        fn _mint(ref self: ContractState, to: ContractAddress, shares: u256) {
            self.total_supply.write(self.total_supply.read() + shares);
            self.balance_of.write(to, self.balance_of.read(to) + shares);
        }

        //E burn function
        fn _burn(ref self: ContractState, from: ContractAddress, shares: u256) {
            self.total_supply.write(self.total_supply.read() - shares);
            self.balance_of.write(from, self.balance_of.read(from) - shares);
        }
        
    }

    //E implement the functions of the contract using ISimpleVault interface
    #[abi(embed_v0)]
    impl SimpleVault of super::ISimpleVault<ContractState> {
        
        //E function to get the balance of a user in the vault
        fn user_balance_of(ref self: ContractState, account: ContractAddress) -> u256 {
            //E read the balance of the user in the vault using erc20 interface
            self.balance_of.read(account)
        }

        //E function to get the total supply of tokens minted in the vault
        fn contract_total_supply(ref self: ContractState) -> u256 {
            //E read contract struct
            self.total_supply.read()
        }

        //E deposit function
        fn deposit(ref self: ContractState, amount: u256){
            //E get the caller address and the contract address using starknet lib
            let caller = get_caller_address();
            let this = get_contract_address();

            //E calculate the number of shares to mint
            let mut shares = 0;

            //E if supply is 0 => mint 1:1 
            if self.total_supply.read() == 0 {
                shares = amount;
            } else {
                //E else get the balance of tokens minted for the vault and compute how much to mint
                let balance: u256 = self.token.read().balance_of(this).try_into().unwrap();
                shares = (amount * self.total_supply.read()) / balance;
            }
            
            //E mint the shares to the caller
            PrivateFunctions::_mint(ref self, caller, shares);
           
            //E transfer the tokens from the caller to the contract
                // low = get the least significant 128 bits
            let amount_felt252: felt252 = amount.low.into();
            self.token.read().transfer_from(caller, this, amount_felt252);
        }

        //E withdraw function
        fn withdraw(ref self: ContractState, shares: u256) {
            //E get the caller address and the contract address using starknet lib
            let caller = get_caller_address();
            let this = get_contract_address();

            //E get the balance of the user in the vault
            let balance = self.user_balance_of(this);
            //E compute the amount to withdraw using totalSupply 
            let amount = (shares * balance) / self.total_supply.read();
            
            //E burn the shares from the caller
            PrivateFunctions::_burn(ref self, caller, shares);
            
            //E transfer the tokens from the contract to the caller
                // low = get the least significant 128 bits
            let amount_felt252: felt252 = amount.low.into();

            //E transfer amount to caller
            self.token.read().transfer(caller, amount_felt252);
        }
    }
}