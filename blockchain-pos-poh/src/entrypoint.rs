pub type ProgramResult = Result<(), ProgramError>;

#[derive(Debug, Clone, PartialEq)]
pub enum ProgramError {
    IncorrectProgramId,
    InvalidArgument,
    InvalidInstructionData,
    InvalidAccountData,
    AccountDataTooSmall,
    InsufficientFounds,
    MissingRequiredSignature,
    StateDataTooSmall,
    ArithmeticOverflow,
    Custom(u64),
}
