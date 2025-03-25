mod balances;
mod system;
mod support;
mod proof_of_existence;

use crate::support::Dispatch;

mod types {
	pub type AccountId = String;
	pub type Balance = u128;
    pub type Nonce = u32;
    pub type BlockNumber = u32;
    pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
	pub type Header = crate::support::Header<BlockNumber>;
	pub type Block = crate::support::Block<Header, Extrinsic>;
	pub type Content = &'static str;
}


#[derive(Debug)]
#[macros::runtime]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
	proof_of_existence: proof_of_existence::Pallet<Self>,
}

impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}

impl proof_of_existence::Config for Runtime {
	type Content = types::Content;
}

fn main() {
	let mut runtime = Runtime::new();
    
    runtime.balances.set_balance(&"alice".to_string(), 100);

	let block_1 = types::Block {
		header: types::Header { block_number: 1 },
		extrinsics: vec![
			support::Extrinsic {
				caller: "alice".to_string(),
				call: RuntimeCall::balances(balances::Call::transfer { to: "bob".to_string(), amount: 30 }),
			},
			support::Extrinsic {
				caller: "alice".to_string(),
				call: RuntimeCall::balances(balances::Call::transfer { to: "charlie".to_string(), amount: 20 }),
			},
		]
	};

	let block_2 = types::Block {
		header: types::Header { block_number: 2 },
		extrinsics: vec![
			support::Extrinsic {
				caller: "bob".to_string(),
				call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim { claim: "Hello, world!" }),
			},
			support::Extrinsic {
				caller: "charlie".to_string(),
				call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim { claim: "Hello, world!" }),
			},
		]
	};

	let block_3 = types::Block {
		header: types::Header { block_number: 3 },
		extrinsics: vec![
			support::Extrinsic {
				caller: "bob".to_string(),
				call: RuntimeCall::proof_of_existence(proof_of_existence::Call::revoke_claim { claim: "Hello, world!" }),
			},
			support::Extrinsic {
				caller: "bob".to_string(),
				call: RuntimeCall::proof_of_existence(proof_of_existence::Call::revoke_claim {
					claim: "Hello, world!",
				}),
			},
		]
	};

    runtime.execute_block(block_1).expect("invalid block");
	runtime.execute_block(block_2).expect("invalid block");
	runtime.execute_block(block_3).expect("invalid block");

    println!("{:#?}", runtime);

}