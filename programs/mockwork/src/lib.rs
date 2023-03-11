use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod mockwork {
    use super::*;

    pub fn create(ctx: Context<MockThreadCreate>, amount: u64, pda_payer: bool) -> Result<()> {
        if !pda_payer {
            msg!("on curve payer");

            transfer(
                CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.payer.to_account_info(),
                        to: ctx.accounts.thread.to_account_info(),
                    },
                ),
                amount,
            )?;
        } else {
            msg!("off curve payer");

            // check off curve xfer
            let space = 256;
            let extra = amount;
            if ctx.accounts.thread.to_account_info().lamports() < space + extra {
                panic!("you need to use cpi_create_with_pda if calling with pda");
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MockThreadCreate<'info> {
    #[account(mut, signer)]
    /// CHECK: we don't care who pays as long as someone pays
    pub payer: AccountInfo<'info>,

    #[account(
        init,
        payer = payer,
        seeds = [
            "sick-thread-bro".as_ref(),
            payer.key.as_ref(),
        ],
        bump,
        space = 256,
    )]
    pub thread: Account<'info, MockThread>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Debug)]
pub struct MockThread {
    pub state: [u8; 128],
}

pub fn thread(payer: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&["sick-thread-bro".as_ref(), payer.as_ref()], &crate::ID).0
}

#[cfg(feature = "cpi")]
pub fn cpi_create_with_pda<'a, 'b, 'c, 'info>(
    cpi_ctx: crate::CpiContext<'a, 'b, 'c, 'info, crate::cpi::accounts::MockThreadCreate<'info>>,
    amount: u64,
) -> Result<()> {
    // replace with more complex thing
    let space = Rent::get().unwrap().minimum_balance(256);
    let total_transfer = space + amount;
    **cpi_ctx
        .accounts
        .payer
        .to_account_info()
        .try_borrow_mut_lamports()? -= total_transfer;
    **cpi_ctx
        .accounts
        .thread
        .to_account_info()
        .try_borrow_mut_lamports()? += total_transfer;

    crate::cpi::create(cpi_ctx, 100, true)?;

    Ok(())
}
