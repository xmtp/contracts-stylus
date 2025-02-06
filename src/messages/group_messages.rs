//! XMTP Group Messages Contract.
//!
//! Contract module which allows implementing an emergency stop mechanism
//! that can be triggered by an authorized account.
//!
//! It provides functions [`Pausable::when_not_paused`]
//! and [`Pausable::when_paused`],
//! which can be added to the functions of your contract.
//!
//! Note that your contract will not be pausable by simply including this
//! module, only once and where you use [`Pausable::when_not_paused`].
//!
use alloc::vec::Vec;

use alloy_primitives::{U256, FixedBytes, Bytes};
use stylus_sdk::{
    evm,
    prelude::storage,
    storage::StorageU256,
    stylus_proc::public,
};
use alloy_sol_types::sol;

sol! {
    #[allow(missing_docs)]
    event MessageSent(bytes32 groupId, bytes message, uint256 sequenceId);
}


/// State of a [`GroupMessages`] Contract.
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
