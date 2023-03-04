/*
    Squads Multisig Program - Program Errors
    https://github.com/squads-protocol/squads-mpl
*/

use safe_anchor_lang::prelude::*;

#[error_code]
pub enum MsError {
    KeyNotInMultisig,
    InvalidTransactionState,
    InvalidNumberOfAccounts,
    InvalidInstructionAccount,
    InvalidAuthorityIndex,
    TransactionAlreadyExecuted,
    CannotRemoveSafeoMember,
    InvalidThreshold,
    DeprecatedTransaction,
    InstructionFailed,
    MaxMembersReached,
    EmptyMembers,
    PartialExecution,
    NotEnoughLamports,
}