// Copyright (C) Haderech Pte. Ltd.
// SPDX-License-Identifier: Apache-2.0

/// EVM transaction processor.
module engine::evm {
    use std::signer;

    use aptos_framework::auth_data::{Self, AbstractionAuthData};
    use aptos_framework::transaction_context;

    const EEVM_ERROR: u64 = 1;

    public fun authenticate(account: signer, data: AbstractionAuthData): signer {
        assert!(auth_data::digest(&data).is_empty(), EEVM_ERROR);
        assert!(auth_data::authenticator(&data).is_empty(), EEVM_ERROR);

        let payload = transaction_context::entry_function_payload().extract();

        let to = transaction_context::account_address(&payload);
        assert!(to == @engine, EEVM_ERROR);
        let module_name = transaction_context::module_name(&payload);
        assert!(module_name.bytes() == &b"evm", EEVM_ERROR);
        let function_name = transaction_context::function_name(&payload);
        assert!(function_name.bytes() == &b"transact", EEVM_ERROR);

        let origin = signer::address_of(&account);
        let args = transaction_context::args(&payload);
        let tx = args.borrow(1);

        set_tx(origin, tx);

        account
    }

    native fun set_tx(origin: address, tx_data: &vector<u8>);

    public entry fun transact(account: &signer, tx: vector<u8>) {}
}
