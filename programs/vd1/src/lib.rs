use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Token, Mint, MintTo,TokenAccount, mint_to},
};
use anchor_spl::associated_token::{AssociatedToken} ;

declare_id!("35TKZPztLWEgif2x6RAYKTR5Tkqa7euB19sYSqrC3M6c");

#[program]
pub mod vd1{
    use super::*;


    pub fn init_mint(_ctx: Context<InitMint>) -> Result<()> {
        msg!("Init Token");
        Ok(())
    }

    pub fn mint_token_x(ctx: Context<MintTokenX>, amount : u64) -> Result<()>{
        msg!("Init Mint x");
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint_account.to_account_info(),
            to: ctx.accounts.token_x.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        // Create the CpiContext we need for the request
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Execute anchor's helper function to mint tokens
        mint_to(cpi_ctx, amount)?;

        Ok(())
    }
    
}

    #[derive(Accounts)]
    pub struct InitMint<'info> {
        #[account(mut)]
        user : Signer<'info>,
        #[account(
            init,
            payer = user,
            mint::decimals = 18,
            mint::authority = user,
            mint::freeze_authority = user,
        )]
        x_mint: Account<'info, Mint>,
        system_program: Program<'info, System>,
        token_program: Program<'info, Token>,
    }



    #[derive(Accounts)]
    pub struct MintTokenX<'info> {
        #[account(mut)]
        user: Signer<'info>,
        #[account(mut)]
        mint_account: Account<'info, Mint>,
        #[account(
            init_if_needed,
            payer = user,
            associated_token::mint = mint_account,
            associated_token::authority = user,
        )]
        token_x: Account<'info, TokenAccount>,
        associated_token_program: Program<'info, AssociatedToken>,
        system_program: Program<'info, System>,
        token_program: Program<'info, Token>,
    }
