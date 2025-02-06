//! XMTP Group Messages Contract.
//!
//! This contract is used to store and retrieve messages for a group.
//!
use alloc::vec::Vec;

use alloy_primitives::{U256, FixedBytes, Bytes};
use stylus_sdk::{
    evm,
    prelude::{storage, entrypoint},
    storage::StorageU256,
    stylus_proc::public,
};
use alloy_sol_types::sol;

sol! {
    #[allow(missing_docs)]
    event MessageSent(bytes32 groupId, bytes message, uint256 sequenceId);
}


/// State of a [`GroupMessages`] Contract.
#[entrypoint]
#[storage]
pub struct GroupMessages {
    pub sequence_id: StorageU256
}


#[public]
impl GroupMessages {
    /// Stores a new MLS commit message.
    /// This replicates `addMessage(bytes32 groupId, bytes message)` from the Solidity contract.
    /// Emits a `MessageSent` log with `(groupId, message, sequenceId)`
    pub fn add_message(&mut self, group_id: FixedBytes<32>, message: Vec<u8>) {
        let current_seq = self.sequence_id.get();
        let new_seq = current_seq + U256::from(1);
        self.sequence_id.set(new_seq);

        evm::log(MessageSent {
            groupId: group_id,
            message: Bytes::from(message),
            sequenceId: new_seq,
        });
    }
}
