use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program_error::ProgramError,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
    program::invoke,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let amount = u64::from_le_bytes(instruction_data.try_into().expect("Invalid amount"));

    let account_info_iter = &mut accounts.iter();
    let borrower_account = next_account_info(account_info_iter)?;
    let liquidity_pool_account = next_account_info(account_info_iter)?;
    let system_program_account = next_account_info(account_info_iter)?;
    let rent_sysvar_account = next_account_info(account_info_iter)?;

    if !borrower_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let rent = &Rent::from_account_info(rent_sysvar_account)?;
    if !rent.is_exempt(borrower_account.lamports(), borrower_account.data_len()) {
        return Err(ProgramError::AccountNotRentExempt);
    }

    let borrow_instruction = system_instruction::transfer(
        liquidity_pool_account.key,
        borrower_account.key,
        amount,
    );
    invoke(
        &borrow_instruction,
        &[
            liquidity_pool_account.clone(),
            borrower_account.clone(),
            system_program_account.clone(),
        ],
    )?;
    msg!("Borrowed {} lamports", amount);

    let repay_instruction = system_instruction::transfer(
        borrower_account.key,
        liquidity_pool_account.key,
        amount,
    );
    invoke(
        &repay_instruction,
        &[
            borrower_account.clone(),
            liquidity_pool_account.clone(),
            system_program_account.clone(),
        ],
    )?;
    msg!("Repaid {} lamports", amount);

    Ok(())
}
