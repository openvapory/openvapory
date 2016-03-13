// Copyright 2015, 2016 Ethcore (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

use std::collections::HashMap;
use std::sync::Arc;
use jsonrpc_core::IoHandler;
use util::hash::{Address, H256};
use util::numbers::U256;
use ethcore::client::{TestBlockChainClient, EachBlockWith};
use v1::{Eth, EthClient};
use v1::tests::helpers::{TestAccount, TestAccountProvider, TestSyncProvider, Config, TestMinerService};

fn blockchain_client() -> Arc<TestBlockChainClient> {
	let mut client = TestBlockChainClient::new();
	client.add_blocks(10, EachBlockWith::Nothing);
	client.set_balance(Address::from(1), U256::from(5));
	client.set_storage(Address::from(1), H256::from(4), H256::from(7));
	client.set_code(Address::from(1), vec![0xff, 0x21]);
	Arc::new(client)
}

fn accounts_provider() -> Arc<TestAccountProvider> {
	let mut accounts = HashMap::new();
	accounts.insert(Address::from(1), TestAccount::new("test"));
	let ap = TestAccountProvider::new(accounts);
	Arc::new(ap)
}

fn sync_provider() -> Arc<TestSyncProvider> {
	Arc::new(TestSyncProvider::new(Config {
		protocol_version: 65,
		num_peers: 120,
	}))
}

fn miner_service() -> Arc<TestMinerService> {
	Arc::new(TestMinerService)
}

struct EthTester {
	_client: Arc<TestBlockChainClient>,
	_sync: Arc<TestSyncProvider>,
	_accounts_provider: Arc<TestAccountProvider>,
	_miner: Arc<TestMinerService>,
	pub io: IoHandler,
}

impl Default for EthTester {
	fn default() -> Self {
		let client = blockchain_client();
		let sync = sync_provider();
		let ap = accounts_provider();
		let miner = miner_service();
		let eth = EthClient::new(&client, &sync, &ap, &miner).to_delegate();
		let io = IoHandler::new();
		io.add_delegate(eth);
		EthTester {
			_client: client,
			_sync: sync,
			_accounts_provider: ap,
			_miner: miner,
			io: io
		}
	}
}

#[test]
fn rpc_eth_protocol_version() {
	let request = r#"{"jsonrpc": "2.0", "method": "eth_protocolVersion", "params": [], "id": 1}"#;
	let response = r#"{"jsonrpc":"2.0","result":"65","id":1}"#;

	assert_eq!(EthTester::default().io.handle_request(request), Some(response.to_owned()));
}

#[test]
#[ignore]
fn rpc_eth_syncing() {
	unimplemented!()
}

#[test]
#[ignore]
fn rpc_eth_hashrate() {
	unimplemented!()
}

#[test]
#[ignore]
fn rpc_eth_author() {
	unimplemented!()
}

#[test]
#[ignore]
fn rpc_eth_mining() {
	unimplemented!()
}

#[test]
fn rpc_eth_gas_price() {
	let request = r#"{"jsonrpc": "2.0", "method": "eth_gasPrice", "params": [], "id": 1}"#;
	let response = r#"{"jsonrpc":"2.0","result":"0x0ba43b7400","id":1}"#;

	assert_eq!(EthTester::default().io.handle_request(request), Some(response.to_owned()));
}

#[test]
fn rpc_eth_accounts() {
	let request = r#"{"jsonrpc": "2.0", "method": "eth_accounts", "params": [], "id": 1}"#;
	let response = r#"{"jsonrpc":"2.0","result":["0x0000000000000000000000000000000000000001"],"id":1}"#;

	assert_eq!(EthTester::default().io.handle_request(request), Some(response.to_owned()));
}

#[test]
fn rpc_eth_block_number() {
	let request = r#"{"jsonrpc": "2.0", "method": "eth_blockNumber", "params": [], "id": 1}"#;
	let response = r#"{"jsonrpc":"2.0","result":"0x0a","id":1}"#;

	assert_eq!(EthTester::default().io.handle_request(request), Some(response.to_owned()));
}

#[test]
fn rpc_eth_balance() {
	let request = r#"{
		"jsonrpc": "2.0",
		"method": "eth_getBalance",
		"params": ["0x0000000000000000000000000000000000000001", "latest"],
		"id": 1
	}"#;
	let response = r#"{"jsonrpc":"2.0","result":"0x05","id":1}"#;

	assert_eq!(EthTester::default().io.handle_request(request), Some(response.to_owned()));
}

