#[cfg(test)]
mod tests {
    use mollusk_svm::{result::Check, Mollusk};
    use solana_sdk::{
        account::AccountSharedData,
        instruction::{AccountMeta, Instruction},
        native_token::LAMPORTS_PER_SOL,
        pubkey::{self, Pubkey},
    };

    const ID: Pubkey = pubkey::Pubkey::new_from_array([
        0x0f, 0x1e, 0x6b, 0x14, 0x21, 0xc0, 0x4a, 0x07, 0x04, 0x31, 0x26, 0x5c, 0x19, 0xc5, 0xbb,
        0xee, 0x19, 0x92, 0xba, 0xe8, 0xaf, 0xd1, 0xcd, 0x07, 0x8e, 0xf8, 0xaf, 0x70, 0x47, 0xdc,
        0x11, 0xf7,
    ]);

    #[test]
    fn test_deposit() {
        let mollusk = Mollusk::new(&ID, "target/deploy/blueshift_vault");

        let (system_program, system_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        let owner = Pubkey::new_from_array([0x02; 32]);
        let owner_account = AccountSharedData::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

        let (vault, _vault_bump) =
            solana_sdk::pubkey::Pubkey::find_program_address(&[(b"vault"), owner.as_ref()], &ID);
        let vault_account = AccountSharedData::new(0, 0, &system_program);

        let data = [vec![0], 1_000_000u64.to_le_bytes().to_vec()].concat();

        let instruction = Instruction::new_with_bytes(
            ID,
            &data,
            vec![
                AccountMeta::new(owner, true),
                AccountMeta::new(vault, false),
                AccountMeta::new_readonly(system_program, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &instruction,
            &[
                (owner, owner_account.into()),
                (vault, vault_account.into()),
                (system_program, system_account),
            ],
            &[Check::success()],
        );
    }

    #[test]
    fn test_withdraw() {
        let mollusk = Mollusk::new(&ID, "target/deploy/blueshift_vault");

        let (system_program, system_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        let owner = Pubkey::new_from_array([0x02; 32]);
        let owner_account = AccountSharedData::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

        let (vault, _vault_bump) =
            solana_sdk::pubkey::Pubkey::find_program_address(&[(b"vault"), owner.as_ref()], &ID);
        let vault_account = AccountSharedData::new(0, 0, &system_program);

        let data = [vec![1]].concat();

        let instruction = Instruction::new_with_bytes(
            ID,
            &data,
            vec![
                AccountMeta::new(owner, true),
                AccountMeta::new(vault, false),
                AccountMeta::new_readonly(system_program, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &instruction,
            &[
                (owner, owner_account.into()),
                (vault, vault_account.into()),
                (system_program, system_account),
            ],
            &[Check::success()],
        );
    }
}
