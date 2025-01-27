// Copyright 2024 Centrifuge Foundation (centrifuge.io).
// This file is part of Centrifuge Chain project.

// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).

// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
//! # Swaps Pallet
//!
//! The Swaps pallet enables applying swaps independently of previous swaps in
//! the same or opposite directions.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use cfg_traits::{
		swaps::{OrderRatio, Swap, SwapInfo, SwapStatus, Swaps, TokenSwaps},
		StatusNotificationHook,
	};
	use frame_support::pallet_prelude::*;
	use sp_runtime::traits::{AtLeast32BitUnsigned, EnsureAdd, EnsureSub, Zero};
	use sp_std::cmp::Ordering;

	use super::*;

	pub type RatioOf<T> =
		<<T as Config>::OrderBook as TokenSwaps<<T as frame_system::Config>::AccountId>>::Ratio;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it
	/// depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Represents an amount that can be swapped
		type Balance: Parameter + Member + AtLeast32BitUnsigned + Default + Copy + MaxEncodedLen;

		/// An identification for a swap
		type SwapId: Parameter + Member + Copy + Ord + MaxEncodedLen;

		/// An identification for an order
		type OrderId: Parameter + Member + Copy + Ord + MaxEncodedLen;

		/// The currency type of transferrable tokens
		type CurrencyId: Parameter + Member + Copy + MaxEncodedLen;

		/// The type which exposes token swap order functionality
		type OrderBook: TokenSwaps<
			Self::AccountId,
			CurrencyId = Self::CurrencyId,
			BalanceIn = Self::Balance,
			BalanceOut = Self::Balance,
			OrderId = Self::OrderId,
		>;

		/// The hook which acts upon a (partially) fulfilled the swap
		type FulfilledSwap: StatusNotificationHook<
			Id = (Self::AccountId, Self::SwapId),
			Status = SwapInfo<Self::Balance, Self::Balance, Self::CurrencyId, RatioOf<Self>>,
			Error = DispatchError,
		>;
	}

	/// Maps a `OrderId` to its corresponding `AccountId` and `SwapId`
	///
	/// NOTE: The storage is killed when the swap order no longer exists
	#[pallet::storage]
	pub type OrderIdToSwapId<T: Config> =
		StorageMap<_, Blake2_128Concat, T::OrderId, (T::AccountId, T::SwapId)>;

	/// Maps an `AccountId` and `SwapId` to its corresponding `OrderId`
	///
	/// NOTE: The storage is killed when the swap order no longer exists
	#[pallet::storage]
	pub type SwapIdToOrderId<T: Config> =
		StorageMap<_, Blake2_128Concat, (T::AccountId, T::SwapId), T::OrderId>;

	#[pallet::error]
	pub enum Error<T> {
		/// Failed to retrieve the order.
		OrderNotFound,

		/// Failed to retrieve the swap.
		SwapNotFound,

		/// Emitted when the cancelled amount is greater than the pending amount
		CancelMoreThanPending,
	}

	impl<T: Config> Pallet<T> {
		pub fn swap_id(order_id: T::OrderId) -> Result<(T::AccountId, T::SwapId), DispatchError> {
			OrderIdToSwapId::<T>::get(order_id).ok_or(Error::<T>::SwapNotFound.into())
		}

		pub fn order_id(
			account: &T::AccountId,
			swap_id: T::SwapId,
		) -> Result<T::OrderId, DispatchError> {
			SwapIdToOrderId::<T>::get((account, swap_id)).ok_or(Error::<T>::OrderNotFound.into())
		}

		pub(crate) fn update_id(
			who: &T::AccountId,
			swap_id: T::SwapId,
			new_order_id: Option<T::OrderId>,
		) -> DispatchResult {
			let previous_order_id = SwapIdToOrderId::<T>::get((who, swap_id));

			if previous_order_id != new_order_id {
				if let Some(old_id) = previous_order_id {
					OrderIdToSwapId::<T>::remove(old_id);
					SwapIdToOrderId::<T>::remove((who.clone(), swap_id));
				}

				if let Some(new_id) = new_order_id {
					OrderIdToSwapId::<T>::insert(new_id, (who.clone(), swap_id));
					SwapIdToOrderId::<T>::insert((who.clone(), swap_id), new_id);
				}
			}

			Ok(())
		}

		#[allow(clippy::type_complexity)]
		fn apply_over(
			who: &T::AccountId,
			new_swap: Swap<T::Balance, T::CurrencyId>,
			over_order_id: Option<T::OrderId>,
		) -> Result<(SwapStatus<T::Balance>, Option<T::OrderId>), DispatchError> {
			match over_order_id {
				None => {
					let order_id = if !new_swap.amount_out.is_zero() {
						Some(T::OrderBook::place_order(
							who.clone(),
							new_swap.currency_in,
							new_swap.currency_out,
							new_swap.amount_out,
							OrderRatio::Market,
						)?)
					} else {
						None
					};

					Ok((
						SwapStatus {
							swapped: T::Balance::zero(),
							pending: new_swap.amount_out,
						},
						order_id,
					))
				}
				Some(order_id) => {
					let swap = T::OrderBook::get_order_details(order_id)
						.ok_or(Error::<T>::OrderNotFound)?
						.swap;

					if swap.is_same_direction(&new_swap)? {
						let amount_to_swap = swap.amount_out.ensure_add(new_swap.amount_out)?;
						T::OrderBook::update_order(order_id, amount_to_swap, OrderRatio::Market)?;

						Ok((
							SwapStatus {
								swapped: T::Balance::zero(),
								pending: amount_to_swap,
							},
							Some(order_id),
						))
					} else {
						let inverse_swap = swap;

						let new_swap_amount_in = T::OrderBook::convert_by_market(
							new_swap.currency_in,
							new_swap.currency_out,
							new_swap.amount_out,
						)?;

						match inverse_swap.amount_out.cmp(&new_swap_amount_in) {
							Ordering::Greater => {
								let amount_to_swap =
									inverse_swap.amount_out.ensure_sub(new_swap_amount_in)?;

								T::OrderBook::update_order(
									order_id,
									amount_to_swap,
									OrderRatio::Market,
								)?;

								Ok((
									SwapStatus {
										swapped: new_swap_amount_in,
										pending: T::Balance::zero(),
									},
									Some(order_id),
								))
							}
							Ordering::Equal => {
								T::OrderBook::cancel_order(order_id)?;

								Ok((
									SwapStatus {
										swapped: new_swap_amount_in,
										pending: T::Balance::zero(),
									},
									None,
								))
							}
							Ordering::Less => {
								T::OrderBook::cancel_order(order_id)?;

								let inverse_swap_amount_in = T::OrderBook::convert_by_market(
									inverse_swap.currency_in,
									inverse_swap.currency_out,
									inverse_swap.amount_out,
								)?;

								let amount_to_swap =
									new_swap.amount_out.ensure_sub(inverse_swap_amount_in)?;

								let order_id = if !amount_to_swap.is_zero() {
									Some(T::OrderBook::place_order(
										who.clone(),
										new_swap.currency_in,
										new_swap.currency_out,
										amount_to_swap,
										OrderRatio::Market,
									)?)
								} else {
									None
								};

								Ok((
									SwapStatus {
										swapped: inverse_swap.amount_out,
										pending: amount_to_swap,
									},
									order_id,
								))
							}
						}
					}
				}
			}
		}

		fn cancel_over(
			cancel_amount: T::Balance,
			currency_id: T::CurrencyId,
			over_order_id: T::OrderId,
		) -> Result<Option<T::OrderId>, DispatchError> {
			let swap = T::OrderBook::get_order_details(over_order_id)
				.ok_or(Error::<T>::OrderNotFound)?
				.swap;

			if swap.currency_out == currency_id {
				match swap.amount_out.cmp(&cancel_amount) {
					Ordering::Greater => {
						let amount_to_swap = swap.amount_out.ensure_sub(cancel_amount)?;
						T::OrderBook::update_order(
							over_order_id,
							amount_to_swap,
							OrderRatio::Market,
						)?;

						Ok(Some(over_order_id))
					}
					Ordering::Equal => {
						T::OrderBook::cancel_order(over_order_id)?;

						Ok(None)
					}
					Ordering::Less => Err(Error::<T>::CancelMoreThanPending)?,
				}
			} else {
				Ok(Some(over_order_id)) //Noop
			}
		}
	}

	/// Trait to perform swaps without handling directly an order book
	impl<T: Config> Swaps<T::AccountId> for Pallet<T> {
		type Amount = T::Balance;
		type CurrencyId = T::CurrencyId;
		type SwapId = T::SwapId;

		fn apply_swap(
			who: &T::AccountId,
			swap_id: Self::SwapId,
			swap: Swap<T::Balance, T::CurrencyId>,
		) -> Result<SwapStatus<Self::Amount>, DispatchError> {
			// Bypassing the swap if both currencies are the same
			if swap.currency_in == swap.currency_out {
				return Ok(SwapStatus {
					swapped: swap.amount_out,
					pending: T::Balance::zero(),
				});
			}

			let previous_order_id = SwapIdToOrderId::<T>::get((who, swap_id));

			let (status, new_order_id) = Self::apply_over(who, swap, previous_order_id)?;

			Self::update_id(who, swap_id, new_order_id)?;

			Ok(status)
		}

		fn cancel_swap(
			who: &T::AccountId,
			swap_id: Self::SwapId,
			balance: T::Balance,
			currency_id: T::CurrencyId,
		) -> DispatchResult {
			match SwapIdToOrderId::<T>::get((who, swap_id)) {
				Some(previous_order_id) => {
					let order_id = Self::cancel_over(balance, currency_id, previous_order_id)?;
					Self::update_id(who, swap_id, order_id)
				}
				None => Ok(()), // Noop
			}
		}

		fn pending_amount(
			who: &T::AccountId,
			swap_id: Self::SwapId,
			from_currency: Self::CurrencyId,
		) -> Result<Self::Amount, DispatchError> {
			Ok(SwapIdToOrderId::<T>::get((who, swap_id))
				.and_then(T::OrderBook::get_order_details)
				.filter(|order_info| order_info.swap.currency_out == from_currency)
				.map(|order_info| order_info.swap.amount_out)
				.unwrap_or_default())
		}
	}

	impl<T: Config> StatusNotificationHook for Pallet<T> {
		type Error = DispatchError;
		type Id = T::OrderId;
		type Status = SwapInfo<T::Balance, T::Balance, T::CurrencyId, RatioOf<T>>;

		fn notify_status_change(order_id: T::OrderId, swap_info: Self::Status) -> DispatchResult {
			if let Ok((who, swap_id)) = Self::swap_id(order_id) {
				if swap_info.remaining.amount_out.is_zero() {
					Self::update_id(&who, swap_id, None)?;
				}

				T::FulfilledSwap::notify_status_change((who, swap_id), swap_info)?;
			}

			Ok(())
		}
	}
}
