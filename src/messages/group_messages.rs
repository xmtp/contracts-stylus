//! XMTP Group Messages Contract.
//!
//! This contract is used to store and retrieve messages for a group.
//!
use alloc::vec::Vec;

use alloy_primitives::{U256, FixedBytes, Bytes};
use alloy_sol_types::sol;
use stylus_sdk::{
    abi::Bytes as AbiBytes,
    evm,
    prelude::*,
    stylus_proc::public,
};

sol! {
    #[allow(missing_docs)]
    event MessageSent(bytes32 groupId, bytes message, uint256 sequenceId);
}

sol_storage! {
    #[entrypoint]
    pub struct GroupMessages {
        uint256 sequence_id;
    }
}

#[public]
impl GroupMessages {
    /// Stores a new MLS commit message.
    /// This replicates `addMessage(bytes32 groupId, bytes message)` from the Solidity contract.
    /// Emits a `MessageSent` log with `(groupId, message, sequenceId)`
    pub fn add_message(&mut self, group_id: FixedBytes<32>, message: AbiBytes) {
        let sequence_id = self.sequence_id.get();
        self.sequence_id.set(sequence_id + U256::from(1));

        evm::log(MessageSent {
            groupId: group_id,
            message: Bytes::from(message.to_vec()),
            sequenceId: self.sequence_id.get(),
        });
    }
}
