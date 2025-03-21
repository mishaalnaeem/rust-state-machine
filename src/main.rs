mod balances;
mod system;
mod support;

use crate::support::DispatchResult;
use crate::support::Dispatch;


mod types {
	pub type AccountId = String;
	pub type Balance = u128;
    pub type Nonce = u32;
    pub type BlockNumber = u32;
    pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
	pub type Header = crate::support::Header<BlockNumber>;
	pub type Block = crate::support::Block<Header, Extrinsic>;
}

pub enum RuntimeCall {
	Balances(balances::Call<Runtime>),
}

#[derive(Debug)]
pub struct Runtime {
	balances: balances::Pallet<Self>,
	system: system::Pallet<Self>,
}

impl Runtime {
    fn new() -> Self {
        Self {
            balances: balances::Pallet::new(),
            system: system::Pallet::new(),
        }
    }

	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
		self.system.inc_block_number();
		if block.header.block_number != self.system.block_number() {
			return Err("block number does not match what is expected");
		}
		
		
		for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
			self.system.inc_nonce(&caller);
			let _res = self.dispatch(caller, call).map_err(|e| {
				eprintln!(
					"Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
					block.header.block_number, i, e
				)
			});
		}
		Ok(())
	}
}

impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}

impl crate::support::Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountId;
	type Call = RuntimeCall;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
        match call {
			RuntimeCall::Balances(call) => {
				self.balances.dispatch(caller, call)
			}
		}
    }
}

fn main() {
	let mut runtime = Runtime::new();
    
    runtime.balances.set_balance(&"alice".to_string(), 100);

	let block_1 = types::Block {
		header: types::Header { block_number: 1 },
		extrinsics: vec![
			support::Extrinsic {
				caller: "alice".to_string(),
				call: RuntimeCall::Balances(balances::Call::Transfer { to: "bob".to_string(), amount: 30 }),
			},
			support::Extrinsic {
				caller: "alice".to_string(),
				call: RuntimeCall::Balances(balances::Call::Transfer { to: "charlie".to_string(), amount: 20 }),
			},
		]
	};

    runtime.execute_block(block_1).expect("invalid block");

    println!("{:#?}", runtime);

}