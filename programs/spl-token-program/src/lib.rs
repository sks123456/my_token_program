use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, MintTo, Transfer, Burn, Approve, Revoke};
use std::str::FromStr;

declare_id!("5tHswiNub3VMUoXcMgfNAgAPmZMnSwoRPsC5jkFjWeBL");

#[program]
pub mod my_token_program {
    use super::*;

    pub fn mint_token(ctx: Context<MintToken>) -> Result<()> {
        msg!("Minting 10 tokens to account: {}", ctx.accounts.token_account.key());
        msg!("Mint authority: {}", ctx.accounts.authority.key());
    
        let _mint_address = Pubkey::from_str("mntLJRzjHeXDkA3AKv7eUGB5wkK2wmnJJBydWMM8c9t").unwrap();
        let _authority_address = Pubkey::from_str("samAH4Ygc4XFrvzkdduVC8fw5e8Mchm6DuG2DgMrmSQ").unwrap();
    
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
    
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, 10)?;
        Ok(())
    }    

    pub fn transfer_token(ctx: Context<TransferToken>) -> Result<()> {
        msg!("Transferring 5 tokens from account: {} to account: {}", ctx.accounts.from.key(), ctx.accounts.to.key());
        msg!("Transfer authority: {}", ctx.accounts.from_authority.key());
        let transfer_instruction = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.from_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, transfer_instruction);
        token::transfer(cpi_ctx, 5)?;
        Ok(())
    }

    pub fn burn_token(ctx: Context<BurnToken>, amount: u64) -> Result<()> {
        msg!("Burning {} tokens from account: {}", amount, ctx.accounts.token_account.key());
        msg!("Burn authority: {}", ctx.accounts.authority.key());
        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::burn(cpi_ctx, amount)?;
        Ok(())
    }
    pub fn approve_delegate(ctx: Context<ApproveDelegate>, amount: u64) -> Result<()> {
        msg!("Approving delegate to spend {} tokens from account: {}", amount, ctx.accounts.token_account.key());
        msg!("Owner authority: {}", ctx.accounts.owner.key());
        let cpi_accounts = Approve {
            to: ctx.accounts.token_account.to_account_info(),
            delegate: ctx.accounts.delegate.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::approve(cpi_ctx, amount)?;
        Ok(())
    }
    pub fn revoke_delegate(ctx: Context<RevokeDelegate>) -> Result<()> {
        msg!("Revoking delegate authority for account: {}", ctx.accounts.token_account.key());
        msg!("Owner authority: {}", ctx.accounts.owner.key());
        let cpi_accounts = Revoke {
            source: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::revoke(cpi_ctx)?;
        Ok(())
    }
       
}

#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TransferToken<'info> {
    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub from: UncheckedAccount<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub from_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct BurnToken<'info> {
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ApproveDelegate<'info> {
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    pub delegate: AccountInfo<'info>,
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
    pub struct RevokeDelegate<'info> {
        #[account(mut)]
        pub token_account: UncheckedAccount<'info>,
        pub owner: Signer<'info>,
        pub token_program: Program<'info, Token>,
    }
