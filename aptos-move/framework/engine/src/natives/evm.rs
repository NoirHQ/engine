// Copyright (C) Haderech Pte. Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use alloy_consensus::TxEnvelope;
use alloy_evm::FromRecoveredTx;
use alloy_rlp::Decodable;
use aptos_native_interface::{
    safely_pop_arg, SafeNativeContext, SafeNativeError, SafeNativeResult,
};
use better_any::{Tid, TidAble};
use move_core_types::account_address::AccountAddress;
use move_vm_types::{
    loaded_data::runtime_types::Type,
    values::{Value, VectorRef},
};
use revm::{context::TxEnv, primitives::Address};
use smallvec::{smallvec, SmallVec};
use std::{cell::RefCell, collections::VecDeque};

const EEVM_ERROR: u64 = 1;

fn evm_error<E>(_: E) -> SafeNativeError {
    SafeNativeError::Abort {
        abort_code: EEVM_ERROR,
    }
}

#[derive(Tid)]
pub struct NativeEvmContext {
    tx: RefCell<Option<TxEnv>>,
}

impl NativeEvmContext {
    pub fn new() -> Self {
        Self {
            tx: Default::default(),
        }
    }
}

fn account_address(eth: &Address) -> AccountAddress {
    let mut data = [0u8; 32];
    data[12..].copy_from_slice(eth.as_slice());
    AccountAddress::new(data)
}

pub(crate) fn native_set_tx(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    assert!(ty_args.is_empty());
    assert_eq!(args.len(), 1);

    let tx_data = safely_pop_arg!(args, VectorRef);
    let origin = safely_pop_arg!(args, AccountAddress);

    let tx = TxEnvelope::decode(&mut &tx_data.as_bytes_ref()[..]).map_err(evm_error)?;
    let signer = tx.recover_signer().map_err(evm_error)?;

    if origin == account_address(&signer) {
        return Err(SafeNativeError::Abort {
            abort_code: EEVM_ERROR,
        });
    }

    let tx = TxEnv::from_recovered_tx(&tx, signer);

    let evm_context = context.extensions().get::<NativeEvmContext>();
    *evm_context.tx.borrow_mut() = Some(tx);

    // TODO: transaction validation

    Ok(smallvec![])
}
