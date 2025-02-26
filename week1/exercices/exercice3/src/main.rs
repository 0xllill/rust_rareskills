fn main() {
    println!("Hello, world!");
}


pub fn deposit(ctx: Context<Deposit>, collat: u64) -> Result<()> {

    let rate = exchange_rate.deposit_rate as u128;
    
    //E integer overflow is possible because 2 u128 integers multiplied together
        // => remediation .checked_mul(rate)
    let amt = (collat as u128 * rate / DECIMALS_SCALAR) as u64; 

    //E convert to u64 : u64::try_from(amt)

    // transfer(token, from, to, amount)
    token::transfer(collateral_token, ctx.caller, ctx.this, collat)?;

    // mint_to(token, to, amount)
    token::mint_to(shares_token, ctx.caller, amt)?;

    Ok(())
}