//! This module contains constants that are needed throughout the codebase.

/// The maximum size that a contract can have when being deployed on the
/// blockchain.
///
/// This is specified in [EIP-170](https://eips.ethereum.org/EIPS/eip-170).
pub const CONTRACT_MAXIMUM_SIZE_BYTES: usize = 24576;

/// The base byte value for the `PUSH` opcode, for `N > 0`.
///
/// This is constructed such that for `PUSHN`, `PUSH_OPCODE_BASE_VALUE` + `N`
/// equals the byte value for the corresponding `PUSH` opcode.
pub const PUSH_OPCODE_BASE_VALUE: u8 = 0x5f;

/// The base byte value for the `DUP` opcode.
///
/// This is constructed such that for `DUPN`, `DUP_OPCODE_BASE_VALUE` + `N`
/// equals the byte value for the corresponding `DUP` opcode.
pub const DUP_OPCODE_BASE_VALUE: u8 = 0x7f;

/// The base byte value for the `SWAP` opcode.
///
/// This is constructed such that for `SWAPN`, `SWAP_OPCODE_BASE_VALUE` + `N`
/// equals the byte value for the corresponding `SWAP` opcode.
pub const SWAP_OPCODE_BASE_VALUE: u8 = 0x8f;

/// The base byte value for the `LOG` opcode.
///
/// This is constructed such that for `LOGN`, `LOG_OPCODE_BASE_VALUE` + `N`
/// equals the byte value for the corresponding `LOG` opcode.
pub const LOG_OPCODE_BASE_VALUE: u8 = 0xa0;

/// The maximum number of bytes that can be pushed at once using the `PUSH`
/// opcode.
pub const PUSH_OPCODE_MAX_BYTES: u8 = 32;