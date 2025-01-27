pub mod handle;

use std::collections::HashMap;

use cfg_primitives::{Balance, BlockNumber, Nonce};
use fudge::primitives::Chain;
use handle::{FudgeHandle, ParachainClient};
use sc_client_api::HeaderBackend;
use sp_api::{ApiRef, ProvideRuntimeApi};
use sp_core::H256;
use sp_runtime::{DispatchError, Storage};

use crate::{
	generic::{
		config::Runtime,
		env::{utils, Env},
	},
	utils::accounts::Keyring,
};

/// Trait that represent a runtime with Fudge support
pub trait FudgeSupport: Runtime {
	/// Type to interact with fudge
	type FudgeHandle: FudgeHandle<Self>;
}

pub type FudgeRelayRuntime<T> = <<T as FudgeSupport>::FudgeHandle as FudgeHandle<T>>::RelayRuntime;

/// Evironment that uses fudge to interact with the runtime
pub struct FudgeEnv<T: Runtime + FudgeSupport> {
	handle: T::FudgeHandle,
	nonce_storage: HashMap<Keyring, Nonce>,
}

impl<T: Runtime + FudgeSupport> Default for FudgeEnv<T> {
	fn default() -> Self {
		Self::from_storage(Default::default(), Default::default(), Default::default())
	}
}

impl<T: Runtime + FudgeSupport> Env<T> for FudgeEnv<T> {
	fn from_parachain_storage(parachain_storage: Storage) -> Self {
		Self::from_storage(Default::default(), parachain_storage, Default::default())
	}

	fn from_storage(
		relay_storage: Storage,
		parachain_storage: Storage,
		sibling_storage: Storage,
	) -> Self {
		let mut handle = T::FudgeHandle::new(relay_storage, parachain_storage, sibling_storage);

		handle.evolve();

		Self {
			handle,
			nonce_storage: HashMap::default(),
		}
	}

	fn submit_now(
		&mut self,
		_who: Keyring,
		_call: impl Into<T::RuntimeCallExt>,
	) -> Result<Balance, DispatchError> {
		unimplemented!("FudgeEnv does not support submit_now() try submit_later()")
	}

	fn submit_later(
		&mut self,
		who: Keyring,
		call: impl Into<T::RuntimeCallExt>,
	) -> Result<(), Box<dyn std::error::Error>> {
		let nonce = *self.nonce_storage.entry(who).or_default();

		let extrinsic = self.parachain_state(|| utils::create_extrinsic::<T>(who, call, nonce));

		self.handle
			.parachain_mut()
			.append_extrinsic(extrinsic)
			.map(|_| ())?;

		self.nonce_storage.insert(who, nonce + 1);

		Ok(())
	}

	fn relay_state_mut<R>(&mut self, f: impl FnOnce() -> R) -> R {
		self.handle.relay_mut().with_mut_state(f).unwrap()
	}

	fn relay_state<R>(&self, f: impl FnOnce() -> R) -> R {
		self.handle.relay().with_state(f).unwrap()
	}

	fn parachain_state_mut<R>(&mut self, f: impl FnOnce() -> R) -> R {
		self.handle.parachain_mut().with_mut_state(f).unwrap()
	}

	fn parachain_state<R>(&self, f: impl FnOnce() -> R) -> R {
		self.handle.parachain().with_state(f).unwrap()
	}

	fn sibling_state_mut<R>(&mut self, f: impl FnOnce() -> R) -> R {
		self.handle.sibling_mut().with_mut_state(f).unwrap()
	}

	fn sibling_state<R>(&self, f: impl FnOnce() -> R) -> R {
		self.handle.sibling().with_state(f).unwrap()
	}

	fn __priv_build_block(&mut self, i: BlockNumber) {
		let current = self.parachain_state(|| frame_system::Pallet::<T>::block_number());
		if i > current + 1 {
			panic!("Jump to future blocks is unsupported in fudge (maybe you've used Blocks::BySecondsFast?)");
		}
		self.handle.evolve();
	}
}

type ApiRefOf<'a, T> = ApiRef<
	'a,
	<ParachainClient<
		<T as Runtime>::BlockExt,
		<<T as FudgeSupport>::FudgeHandle as FudgeHandle<T>>::ParachainConstructApi,
	> as sp_api::ProvideRuntimeApi<<T as Runtime>::BlockExt>>::Api,
>;

/// Specialized fudge methods
impl<T: Runtime + FudgeSupport> FudgeEnv<T> {
	pub fn chain_state_mut<R>(&mut self, chain: Chain, f: impl FnOnce() -> R) -> R {
		self.handle.with_mut_state(chain, f)
	}

	pub fn chain_state<R>(&self, chain: Chain, f: impl FnOnce() -> R) -> R {
		self.handle.with_state(chain, f)
	}

	pub fn with_api<F>(&self, exec: F)
	where
		F: FnOnce(ApiRefOf<T>, H256),
	{
		let client = self.handle.parachain().client();
		let best_hash = client.info().best_hash;
		let api = client.runtime_api();

		exec(api, best_hash);
	}
}

mod tests {
	use cfg_primitives::CFG;

	use super::*;
	use crate::generic::{env::Blocks, utils::genesis::Genesis};

	#[test_runtimes(all)]
	fn correct_nonce_for_submit_later<T: Runtime + FudgeSupport>() {
		let mut env = FudgeEnv::<T>::from_parachain_storage(
			Genesis::default()
				.add(pallet_balances::GenesisConfig::<T> {
					balances: vec![(Keyring::Alice.id(), 1 * CFG)],
				})
				.storage(),
		);

		env.submit_later(
			Keyring::Alice,
			frame_system::Call::remark { remark: vec![] },
		)
		.unwrap();

		env.submit_later(
			Keyring::Alice,
			frame_system::Call::remark { remark: vec![] },
		)
		.unwrap();

		env.pass(Blocks::ByNumber(1));

		env.submit_later(
			Keyring::Alice,
			frame_system::Call::remark { remark: vec![] },
		)
		.unwrap();
	}
}
