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

pub mod evm;

use aptos_native_interface::{RawSafeNative, SafeNativeBuilder};
use move_core_types::{account_address::AccountAddress, identifier::Identifier};
use move_vm_runtime::native_functions::NativeFunctionTable;

pub fn all_natives(
    address: AccountAddress,
    builder: &mut SafeNativeBuilder,
) -> NativeFunctionTable {
    builder.with_incremental_gas_charging(false, |builder| {
        builder
            .make_named_natives([("set_tx", evm::native_set_tx as RawSafeNative)])
            .map(|(func_name, func)| {
                (
                    address,
                    Identifier::new("evm").unwrap(),
                    Identifier::new(func_name).unwrap(),
                    func,
                )
            })
            .collect()
    })
}
