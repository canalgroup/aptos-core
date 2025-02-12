// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

//! MoveVM and Session wrapped, to make sure Aptos natives and extensions are always installed and
//! taken care of after session finish.
mod resolver;
pub mod session;
mod vm;
mod warm_vm_cache;
pub(crate) mod write_op_converter;

pub use crate::move_vm_ext::{
    resolver::{AptosMoveResolver, AsExecutorView, AsResourceGroupView, ResourceGroupResolver},
    session::SessionExt,
    vm::{get_max_binary_format_version, get_max_identifier_size, verifier_config, MoveVmExt},
};
pub use aptos_types::transaction::user_transaction_context::UserTransactionContext;
use aptos_types::{access_path::AccessPath, state_store::state_key::StateKey};
use move_binary_format::errors::{PartialVMError, PartialVMResult};
use move_core_types::{
    account_address::AccountAddress, language_storage::StructTag, vm_status::StatusCode,
};
pub use session::session_id::SessionId;

pub(crate) fn resource_state_key(
    address: AccountAddress,
    tag: StructTag,
) -> PartialVMResult<StateKey> {
    let access_path = AccessPath::resource_access_path(address, tag).map_err(|e| {
        PartialVMError::new(StatusCode::VALUE_SERIALIZATION_ERROR)
            .with_message(format!("Failed to serialize struct tag: {}", e))
    })?;
    Ok(StateKey::access_path(access_path))
}
