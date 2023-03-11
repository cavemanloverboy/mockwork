use anchor_lang::{
    prelude::*,
    solana_program::native_token::LAMPORTS_PER_SOL,
    system_program::{transfer, Transfer},
};
use mockwork::{cpi::accounts::MockThreadCreate, program::Mockwork};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnW");

#[program]
pub mod user {

    use mockwork::cpi_create_with_pda;

    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        // Transfer extra sol
        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.admin.to_account_info(),
                    to: ctx.accounts.payer.to_account_info(),
                },
            ),
            LAMPORTS_PER_SOL,
        )?;

        Ok(())
    }

    pub fn use_create(ctx: Context<UseCreate>) -> Result<()> {
        // Create cpi_ctx for MockThreadCreate
        let signer_seeds: &[&[&[u8]]] =
            &[&["pda-payoooor".as_ref(), &[*ctx.bumps.get("payer").unwrap()]]];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.mockwork_program.to_account_info(),
            MockThreadCreate {
                payer: ctx.accounts.payer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                thread: ctx.accounts.thread.to_account_info(),
            },
            signer_seeds,
        );

        // Create mockwork thread with pda as signer
        cpi_create_with_pda(cpi_ctx, 100)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct UseCreate<'info> {
    #[account(
        mut,
        seeds = ["pda-payoooor".as_ref()],
        bump,
    )]
    pub payer: Account<'info, Payer>,

    #[account(mut)]
    /// CHECK: checked by mockwork
    pub thread: AccountInfo<'info>,

    pub mockwork_program: Program<'info, Mockwork>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Payer {}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = 8,
        seeds = ["pda-payoooor".as_ref()],
        bump,
    )]
    pub payer: Account<'info, Payer>,

    pub system_program: Program<'info, System>,
}

pub fn payer() -> Pubkey {
    Pubkey::find_program_address(&["pda-payoooor".as_ref()], &crate::ID).0
}
