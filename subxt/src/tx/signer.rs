// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! A library to **sub**mit e**xt**rinsics to a
//! [substrate](https://github.com/paritytech/substrate) node via RPC.

use crate::Config;
use sp_core::Pair;
use sp_runtime::traits::{
    IdentifyAccount,
    Verify,
};

/// Signing transactions requires a [`Signer`]. This is responsible for
/// providing the "from" account that the transaction is being signed by,
/// as well as actually signing a SCALE encoded payload.
pub trait Signer<T: Config> {
    /// Return the "from" account ID.
    fn account_id(&self) -> &T::AccountId;

    /// Return the "from" address.
    fn address(&self) -> T::Address;

    /// Takes a signer payload for an extrinsic, and returns a signature based on it.
    ///
    /// Some signers may fail, for instance because the hardware on which the keys are located has
    /// refused the operation.
    fn sign(&self, signer_payload: &[u8]) -> T::Signature;
}

/// A [`Signer`] implementation that can be constructed from an [`Pair`].
#[derive(Clone, Debug)]
pub struct PairSigner<T: Config, P: Pair> {
    account_id: T::AccountId,
    signer: P,
}

impl<T, P> PairSigner<T, P>
where
    T: Config,
    T::Signature: From<P::Signature>,
    <T::Signature as Verify>::Signer:
        From<P::Public> + IdentifyAccount<AccountId = T::AccountId>,
    P: Pair,
{
    /// Creates a new [`Signer`] from a [`Pair`].
    pub fn new(signer: P) -> Self {
        let account_id =
            <T::Signature as Verify>::Signer::from(signer.public()).into_account();
        Self { account_id, signer }
    }

    /// Returns the [`Pair`] implementation used to construct this.
    pub fn signer(&self) -> &P {
        &self.signer
    }

    /// Return the account ID.
    pub fn account_id(&self) -> &T::AccountId {
        &self.account_id
    }
}

impl<T, P> Signer<T> for PairSigner<T, P>
where
    T: Config,
    T::AccountId: Into<T::Address> + Clone + 'static,
    P: Pair + 'static,
    P::Signature: Into<T::Signature> + 'static,
{
    fn account_id(&self) -> &T::AccountId {
        &self.account_id
    }

    fn address(&self) -> T::Address {
        self.account_id.clone().into()
    }

    fn sign(&self, signer_payload: &[u8]) -> T::Signature {
        self.signer.sign(signer_payload).into()
    }
}
