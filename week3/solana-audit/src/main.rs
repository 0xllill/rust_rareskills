use anchor_lang::prelude::*;
use anchor_spl::token::{ self, Mint, Token, TokenAccount, Transfer};
use crate::errors::ErrorCode;
use crate::{stake_pool_signer_seeds, state::StakePool };  
  

#[assert_size(568)]
#[account(zero_copy)]
#[repr(C)]
pub struct StakePool { 
    /// The original creator of the StakePool. Necessary for signer seeds
    pub creator: Pubkey,
    /** Pubkey that can make updates to StakePool */
    pub authority: Pubkey,
    /** Pubkey that can lock any reward pool */
    pub locker: Pubkey,
    /** Total amount staked that accounts for the lock up period weighting.
    Note, this is not equal to the amount of SPL Tokens staked. */
    pub total_weighted_stake: u128,
    /** Token Account to store the staked SPL Token */
    pub vault: Pubkey,
    /** Mint of the token being staked */
    pub mint: Pubkey,
    /** Mint of the token representing effective stake */
    pub stake_mint: Pubkey,
    /// Array of RewardPools that apply to the stake pool.
    /// Unused entries are Pubkey default. In arbitrary order, and may have gaps.
    pub reward_pools: [RewardPool; MAX_REWARD_POOLS],
    /// The minimum weight received for staking. In terms of 1 / SCALE_FACTOR_BASE.
    /// Examples:
    /// * `min_weight = 1 x SCALE_FACTOR_BASE` = minmum of 1x multiplier for > min_duration staking
    /// * `min_weight = 2 x SCALE_FACTOR_BASE` = minmum of 2x multiplier for > min_duration staking
    pub base_weight: u64,
    /// Maximum weight for staking lockup (i.e. weight multiplier when locked
    /// up for max duration). In terms of 1 / SCALE_FACTOR_BASE. Examples:
    /// * A `max_weight = 1 x SCALE_FACTOR_BASE` = 1x multiplier for max staking duration
    /// * A `max_weight = 2 x SCALE_FACTOR_BASE` = 2x multiplier for max staking duration
    pub max_weight: u64,
    /** Minimum duration for lockup. At this point, the staker would receive the base weight. In seconds. */
    pub min_duration: u64,
    /** Maximum duration for lockup. At this point, the staker would receive the max weight. In seconds. */
    pub max_duration: u64,
    /** Nonce to derive multiple stake pools from same mint */
    pub nonce: u8,
    /** Bump seed for stake_mint */
    pub bump_seed: u8,
    // padding to next 8-byte
    _padding0: [u8; 6],
    _reserved0: [u8; 256]
}



#[derive(Accounts)] 
pub struct Slashing<'info> {
    // Payer to actually stake the mint tokens
    #[account(mut)]
    pub authority: Signer<'info>,  

    /// Vault of the StakePool token will be transfer to
    #[account(mut)] //E @audit maybe ensure that token_program = mint.owner
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub stake_mint: Account<'info, Mint>,

    //E wrapper type that provides safe access to zero-copy accounts (like StakePool)
    /// StakePool owning the vault that holds the deposit
    #[account(
        mut,
        has_one = vault @ ErrorCode::InvalidStakePoolVault, //E ensure vault is the stake_pool vault
        has_one = stake_mint @ ErrorCode::InvalidAuthority, //E ensure stake_mint is the stake_pool stake_mint
    )]
    pub stake_pool: AccountLoader<'info, StakePool>, //E @audit should ensure authority is the same as stake_pool authority
    
    pub token_program: Program<'info, Token>,
}
 
pub fn slashing_handler<'info>(
    ctx: Context<Slashing>,
    amount: u64,
    router: u8,
    is_locked: u8 
) -> Result<()> {
    {    
        //E load stake_pool account from accounts context
        let stake_pool = &mut ctx.accounts.stake_pool.load_mut()?;


        //E load the reward pool from the stake_pool @audit check that router is less than MAX_REWARD_POOLS
        let pool = &mut stake_pool.reward_pools[usize::from(router)];

        //E @audit no further checks that is_locked is locked or no?
        pool.is_locked = is_locked;

        let cpi_ctx = CpiContext {
            program: ctx.accounts.token_program.to_account_info(), //E @audit verify that the authority of the token_program is the stake_pool
            accounts: Transfer {
                from: ctx.accounts.vault.to_account_info(), //E @audit from and to are equal
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.stake_pool.to_account_info(), 
            },
            //E no additional accounts are being provided
            remaining_accounts: Vec::new(),
            //E generate the seeds needed for the stake pool PDA to sign the token transfer
                // something like [b"stake_pool", stake_pool.creator.as_ref(), &[stake_pool.bump_seed]]
            signer_seeds: &[stake_pool_signer_seeds!(stake_pool)],
        };

        //E transfer the tokens from the vault to the stake pool (@audit no check for return value ?)
        let _ = token::transfer(cpi_ctx, amount);

        Ok(())
    } 
}
