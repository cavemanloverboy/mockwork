use std::{error::Error, rc::Rc};

use anchor_client::{
    anchor_lang::system_program,
    solana_sdk::{signature::read_keypair_file, signer::Signer},
    Client, Cluster,
};

fn main() -> Result<(), Box<dyn Error>> {
    // let admin = Rc::new(read_keypair_file("../admin.json")?) as Rc<dyn Signer>;
    let admin = Rc::new(read_keypair_file("admin.json")?) as Rc<dyn Signer>;

    let client = Client::new(Cluster::Localnet, Rc::clone(&admin));
    let program = client.program(user::ID);
    let mockwork_program = client.program(mockwork::ID);

    program
        .request()
        .accounts(user::accounts::Init {
            admin: admin.pubkey(),
            payer: user::payer(),
            system_program: system_program::ID,
        })
        .args(user::instruction::Init {})
        .send()?;

    program
        .request()
        .accounts(user::accounts::UseCreate {
            payer: user::payer(),
            thread: mockwork::thread(user::payer()),
            mockwork_program: mockwork::ID,
            system_program: system_program::ID,
        })
        .args(user::instruction::UseCreate {})
        .send()
        .unwrap();

    mockwork_program
        .request()
        .accounts(mockwork::accounts::MockThreadCreate {
            payer: admin.pubkey(),
            thread: mockwork::thread(admin.pubkey()),
            system_program: system_program::ID,
        })
        .args(mockwork::instruction::Create {
            amount: 100,
            pda_payer: false,
        })
        .send()
        .unwrap();

    Ok(())
}
