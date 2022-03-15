#![allow(non_camel_case_types)]
//! This file was automatically generated using the following command:
//!
//! ```bash
//! contrib/msggen/msggen/__main__.py
//! ```
//!
//! Do not edit this file, it'll be overwritten. Rather edit the schema that
//! this file was generated from

use serde::{Deserialize, Serialize};
pub use requests::*;
pub use responses::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "method", content = "params")]
#[serde(rename_all = "lowercase")]
pub enum Request {
	Getinfo(requests::GetinfoRequest),
	ListPeers(requests::ListpeersRequest),
	ListFunds(requests::ListfundsRequest),
	ListChannels(requests::ListchannelsRequest),
	AddGossip(requests::AddgossipRequest),
	AutoCleanInvoice(requests::AutocleaninvoiceRequest),
	CheckMessage(requests::CheckmessageRequest),
	Close(requests::CloseRequest),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "method", content = "result")]
#[serde(rename_all = "lowercase")]
pub enum Response {
	Getinfo(responses::GetinfoResponse),
	ListPeers(responses::ListpeersResponse),
	ListFunds(responses::ListfundsResponse),
	ListChannels(responses::ListchannelsResponse),
	AddGossip(responses::AddgossipResponse),
	AutoCleanInvoice(responses::AutocleaninvoiceResponse),
	CheckMessage(responses::CheckmessageResponse),
	Close(responses::CloseResponse),
}

