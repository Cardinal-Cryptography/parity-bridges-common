// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! Substrate node RPC errors.

use crate::{BlockNumberOf, Chain, HashOf};

use bp_polkadot_core::parachains::ParaId;
use jsonrpsee::core::Error as RpcError;
use relay_utils::MaybeConnectionError;
use sc_rpc_api::system::Health;
use sp_core::{storage::StorageKey, Bytes};
use sp_runtime::transaction_validity::TransactionValidityError;
use thiserror::Error;

/// Result type used by Substrate client.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur only when interacting with
/// a Substrate node through RPC.
#[derive(Error, Debug)]
pub enum Error {
	/// IO error.
	#[error("IO error: {0}")]
	Io(#[from] std::io::Error),
	/// An error that can occur when making a request to
	/// an JSON-RPC server.
	#[error("RPC error: {0}")]
	RpcError(#[from] RpcError),
	/// The response from the server could not be SCALE decoded.
	#[error("Response parse failed: {0}")]
	ResponseParseFailed(#[from] codec::Error),
	/// Internal channel error - communication channel is either closed, or full.
	/// It can be solved with reconnect.
	#[error("Internal communication channel error: {0:?}.")]
	ChannelError(String),
	/// Required parachain head is not present at the relay chain.
	#[error("Parachain {0:?} head {1} is missing from the relay chain storage.")]
	MissingRequiredParachainHead(ParaId, u64),
	/// Failed to find finality proof for the given header.
	#[error("Failed to find finality proof for header {0}.")]
	FinalityProofNotFound(u64),
	/// The client we're connected to is not synced, so we can't rely on its state.
	#[error("Substrate client is not synced {0}.")]
	ClientNotSynced(Health),
	/// Failed to get system health.
	#[error("Failed to get system health of {chain} node: {error:?}.")]
	FailedToGetSystemHealth {
		/// Name of the chain where the error has happened.
		chain: String,
		/// Underlying error.
		error: Box<Error>,
	},
	/// Failed to read best finalized header hash from given chain.
	#[error("Failed to read best finalized header hash of {chain}: {error:?}.")]
	FailedToReadBestFinalizedHeaderHash {
		/// Name of the chain where the error has happened.
		chain: String,
		/// Underlying error.
		error: Box<Error>,
	},
	/// Failed to read best finalized header from given chain.
	#[error("Failed to read best header of {chain}: {error:?}.")]
	FailedToReadBestHeader {
		/// Name of the chain where the error has happened.
		chain: String,
		/// Underlying error.
		error: Box<Error>,
	},
	/// Failed to read header hash by number from given chain.
	#[error("Failed to read header hash by number {number} of {chain}: {error:?}.")]
	FailedToReadHeaderHashByNumber {
		/// Name of the chain where the error has happened.
		chain: String,
		/// Number of the header we've tried to read.
		number: String,
		/// Underlying error.
		error: Box<Error>,
	},
	/// Failed to read header by hash from given chain.
	#[error("Failed to read header {hash} of {chain}: {error:?}.")]
	FailedToReadHeaderByHash {
		/// Name of the chain where the error has happened.
		chain: String,
		/// Hash of the header we've tried to read.
		hash: String,
		/// Underlying error.
		error: Box<Error>,
	},
	/// Failed to read block by hash from given chain.
	#[error("Failed to read block {hash} of {chain}: {error:?}.")]
	FailedToReadBlockByHash {
		/// Name of the chain where the error has happened.
		chain: String,
		/// Hash of the header we've tried to read.
		hash: String,
		/// Underlying error.
		error: Box<Error>,
	},
	/// Failed to read sotrage value at given chain.
	#[error("Failed to read storage value {key:?} at {chain}: {error:?}.")]
	FailedToReadStorageValue {
		/// Name of the chain where the error has happened.
		chain: String,
		/// Hash of the block we've tried to read value from.
		hash: String,
		/// Runtime storage key
		key: StorageKey,
		/// Underlying error.
		error: Box<Error>,
	},
	/// Failed to read runtime version of given chain.
	#[error("Failed to read runtime version of {chain}: {error:?}.")]
	FailedToReadRuntimeVersion {
		/// Name of the chain where the error has happened.
		chain: String,
		/// Underlying error.
		error: Box<Error>,
	},
	/// Failed to get pending extrinsics.
	#[error("Failed to get pending extrinsics of {chain}: {error:?}.")]
	FailedToGetPendingExtrinsics {
		/// Name of the chain where the error has happened.
		chain: String,
		/// Underlying error.
		error: Box<Error>,
	},
	/// Failed to submit transaction.
	#[error("Failed to submit {chain} transaction: {error:?}.")]
	FailedToSubmitTransaction {
		/// Name of the chain where the error has happened.
		chain: String,
		/// Underlying error.
		error: Box<Error>,
	},
	/// Runtime call has failed.
	#[error("Runtime call {method} with arguments {arguments:?} of chain {chain} at {hash} has failed: {error:?}.")]
	FailedStateCall {
		/// Name of the chain where the error has happened.
		chain: String,
		/// Hash of the block we've tried to call at.
		hash: String,
		/// Runtime API method.
		method: String,
		/// Encoded method arguments.
		arguments: Bytes,
		/// Underlying error.
		error: Box<Error>,
	},
	/// Failed to prove storage keys.
	#[error("Failed to prove storage keys {storage_keys:?} of {chain} at {hash}: {error:?}.")]
	FailedToProveStorage {
		/// Name of the chain where the error has happened.
		chain: String,
		/// Hash of the block we've tried to prove keys at.
		hash: String,
		/// Storage keys we have tried to prove.
		storage_keys: Vec<StorageKey>,
		/// Underlying error.
		error: Box<Error>,
	},
	/// Failed to subscribe to GRANDPA justifications stream.
	#[error("Failed to subscribe to {chain} justifications: {error:?}.")]
	FailedToSubscribeJustifications {
		/// Name of the chain where the error has happened.
		chain: String,
		/// Underlying error.
		error: Box<Error>,
	},
	/// The bridge pallet is halted and all transactions will be rejected.
	#[error("Bridge pallet is halted.")]
	BridgePalletIsHalted,
	/// The bridge pallet is not yet initialized and all transactions will be rejected.
	#[error("Bridge pallet is not initialized.")]
	BridgePalletIsNotInitialized,
	/// The Substrate transaction is invalid.
	#[error("Substrate transaction is invalid: {0:?}")]
	TransactionInvalid(#[from] TransactionValidityError),
	/// Custom logic error.
	#[error("{0}")]
	Custom(String),
}

impl From<tokio::task::JoinError> for Error {
	fn from(error: tokio::task::JoinError) -> Self {
		Error::ChannelError(format!("failed to wait tokio task: {error}"))
	}
}

impl<T> From<async_std::channel::TrySendError<T>> for Error {
	fn from(error: async_std::channel::TrySendError<T>) -> Self {
		Error::ChannelError(format!("`try_send` has failed: {error:?}"))
	}
}

impl From<async_std::channel::RecvError> for Error {
	fn from(error: async_std::channel::RecvError) -> Self {
		Error::ChannelError(format!("`recv` has failed: {error:?}"))
	}
}

impl Error {
	/// Box the error.
	pub fn boxed(self) -> Box<Self> {
		Box::new(self)
	}

	/// Returns nested error reference.
	pub fn nested(&self) -> Option<&Self> {
		match *self {
			Self::FailedToReadBestFinalizedHeaderHash { ref error, .. } => Some(&**error),
			Self::FailedToReadBestHeader { ref error, .. } => Some(&**error),
			Self::FailedToReadHeaderHashByNumber { ref error, .. } => Some(&**error),
			Self::FailedToReadHeaderByHash { ref error, .. } => Some(&**error),
			Self::FailedToReadBlockByHash { ref error, .. } => Some(&**error),
			Self::FailedToReadStorageValue { ref error, .. } => Some(&**error),
			Self::FailedToReadRuntimeVersion { ref error, .. } => Some(&**error),
			Self::FailedToGetPendingExtrinsics { ref error, .. } => Some(&**error),
			Self::FailedToSubmitTransaction { ref error, .. } => Some(&**error),
			Self::FailedStateCall { ref error, .. } => Some(&**error),
			Self::FailedToProveStorage { ref error, .. } => Some(&**error),
			Self::FailedToGetSystemHealth { ref error, .. } => Some(&**error),
			Self::FailedToSubscribeJustifications { ref error, .. } => Some(&**error),
			_ => None,
		}
	}

	/// Constructs `FailedToReadHeaderHashByNumber` variant.
	pub fn failed_to_read_header_hash_by_number<C: Chain>(
		number: BlockNumberOf<C>,
		e: Error,
	) -> Self {
		Error::FailedToReadHeaderHashByNumber {
			chain: C::NAME.into(),
			number: format!("{number}"),
			error: e.boxed(),
		}
	}

	/// Constructs `FailedToReadHeaderByHash` variant.
	pub fn failed_to_read_header_by_hash<C: Chain>(hash: HashOf<C>, e: Error) -> Self {
		Error::FailedToReadHeaderByHash {
			chain: C::NAME.into(),
			hash: format!("{hash}"),
			error: e.boxed(),
		}
	}

	/// Constructs `FailedToReadBlockByHash` variant.
	pub fn failed_to_read_block_by_hash<C: Chain>(hash: HashOf<C>, e: Error) -> Self {
		Error::FailedToReadHeaderByHash {
			chain: C::NAME.into(),
			hash: format!("{hash}"),
			error: e.boxed(),
		}
	}

	/// Constructs `FailedToReadBestFinalizedHeaderHash` variant.
	pub fn failed_to_read_best_finalized_header_hash<C: Chain>(e: Error) -> Self {
		Error::FailedToReadBestFinalizedHeaderHash { chain: C::NAME.into(), error: e.boxed() }
	}

	/// Constructs `FailedToReadBestHeader` variant.
	pub fn failed_to_read_best_header<C: Chain>(e: Error) -> Self {
		Error::FailedToReadBestHeader { chain: C::NAME.into(), error: e.boxed() }
	}

	/// Constructs `FailedToReadRuntimeVersion` variant.
	pub fn failed_to_read_runtime_version<C: Chain>(e: Error) -> Self {
		Error::FailedToReadRuntimeVersion { chain: C::NAME.into(), error: e.boxed() }
	}

	/// Constructs `FailedToReadStorageValue` variant.
	pub fn failed_to_read_storage_value<C: Chain>(
		at: HashOf<C>,
		key: StorageKey,
		e: Error,
	) -> Self {
		Error::FailedToReadStorageValue {
			chain: C::NAME.into(),
			hash: format!("{at}"),
			key,
			error: e.boxed(),
		}
	}

	/// Constructs `FailedToGetPendingExtrinsics` variant.
	pub fn failed_to_get_pending_extrinsics<C: Chain>(e: Error) -> Self {
		Error::FailedToGetPendingExtrinsics { chain: C::NAME.into(), error: e.boxed() }
	}

	/// Constructs `FailedToSubmitTransaction` variant.
	pub fn failed_to_submit_transaction<C: Chain>(e: Error) -> Self {
		Error::FailedToSubmitTransaction { chain: C::NAME.into(), error: e.boxed() }
	}

	/// Constructs `FailedStateCall` variant.
	pub fn failed_state_call<C: Chain>(
		at: HashOf<C>,
		method: String,
		arguments: Bytes,
		e: Error,
	) -> Self {
		Error::FailedStateCall {
			chain: C::NAME.into(),
			hash: format!("{at}"),
			method,
			arguments,
			error: e.boxed(),
		}
	}

	/// Constructs `FailedToProveStorage` variant.
	pub fn failed_to_prove_storage<C: Chain>(
		at: HashOf<C>,
		storage_keys: Vec<StorageKey>,
		e: Error,
	) -> Self {
		Error::FailedToProveStorage {
			chain: C::NAME.into(),
			hash: format!("{at}"),
			storage_keys,
			error: e.boxed(),
		}
	}

	/// Constructs `FailedToGetSystemHealth` variant.
	pub fn failed_to_get_system_health<C: Chain>(e: Error) -> Self {
		Error::FailedToGetSystemHealth { chain: C::NAME.into(), error: e.boxed() }
	}

	/// Constructs `FailedToSubscribeJustifications` variant.
	pub fn failed_to_subscribe_justification<C: Chain>(e: Error) -> Self {
		Error::FailedToSubscribeJustifications { chain: C::NAME.into(), error: e.boxed() }
	}
}

impl MaybeConnectionError for Error {
	fn is_connection_error(&self) -> bool {
		match *self {
			Error::ChannelError(_) => true,
			Error::RpcError(ref e) =>
				matches!(*e, RpcError::Transport(_) | RpcError::RestartNeeded(_),),
			Error::ClientNotSynced(_) => true,
			_ => self.nested().map(|e| e.is_connection_error()).unwrap_or(false),
		}
	}
}