#[test]
fn rpc_eth_storage_at() {
	let request = r#"{
		"jsonrpc": "2.0",
		"method": "eth_getStorageAt",
		"params": ["0x0000000000000000000000000000000000000001", "0x4", "latest"],
		"id": 1
	}"#;
	let response = r#"{"jsonrpc":"2.0","result":"0x07","id":1}"#;

	assert_eq!(EthTester::default().io.handle_request(request), Some(response.to_owned()));
}

#[test]
fn rpc_eth_transaction_count() {
	let request = r#"{
		"jsonrpc": "2.0",
		"method": "eth_getTransactionCount",
		"params": ["0x0000000000000000000000000000000000000001", "latest"],
		"id": 1
	}"#;
	let response = r#"{"jsonrpc":"2.0","result":"0x00","id":1}"#;

	assert_eq!(EthTester::default().io.handle_request(request), Some(response.to_owned()));
}

#[test]
fn rpc_eth_block_transaction_count_by_hash() {
	let request = r#"{
		"jsonrpc": "2.0",
		"method": "eth_getBlockTransactionCountByHash",
		"params": ["0xb903239f8543d04b5dc1ba6579132b143087c68db1b2168786408fcbce568238"],
		"id": 1
	}"#;
	let response = r#"{"jsonrpc":"2.0","result":"0x00","id":1}"#;

	assert_eq!(EthTester::default().io.handle_request(request), Some(response.to_owned()));
}

#[test]
fn rpc_eth_transaction_count_by_number() {
	let request = r#"{
		"jsonrpc": "2.0",
		"method": "eth_getBlockTransactionCountByNumber",
		"params": ["latest"],
		"id": 1
	}"#;
	let response = r#"{"jsonrpc":"2.0","result":"0x00","id":1}"#;

	assert_eq!(EthTester::default().io.handle_request(request), Some(response.to_owned()));
}

#[test]
fn rpc_eth_uncle_count_by_block_hash() {
	let request = r#"{
		"jsonrpc": "2.0",
		"method": "eth_getUncleCountByBlockHash",
		"params": ["0xb903239f8543d04b5dc1ba6579132b143087c68db1b2168786408fcbce568238"],
		"id": 1
	}"#;
	let response = r#"{"jsonrpc":"2.0","result":"0x00","id":1}"#;

	assert_eq!(EthTester::default().io.handle_request(request), Some(response.to_owned()));
}

#[test]
fn rpc_eth_uncle_count_by_block_number() {
	let request = r#"{
		"jsonrpc": "2.0",
		"method": "eth_getUncleCountByBlockNumber",
		"params": ["latest"],
		"id": 1
	}"#;
	let response = r#"{"jsonrpc":"2.0","result":"0x00","id":1}"#;

	assert_eq!(EthTester::default().io.handle_request(request), Some(response.to_owned()));
}

#[test]
fn rpc_eth_code() {
	let request = r#"{
		"jsonrpc": "2.0",
		"method": "eth_getCode",
		"params": ["0x0000000000000000000000000000000000000001", "latest"],
		"id": 1
	}"#;
	let response = r#"{"jsonrpc":"2.0","result":"0xff21","id":1}"#;

	assert_eq!(EthTester::default().io.handle_request(request), Some(response.to_owned()));
}

#[test]
#[ignore]
fn rpc_eth_call() {
	unimplemented!()
}

#[test]
#[ignore]
fn rpc_eth_send_transaction() {
	unimplemented!()
}

#[test]
#[ignore]
fn rpc_eth_send_raw_transaction() {
	unimplemented!()
}

#[test]
#[ignore]
fn rpc_eth_sign() {
	unimplemented!()
}

#[test]
#[ignore]
fn rpc_eth_estimate_gas() {
	unimplemented!()
}

#[test]
fn rpc_eth_compilers() {
	let request = r#"{"jsonrpc": "2.0", "method": "eth_getCompilers", "params": [], "id": 1}"#;
	let response = r#"{"jsonrpc":"2.0","result":[],"id":1}"#;

	assert_eq!(EthTester::default().io.handle_request(request), Some(response.to_owned()));
}



