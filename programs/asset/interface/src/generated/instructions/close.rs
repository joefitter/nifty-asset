//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct Close {
    /// The unitialized buffer account
    pub buffer: solana_program::pubkey::Pubkey,
    /// The account receiving refunded rent
    pub recipient: solana_program::pubkey::Pubkey,
}

impl Close {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.buffer,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.recipient,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = CloseInstructionData::new().try_to_vec().unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::INTERFACE_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
struct CloseInstructionData {
    discriminator: u8,
}

impl CloseInstructionData {
    fn new() -> Self {
        Self { discriminator: 0 }
    }
}

/// Instruction builder for `Close`.
///
/// ### Accounts:
///
///   0. `[writable, signer]` buffer
///   1. `[writable]` recipient
#[derive(Default)]
pub struct CloseBuilder {
    buffer: Option<solana_program::pubkey::Pubkey>,
    recipient: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CloseBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// The unitialized buffer account
    #[inline(always)]
    pub fn buffer(&mut self, buffer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.buffer = Some(buffer);
        self
    }
    /// The account receiving refunded rent
    #[inline(always)]
    pub fn recipient(&mut self, recipient: solana_program::pubkey::Pubkey) -> &mut Self {
        self.recipient = Some(recipient);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = Close {
            buffer: self.buffer.expect("buffer is not set"),
            recipient: self.recipient.expect("recipient is not set"),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `close` CPI accounts.
pub struct CloseCpiAccounts<'a, 'b> {
    /// The unitialized buffer account
    pub buffer: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account receiving refunded rent
    pub recipient: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `close` CPI instruction.
pub struct CloseCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The unitialized buffer account
    pub buffer: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account receiving refunded rent
    pub recipient: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> CloseCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: CloseCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            buffer: accounts.buffer,
            recipient: accounts.recipient,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.buffer.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.recipient.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = CloseInstructionData::new().try_to_vec().unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::INTERFACE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(2 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.buffer.clone());
        account_infos.push(self.recipient.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `Close` via CPI.
///
/// ### Accounts:
///
///   0. `[writable, signer]` buffer
///   1. `[writable]` recipient
pub struct CloseCpiBuilder<'a, 'b> {
    instruction: Box<CloseCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CloseCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CloseCpiBuilderInstruction {
            __program: program,
            buffer: None,
            recipient: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// The unitialized buffer account
    #[inline(always)]
    pub fn buffer(
        &mut self,
        buffer: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.buffer = Some(buffer);
        self
    }
    /// The account receiving refunded rent
    #[inline(always)]
    pub fn recipient(
        &mut self,
        recipient: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.recipient = Some(recipient);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let instruction = CloseCpi {
            __program: self.instruction.__program,

            buffer: self.instruction.buffer.expect("buffer is not set"),

            recipient: self.instruction.recipient.expect("recipient is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct CloseCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    buffer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    recipient: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