pub mod requests {
    #[allow(unused_imports)]
    use crate::primitives::*;
    #[allow(unused_imports)]
    use serde::{{Deserialize, Serialize}};

	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct GetinfoRequest {
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct ListpeersRequest {
	    #[serde(alias = "id", skip_serializing_if = "Option::is_none")]
	    pub id: Option<String>,
	    #[serde(alias = "level", skip_serializing_if = "Option::is_none")]
	    pub level: Option<String>,
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct ListfundsRequest {
	    #[serde(alias = "spent", skip_serializing_if = "Option::is_none")]
	    pub spent: Option<bool>,
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct ListchannelsRequest {
	    #[serde(alias = "short_channel_id", skip_serializing_if = "Option::is_none")]
	    pub short_channel_id: Option<String>,
	    #[serde(alias = "source", skip_serializing_if = "Option::is_none")]
	    pub source: Option<String>,
	    #[serde(alias = "destination", skip_serializing_if = "Option::is_none")]
	    pub destination: Option<String>,
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct AddgossipRequest {
	    #[serde(alias = "message")]
	    pub message: String,
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct AutocleaninvoiceRequest {
	    #[serde(alias = "expired_by", skip_serializing_if = "Option::is_none")]
	    pub expired_by: Option<u64>,
	    #[serde(alias = "cycle_seconds", skip_serializing_if = "Option::is_none")]
	    pub cycle_seconds: Option<u64>,
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct CheckmessageRequest {
	    #[serde(alias = "message")]
	    pub message: String,
	    #[serde(alias = "zbase")]
	    pub zbase: String,
	    #[serde(alias = "pubkey", skip_serializing_if = "Option::is_none")]
	    pub pubkey: Option<String>,
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct CloseRequest {
	    #[serde(alias = "id")]
	    pub id: String,
	    #[serde(alias = "unilateraltimeout", skip_serializing_if = "Option::is_none")]
	    pub unilateraltimeout: Option<u32>,
	    #[serde(alias = "destination", skip_serializing_if = "Option::is_none")]
	    pub destination: Option<String>,
	    #[serde(alias = "fee_negotiation_step", skip_serializing_if = "Option::is_none")]
	    pub fee_negotiation_step: Option<String>,
	    #[serde(alias = "wrong_funding", skip_serializing_if = "Option::is_none")]
	    pub wrong_funding: Option<String>,
	    #[serde(alias = "force_lease_closed", skip_serializing_if = "Option::is_none")]
	    pub force_lease_closed: Option<bool>,
	}

}


pub mod responses {
    #[allow(unused_imports)]
    use crate::primitives::*;
    #[allow(unused_imports)]
    use serde::{{Deserialize, Serialize}};

	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct GetinfoOur_features {
	    #[serde(alias = "init")]
	    pub init: String,
	    #[serde(alias = "node")]
	    pub node: String,
	    #[serde(alias = "channel")]
	    pub channel: String,
	    #[serde(alias = "invoice")]
	    pub invoice: String,
	}

	/// Type of connection
	#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
	#[serde(rename_all = "lowercase")]
	pub enum GetinfoAddressType {
	    DNS,
	    IPV4,
	    IPV6,
	    TORV2,
	    TORV3,
	    WEBSOCKET,
	}

	impl TryFrom<i32> for GetinfoAddressType {
	    type Error = anyhow::Error;
	    fn try_from(c: i32) -> Result<GetinfoAddressType, anyhow::Error> {
	        match c {
	    0 => Ok(GetinfoAddressType::DNS),
	    1 => Ok(GetinfoAddressType::IPV4),
	    2 => Ok(GetinfoAddressType::IPV6),
	    3 => Ok(GetinfoAddressType::TORV2),
	    4 => Ok(GetinfoAddressType::TORV3),
	    5 => Ok(GetinfoAddressType::WEBSOCKET),
	            o => Err(anyhow::anyhow!("Unknown variant {} for enum GetinfoAddressType", o)),
	        }
	    }
	}
	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct GetinfoAddress {
	    // Path `Getinfo.address[].type`
	    #[serde(rename = "type")]
	    pub item_type: GetinfoAddressType,
	    #[serde(alias = "port")]
	    pub port: u16,
	    #[serde(alias = "address", skip_serializing_if = "Option::is_none")]
	    pub address: Option<String>,
	}

	/// Type of connection
	#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
	#[serde(rename_all = "lowercase")]
	pub enum GetinfoBindingType {
	    LOCAL_SOCKET,
	    IPV4,
	    IPV6,
	    TORV2,
	    TORV3,
	}

	impl TryFrom<i32> for GetinfoBindingType {
	    type Error = anyhow::Error;
	    fn try_from(c: i32) -> Result<GetinfoBindingType, anyhow::Error> {
	        match c {
	    0 => Ok(GetinfoBindingType::LOCAL_SOCKET),
	    1 => Ok(GetinfoBindingType::IPV4),
	    2 => Ok(GetinfoBindingType::IPV6),
	    3 => Ok(GetinfoBindingType::TORV2),
	    4 => Ok(GetinfoBindingType::TORV3),
	            o => Err(anyhow::anyhow!("Unknown variant {} for enum GetinfoBindingType", o)),
	        }
	    }
	}
	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct GetinfoBinding {
	    // Path `Getinfo.binding[].type`
	    #[serde(rename = "type")]
	    pub item_type: GetinfoBindingType,
	    #[serde(alias = "address", skip_serializing_if = "Option::is_none")]
	    pub address: Option<String>,
	    #[serde(alias = "port", skip_serializing_if = "Option::is_none")]
	    pub port: Option<u16>,
	    #[serde(alias = "socket", skip_serializing_if = "Option::is_none")]
	    pub socket: Option<String>,
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct GetinfoResponse {
	    #[serde(alias = "id")]
	    pub id: String,
	    #[serde(alias = "alias")]
	    pub alias: String,
	    #[serde(alias = "color")]
	    pub color: String,
	    #[serde(alias = "num_peers")]
	    pub num_peers: u32,
	    #[serde(alias = "num_pending_channels")]
	    pub num_pending_channels: u32,
	    #[serde(alias = "num_active_channels")]
	    pub num_active_channels: u32,
	    #[serde(alias = "num_inactive_channels")]
	    pub num_inactive_channels: u32,
	    #[serde(alias = "version")]
	    pub version: String,
	    #[serde(alias = "lightning-dir")]
	    pub lightning_dir: String,
	    #[serde(alias = "blockheight")]
	    pub blockheight: u32,
	    #[serde(alias = "network")]
	    pub network: String,
	    #[serde(alias = "fees_collected_msat")]
	    pub fees_collected_msat: Amount,
	    #[serde(alias = "address")]
	    pub address: Vec<GetinfoAddress>,
	    #[serde(alias = "binding")]
	    pub binding: Vec<GetinfoBinding>,
	    #[serde(alias = "warning_bitcoind_sync", skip_serializing_if = "Option::is_none")]
	    pub warning_bitcoind_sync: Option<String>,
	    #[serde(alias = "warning_lightningd_sync", skip_serializing_if = "Option::is_none")]
	    pub warning_lightningd_sync: Option<String>,
	}

	#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
	#[serde(rename_all = "lowercase")]
	pub enum ListpeersPeersLogType {
	    SKIPPED,
	    BROKEN,
	    UNUSUAL,
	    INFO,
	    DEBUG,
	    IO_IN,
	    IO_OUT,
	}

	impl TryFrom<i32> for ListpeersPeersLogType {
	    type Error = anyhow::Error;
	    fn try_from(c: i32) -> Result<ListpeersPeersLogType, anyhow::Error> {
	        match c {
	    0 => Ok(ListpeersPeersLogType::SKIPPED),
	    1 => Ok(ListpeersPeersLogType::BROKEN),
	    2 => Ok(ListpeersPeersLogType::UNUSUAL),
	    3 => Ok(ListpeersPeersLogType::INFO),
	    4 => Ok(ListpeersPeersLogType::DEBUG),
	    5 => Ok(ListpeersPeersLogType::IO_IN),
	    6 => Ok(ListpeersPeersLogType::IO_OUT),
	            o => Err(anyhow::anyhow!("Unknown variant {} for enum ListpeersPeersLogType", o)),
	        }
	    }
	}
	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct ListpeersPeersLog {
	    // Path `ListPeers.peers[].log[].type`
	    #[serde(rename = "type")]
	    pub item_type: ListpeersPeersLogType,
	    #[serde(alias = "num_skipped", skip_serializing_if = "Option::is_none")]
	    pub num_skipped: Option<u32>,
	    #[serde(alias = "time", skip_serializing_if = "Option::is_none")]
	    pub time: Option<String>,
	    #[serde(alias = "source", skip_serializing_if = "Option::is_none")]
	    pub source: Option<String>,
	    #[serde(alias = "log", skip_serializing_if = "Option::is_none")]
	    pub log: Option<String>,
	    #[serde(alias = "node_id", skip_serializing_if = "Option::is_none")]
	    pub node_id: Option<String>,
	    #[serde(alias = "data", skip_serializing_if = "Option::is_none")]
	    pub data: Option<String>,
	}

	/// the channel state, in particular "CHANNELD_NORMAL" means the channel can be used normally
	#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
	#[serde(rename_all = "lowercase")]
	pub enum ListpeersPeersChannelsState {
	    OPENINGD,
	    CHANNELD_AWAITING_LOCKIN,
	    CHANNELD_NORMAL,
	    CHANNELD_SHUTTING_DOWN,
	    CLOSINGD_SIGEXCHANGE,
	    CLOSINGD_COMPLETE,
	    AWAITING_UNILATERAL,
	    FUNDING_SPEND_SEEN,
	    ONCHAIN,
	    DUALOPEND_OPEN_INIT,
	    DUALOPEND_AWAITING_LOCKIN,
	}

	impl TryFrom<i32> for ListpeersPeersChannelsState {
	    type Error = anyhow::Error;
	    fn try_from(c: i32) -> Result<ListpeersPeersChannelsState, anyhow::Error> {
	        match c {
	    0 => Ok(ListpeersPeersChannelsState::OPENINGD),
	    1 => Ok(ListpeersPeersChannelsState::CHANNELD_AWAITING_LOCKIN),
	    2 => Ok(ListpeersPeersChannelsState::CHANNELD_NORMAL),
	    3 => Ok(ListpeersPeersChannelsState::CHANNELD_SHUTTING_DOWN),
	    4 => Ok(ListpeersPeersChannelsState::CLOSINGD_SIGEXCHANGE),
	    5 => Ok(ListpeersPeersChannelsState::CLOSINGD_COMPLETE),
	    6 => Ok(ListpeersPeersChannelsState::AWAITING_UNILATERAL),
	    7 => Ok(ListpeersPeersChannelsState::FUNDING_SPEND_SEEN),
	    8 => Ok(ListpeersPeersChannelsState::ONCHAIN),
	    9 => Ok(ListpeersPeersChannelsState::DUALOPEND_OPEN_INIT),
	    10 => Ok(ListpeersPeersChannelsState::DUALOPEND_AWAITING_LOCKIN),
	            o => Err(anyhow::anyhow!("Unknown variant {} for enum ListpeersPeersChannelsState", o)),
	        }
	    }
	}
	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct ListpeersPeersChannelsFeerate {
	    #[serde(alias = "perkw")]
	    pub perkw: u32,
	    #[serde(alias = "perkb")]
	    pub perkb: u32,
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct ListpeersPeersChannelsInflight {
	    #[serde(alias = "funding_txid")]
	    pub funding_txid: String,
	    #[serde(alias = "funding_outnum")]
	    pub funding_outnum: u32,
	    #[serde(alias = "feerate")]
	    pub feerate: String,
	    #[serde(alias = "total_funding_msat")]
	    pub total_funding_msat: Amount,
	    #[serde(alias = "our_funding_msat")]
	    pub our_funding_msat: Amount,
	    #[serde(alias = "scratch_txid")]
	    pub scratch_txid: String,
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct ListpeersPeersChannelsFunding {
	    #[serde(alias = "local_msat")]
	    pub local_msat: Amount,
	    #[serde(alias = "remote_msat")]
	    pub remote_msat: Amount,
	    #[serde(alias = "pushed_msat")]
	    pub pushed_msat: Amount,
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct ListpeersPeersChannelsState_changes {
	    #[serde(alias = "timestamp")]
	    pub timestamp: String,
	    // Path `ListPeers.peers[].channels[].state_changes[].old_state`
	    #[serde(rename = "old_state")]
	    pub old_state: ChannelState,
	    // Path `ListPeers.peers[].channels[].state_changes[].new_state`
	    #[serde(rename = "new_state")]
	    pub new_state: ChannelState,
	    // Path `ListPeers.peers[].channels[].state_changes[].cause`
	    #[serde(rename = "cause")]
	    pub cause: ChannelStateChangeCause,
	    #[serde(alias = "message")]
	    pub message: String,
	}

	/// Whether it came from peer, or is going to peer
	#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
	#[serde(rename_all = "lowercase")]
	pub enum ListpeersPeersChannelsHtlcsDirection {
	    IN,
	    OUT,
	}

	impl TryFrom<i32> for ListpeersPeersChannelsHtlcsDirection {
	    type Error = anyhow::Error;
	    fn try_from(c: i32) -> Result<ListpeersPeersChannelsHtlcsDirection, anyhow::Error> {
	        match c {
	    0 => Ok(ListpeersPeersChannelsHtlcsDirection::IN),
	    1 => Ok(ListpeersPeersChannelsHtlcsDirection::OUT),
	            o => Err(anyhow::anyhow!("Unknown variant {} for enum ListpeersPeersChannelsHtlcsDirection", o)),
	        }
	    }
	}
	/// Status of the HTLC
	#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
	#[serde(rename_all = "lowercase")]
	pub enum ListpeersPeersChannelsHtlcsState {
	    SENT_ADD_HTLC,
	    SENT_ADD_COMMIT,
	    RCVD_ADD_REVOCATION,
	    RCVD_ADD_ACK_COMMIT,
	    SENT_ADD_ACK_REVOCATION,
	    RCVD_REMOVE_HTLC,
	    RCVD_REMOVE_COMMIT,
	    SENT_REMOVE_REVOCATION,
	    SENT_REMOVE_ACK_COMMIT,
	    RCVD_REMOVE_ACK_REVOCATION,
	}

	impl TryFrom<i32> for ListpeersPeersChannelsHtlcsState {
	    type Error = anyhow::Error;
	    fn try_from(c: i32) -> Result<ListpeersPeersChannelsHtlcsState, anyhow::Error> {
	        match c {
	    0 => Ok(ListpeersPeersChannelsHtlcsState::SENT_ADD_HTLC),
	    1 => Ok(ListpeersPeersChannelsHtlcsState::SENT_ADD_COMMIT),
	    2 => Ok(ListpeersPeersChannelsHtlcsState::RCVD_ADD_REVOCATION),
	    3 => Ok(ListpeersPeersChannelsHtlcsState::RCVD_ADD_ACK_COMMIT),
	    4 => Ok(ListpeersPeersChannelsHtlcsState::SENT_ADD_ACK_REVOCATION),
	    5 => Ok(ListpeersPeersChannelsHtlcsState::RCVD_REMOVE_HTLC),
	    6 => Ok(ListpeersPeersChannelsHtlcsState::RCVD_REMOVE_COMMIT),
	    7 => Ok(ListpeersPeersChannelsHtlcsState::SENT_REMOVE_REVOCATION),
	    8 => Ok(ListpeersPeersChannelsHtlcsState::SENT_REMOVE_ACK_COMMIT),
	    9 => Ok(ListpeersPeersChannelsHtlcsState::RCVD_REMOVE_ACK_REVOCATION),
	            o => Err(anyhow::anyhow!("Unknown variant {} for enum ListpeersPeersChannelsHtlcsState", o)),
	        }
	    }
	}
	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct ListpeersPeersChannelsHtlcs {
	    // Path `ListPeers.peers[].channels[].htlcs[].direction`
	    #[serde(rename = "direction")]
	    pub direction: ListpeersPeersChannelsHtlcsDirection,
	    #[serde(alias = "id")]
	    pub id: u64,
	    #[serde(alias = "amount_msat")]
	    pub amount_msat: Amount,
	    #[serde(alias = "expiry")]
	    pub expiry: u32,
	    #[serde(alias = "payment_hash")]
	    pub payment_hash: String,
	    #[serde(alias = "local_trimmed", skip_serializing_if = "Option::is_none")]
	    pub local_trimmed: Option<bool>,
	    #[serde(alias = "status", skip_serializing_if = "Option::is_none")]
	    pub status: Option<String>,
	    // Path `ListPeers.peers[].channels[].htlcs[].state`
	    #[serde(rename = "state")]
	    pub state: ListpeersPeersChannelsHtlcsState,
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct ListpeersPeersChannels {
	    // Path `ListPeers.peers[].channels[].state`
	    #[serde(rename = "state")]
	    pub state: ListpeersPeersChannelsState,
	    #[serde(alias = "scratch_txid", skip_serializing_if = "Option::is_none")]
	    pub scratch_txid: Option<String>,
	    #[serde(alias = "owner", skip_serializing_if = "Option::is_none")]
	    pub owner: Option<String>,
	    #[serde(alias = "short_channel_id", skip_serializing_if = "Option::is_none")]
	    pub short_channel_id: Option<String>,
	    #[serde(alias = "channel_id", skip_serializing_if = "Option::is_none")]
	    pub channel_id: Option<String>,
	    #[serde(alias = "funding_txid", skip_serializing_if = "Option::is_none")]
	    pub funding_txid: Option<String>,
	    #[serde(alias = "funding_outnum", skip_serializing_if = "Option::is_none")]
	    pub funding_outnum: Option<u32>,
	    #[serde(alias = "initial_feerate", skip_serializing_if = "Option::is_none")]
	    pub initial_feerate: Option<String>,
	    #[serde(alias = "last_feerate", skip_serializing_if = "Option::is_none")]
	    pub last_feerate: Option<String>,
	    #[serde(alias = "next_feerate", skip_serializing_if = "Option::is_none")]
	    pub next_feerate: Option<String>,
	    #[serde(alias = "next_fee_step", skip_serializing_if = "Option::is_none")]
	    pub next_fee_step: Option<u32>,
	    #[serde(alias = "inflight")]
	    pub inflight: Vec<ListpeersPeersChannelsInflight>,
	    #[serde(alias = "close_to", skip_serializing_if = "Option::is_none")]
	    pub close_to: Option<String>,
	    #[serde(alias = "private", skip_serializing_if = "Option::is_none")]
	    pub private: Option<bool>,
	    // Path `ListPeers.peers[].channels[].opener`
	    #[serde(rename = "opener")]
	    pub opener: ChannelSide,
	    pub closer: Option<ChannelSide>,
	    #[serde(alias = "features")]
	    pub features: Vec<String>,
	    #[serde(alias = "to_us_msat", skip_serializing_if = "Option::is_none")]
	    pub to_us_msat: Option<Amount>,
	    #[serde(alias = "min_to_us_msat", skip_serializing_if = "Option::is_none")]
	    pub min_to_us_msat: Option<Amount>,
	    #[serde(alias = "max_to_us_msat", skip_serializing_if = "Option::is_none")]
	    pub max_to_us_msat: Option<Amount>,
	    #[serde(alias = "total_msat", skip_serializing_if = "Option::is_none")]
	    pub total_msat: Option<Amount>,
	    #[serde(alias = "fee_base_msat", skip_serializing_if = "Option::is_none")]
	    pub fee_base_msat: Option<Amount>,
	    #[serde(alias = "fee_proportional_millionths", skip_serializing_if = "Option::is_none")]
	    pub fee_proportional_millionths: Option<u32>,
	    #[serde(alias = "dust_limit_msat", skip_serializing_if = "Option::is_none")]
	    pub dust_limit_msat: Option<Amount>,
	    #[serde(alias = "max_total_htlc_in_msat", skip_serializing_if = "Option::is_none")]
	    pub max_total_htlc_in_msat: Option<Amount>,
	    #[serde(alias = "their_reserve_msat", skip_serializing_if = "Option::is_none")]
	    pub their_reserve_msat: Option<Amount>,
	    #[serde(alias = "our_reserve_msat", skip_serializing_if = "Option::is_none")]
	    pub our_reserve_msat: Option<Amount>,
	    #[serde(alias = "spendable_msat", skip_serializing_if = "Option::is_none")]
	    pub spendable_msat: Option<Amount>,
	    #[serde(alias = "receivable_msat", skip_serializing_if = "Option::is_none")]
	    pub receivable_msat: Option<Amount>,
	    #[serde(alias = "minimum_htlc_in_msat", skip_serializing_if = "Option::is_none")]
	    pub minimum_htlc_in_msat: Option<Amount>,
	    #[serde(alias = "their_to_self_delay", skip_serializing_if = "Option::is_none")]
	    pub their_to_self_delay: Option<u32>,
	    #[serde(alias = "our_to_self_delay", skip_serializing_if = "Option::is_none")]
	    pub our_to_self_delay: Option<u32>,
	    #[serde(alias = "max_accepted_htlcs", skip_serializing_if = "Option::is_none")]
	    pub max_accepted_htlcs: Option<u32>,
	    #[serde(alias = "state_changes")]
	    pub state_changes: Vec<ListpeersPeersChannelsState_changes>,
	    #[serde(alias = "status")]
	    pub status: Vec<String>,
	    #[serde(alias = "in_payments_offered", skip_serializing_if = "Option::is_none")]
	    pub in_payments_offered: Option<u64>,
	    #[serde(alias = "in_offered_msat", skip_serializing_if = "Option::is_none")]
	    pub in_offered_msat: Option<Amount>,
	    #[serde(alias = "in_payments_fulfilled", skip_serializing_if = "Option::is_none")]
	    pub in_payments_fulfilled: Option<u64>,
	    #[serde(alias = "in_fulfilled_msat", skip_serializing_if = "Option::is_none")]
	    pub in_fulfilled_msat: Option<Amount>,
	    #[serde(alias = "out_payments_offered", skip_serializing_if = "Option::is_none")]
	    pub out_payments_offered: Option<u64>,
	    #[serde(alias = "out_offered_msat", skip_serializing_if = "Option::is_none")]
	    pub out_offered_msat: Option<Amount>,
	    #[serde(alias = "out_payments_fulfilled", skip_serializing_if = "Option::is_none")]
	    pub out_payments_fulfilled: Option<u64>,
	    #[serde(alias = "out_fulfilled_msat", skip_serializing_if = "Option::is_none")]
	    pub out_fulfilled_msat: Option<Amount>,
	    #[serde(alias = "htlcs")]
	    pub htlcs: Vec<ListpeersPeersChannelsHtlcs>,
	    #[serde(alias = "close_to_addr", skip_serializing_if = "Option::is_none")]
	    pub close_to_addr: Option<String>,
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct ListpeersPeers {
	    #[serde(alias = "id")]
	    pub id: String,
	    #[serde(alias = "connected")]
	    pub connected: bool,
	    #[serde(alias = "log")]
	    pub log: Vec<ListpeersPeersLog>,
	    #[serde(alias = "channels")]
	    pub channels: Vec<ListpeersPeersChannels>,
	    #[serde(alias = "netaddr")]
	    pub netaddr: Vec<String>,
	    #[serde(alias = "features", skip_serializing_if = "Option::is_none")]
	    pub features: Option<String>,
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct ListpeersResponse {
	    #[serde(alias = "peers")]
	    pub peers: Vec<ListpeersPeers>,
	}

	#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
	#[serde(rename_all = "lowercase")]
	pub enum ListfundsOutputsStatus {
	    UNCONFIRMED,
	    CONFIRMED,
	    SPENT,
	}

	impl TryFrom<i32> for ListfundsOutputsStatus {
	    type Error = anyhow::Error;
	    fn try_from(c: i32) -> Result<ListfundsOutputsStatus, anyhow::Error> {
	        match c {
	    0 => Ok(ListfundsOutputsStatus::UNCONFIRMED),
	    1 => Ok(ListfundsOutputsStatus::CONFIRMED),
	    2 => Ok(ListfundsOutputsStatus::SPENT),
	            o => Err(anyhow::anyhow!("Unknown variant {} for enum ListfundsOutputsStatus", o)),
	        }
	    }
	}
	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct ListfundsOutputs {
	    #[serde(alias = "txid")]
	    pub txid: String,
	    #[serde(alias = "output")]
	    pub output: u32,
	    #[serde(alias = "amount_msat")]
	    pub amount_msat: Amount,
	    #[serde(alias = "scriptpubkey")]
	    pub scriptpubkey: String,
	    #[serde(alias = "address", skip_serializing_if = "Option::is_none")]
	    pub address: Option<String>,
	    #[serde(alias = "redeemscript", skip_serializing_if = "Option::is_none")]
	    pub redeemscript: Option<String>,
	    // Path `ListFunds.outputs[].status`
	    #[serde(rename = "status")]
	    pub status: ListfundsOutputsStatus,
	    #[serde(alias = "blockheight", skip_serializing_if = "Option::is_none")]
	    pub blockheight: Option<u32>,
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct ListfundsChannels {
	    #[serde(alias = "peer_id")]
	    pub peer_id: String,
	    #[serde(alias = "our_amount_msat")]
	    pub our_amount_msat: Amount,
	    #[serde(alias = "amount_msat")]
	    pub amount_msat: Amount,
	    #[serde(alias = "funding_txid")]
	    pub funding_txid: String,
	    #[serde(alias = "funding_output")]
	    pub funding_output: u32,
	    #[serde(alias = "connected")]
	    pub connected: bool,
	    // Path `ListFunds.channels[].state`
	    #[serde(rename = "state")]
	    pub state: ChannelState,
	    #[serde(alias = "short_channel_id", skip_serializing_if = "Option::is_none")]
	    pub short_channel_id: Option<String>,
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct ListfundsResponse {
	    #[serde(alias = "outputs")]
	    pub outputs: Vec<ListfundsOutputs>,
	    #[serde(alias = "channels")]
	    pub channels: Vec<ListfundsChannels>,
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct ListchannelsChannels {
	    #[serde(alias = "source")]
	    pub source: String,
	    #[serde(alias = "destination")]
	    pub destination: String,
	    #[serde(alias = "short_channel_id")]
	    pub short_channel_id: String,
	    #[serde(alias = "public")]
	    pub public: bool,
	    #[serde(alias = "amount_msat")]
	    pub amount_msat: Amount,
	    #[serde(alias = "message_flags")]
	    pub message_flags: u8,
	    #[serde(alias = "channel_flags")]
	    pub channel_flags: u8,
	    #[serde(alias = "active")]
	    pub active: bool,
	    #[serde(alias = "last_update")]
	    pub last_update: u32,
	    #[serde(alias = "base_fee_millisatoshi")]
	    pub base_fee_millisatoshi: u32,
	    #[serde(alias = "fee_per_millionth")]
	    pub fee_per_millionth: u32,
	    #[serde(alias = "delay")]
	    pub delay: u32,
	    #[serde(alias = "htlc_minimum_msat")]
	    pub htlc_minimum_msat: Amount,
	    #[serde(alias = "htlc_maximum_msat", skip_serializing_if = "Option::is_none")]
	    pub htlc_maximum_msat: Option<Amount>,
	    #[serde(alias = "features")]
	    pub features: String,
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct ListchannelsResponse {
	    #[serde(alias = "channels")]
	    pub channels: Vec<ListchannelsChannels>,
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct AddgossipResponse {
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct AutocleaninvoiceResponse {
	    #[serde(alias = "enabled")]
	    pub enabled: bool,
	    #[serde(alias = "expired_by", skip_serializing_if = "Option::is_none")]
	    pub expired_by: Option<u64>,
	    #[serde(alias = "cycle_seconds", skip_serializing_if = "Option::is_none")]
	    pub cycle_seconds: Option<u64>,
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct CheckmessageResponse {
	    #[serde(alias = "verified")]
	    pub verified: bool,
	    #[serde(alias = "pubkey", skip_serializing_if = "Option::is_none")]
	    pub pubkey: Option<String>,
	}

	/// Whether we successfully negotiated a mutual close, closed without them, or discarded not-yet-opened channel
	#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
	#[serde(rename_all = "lowercase")]
	pub enum CloseType {
	    MUTUAL,
	    UNILATERAL,
	    UNOPENED,
	}

	impl TryFrom<i32> for CloseType {
	    type Error = anyhow::Error;
	    fn try_from(c: i32) -> Result<CloseType, anyhow::Error> {
	        match c {
	    0 => Ok(CloseType::MUTUAL),
	    1 => Ok(CloseType::UNILATERAL),
	    2 => Ok(CloseType::UNOPENED),
	            o => Err(anyhow::anyhow!("Unknown variant {} for enum CloseType", o)),
	        }
	    }
	}
	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct CloseResponse {
	    // Path `Close.type`
	    #[serde(rename = "type")]
	    pub item_type: CloseType,
	    #[serde(alias = "tx", skip_serializing_if = "Option::is_none")]
	    pub tx: Option<String>,
	    #[serde(alias = "txid", skip_serializing_if = "Option::is_none")]
	    pub txid: Option<String>,
	}

}

