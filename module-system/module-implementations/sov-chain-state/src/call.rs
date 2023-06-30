use borsh::{BorshDeserialize, BorshSerialize};
use sov_rollup_interface::zk::traits::{StateTransition, ValidityCondition};
use sov_state::WorkingSet;

use crate::ChainState;

impl<
        Ctx: sov_modules_api::Context,
        Cond: ValidityCondition + BorshSerialize + BorshDeserialize,
    > ChainState<Ctx, Cond>
{
    /// Increment the current slot height
    pub(crate) fn increment_slot_height(&self, working_set: &mut WorkingSet<Ctx::Storage>) {
        let current_height = self
            .slot_height
            .get(working_set)
            .expect("Block height must be initialized");
        self.slot_height.set(&(current_height + 1), working_set);
    }

    /// Store the previous state transition
    pub(crate) fn store_state_transition(
        &self,
        height: u64,
        transition: StateTransition<Cond>,
        working_set: &mut WorkingSet<Ctx::Storage>,
    ) {
        
        self.historical_transitions
            .set(&height, &transition, working_set);
    }
}