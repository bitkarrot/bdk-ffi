mod bitcoin;
mod descriptor;
mod electrum;
mod error;
mod esplora;
mod keys;
mod macros;
mod store;
mod tx_builder;
mod types;
mod wallet;

use crate::bitcoin::Address;
use crate::bitcoin::AddressData;
use crate::bitcoin::Amount;
use crate::bitcoin::FeeRate;
use crate::bitcoin::FinalizedPsbtResult;
use crate::bitcoin::Header;
use crate::bitcoin::OutPoint;
use crate::bitcoin::Psbt;
use crate::bitcoin::Script;
use crate::bitcoin::Transaction;
use crate::bitcoin::TxIn;
use crate::bitcoin::TxOut;
use crate::bitcoin::WitnessProgram;
use crate::descriptor::Descriptor;
use crate::electrum::ElectrumClient;
use crate::electrum::HeaderNotification;
use crate::electrum::ServerFeaturesRes;
use crate::error::AddressParseError;
use crate::error::Bip32Error;
use crate::error::Bip39Error;
use crate::error::CalculateFeeError;
use crate::error::CannotConnectError;
use crate::error::CreateTxError;
use crate::error::CreateWithPersistError;
use crate::error::DescriptorError;
use crate::error::DescriptorKeyError;
use crate::error::ElectrumError;
use crate::error::EsploraError;
use crate::error::ExtractTxError;
use crate::error::FeeRateError;
use crate::error::FromScriptError;
use crate::error::LoadWithPersistError;
use crate::error::MiniscriptError;
use crate::error::ParseAmountError;
use crate::error::PersistenceError;
use crate::error::PsbtError;
use crate::error::PsbtFinalizeError;
use crate::error::PsbtParseError;
use crate::error::RequestBuilderError;
use crate::error::SignerError;
use crate::error::SqliteError;
use crate::error::TransactionError;
use crate::error::TxidParseError;
use crate::esplora::EsploraClient;
use crate::keys::DerivationPath;
use crate::keys::DescriptorPublicKey;
use crate::keys::DescriptorSecretKey;
use crate::keys::Mnemonic;
use crate::store::Connection;
use crate::tx_builder::BumpFeeTxBuilder;
use crate::tx_builder::TxBuilder;
use crate::types::AddressInfo;
use crate::types::Balance;
use crate::types::BlockId;
use crate::types::CanonicalTx;
use crate::types::ChainPosition;
use crate::types::Condition;
use crate::types::ConfirmationBlockTime;
use crate::types::FullScanRequest;
use crate::types::FullScanRequestBuilder;
use crate::types::FullScanScriptInspector;
use crate::types::KeychainAndIndex;
use crate::types::LocalOutput;
use crate::types::LockTime;
use crate::types::PkOrF;
use crate::types::Policy;
use crate::types::Satisfaction;
use crate::types::SatisfiableItem;
use crate::types::ScriptAmount;
use crate::types::SentAndReceivedValues;
use crate::types::SignOptions;
use crate::types::SyncRequest;
use crate::types::SyncRequestBuilder;
use crate::types::SyncScriptInspector;
use crate::types::Tx;
use crate::types::TxStatus;
use crate::types::Update;
use crate::wallet::Wallet;

use bdk_wallet::bitcoin::Network;
use bdk_wallet::keys::bip39::WordCount;
use bdk_wallet::tx_builder::ChangeSpendPolicy;
use bdk_wallet::ChangeSet;
use bdk_wallet::KeychainKind;

uniffi::include_scaffolding!("bdk");
