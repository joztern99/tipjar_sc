use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    system_instruction,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
};

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8], // Ignored, all instructions are hello world
) -> ProgramResult {
    msg!("Tip Jar program entrypoint");

    let accounts_iter = &mut accounts.iter();
    let funder_account = next_account_info(accounts_iter)?; // Account sending the tip
    let tip_jar_account = next_account_info(accounts_iter)?; // Account receiving the tip

    if !funder_account.is_signer {
        msg!("Funder account must be a signer");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let amount_to_tip = 1_000_000; // Amount to tip in lamports (smallest unit of SOL)
    msg!("Tipping {} lamports", amount_to_tip);

    invoke(
        &system_instruction::transfer(
            &funder_account.key,
            &tip_jar_account.key,
            amount_to_tip,
        ),
        &[funder_account.clone(), tip_jar_account.clone()],
    )?;

    msg!("Tip completed successfully");
    Ok(())
}
