use cfg_traits::liquidity_pools::Codec;
use parity_scale_codec::{Decode, Encode, Error, Input, MaxEncodedLen};
use scale_info::TypeInfo;

#[derive(Debug, Eq, PartialEq, Clone, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub enum MessageMock {
	First,
	Second,
}

impl MessageMock {
	fn call_type(&self) -> u8 {
		match self {
			MessageMock::First => 0,
			MessageMock::Second => 1,
		}
	}
}

impl Codec for MessageMock {
	fn serialize(&self) -> Vec<u8> {
		vec![self.call_type()]
	}

	fn deserialize<I: Input>(input: &mut I) -> Result<Self, Error> {
		let call_type = input.read_byte()?;

		match call_type {
			0 => Ok(MessageMock::First),
			1 => Ok(MessageMock::Second),
			_ => Err("unsupported message".into()),
		}
	}
}

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use cfg_traits::liquidity_pools::InboundQueue;
	use frame_support::pallet_prelude::*;
	use mock_builder::{execute_call, register_call};

	use crate::liquidity_pools::MessageMock;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type DomainAddress;
		type Message;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	type CallIds<T: Config> = StorageMap<_, _, String, mock_builder::CallId>;

	impl<T: Config> Pallet<T> {
		pub fn mock_submit(f: impl Fn(T::DomainAddress, MessageMock) -> DispatchResult + 'static) {
			register_call!(move |(sender, msg)| f(sender, msg));
		}
	}

	impl<T: Config> InboundQueue for Pallet<T> {
		type Message = T::Message;
		type Sender = T::DomainAddress;

		fn submit(sender: Self::Sender, msg: Self::Message) -> DispatchResult {
			execute_call!((sender, msg))
		}
	}
}
