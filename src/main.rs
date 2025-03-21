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
			return Err("block number does not match what is expected")
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
        unimplemented!();
    }
}

fn main() {
	let mut runtime = Runtime::new();
    
    runtime.balances.set_balance(&"alice".to_string(), 100);

    runtime.system.inc_block_number();
    assert_eq!(runtime.system.block_number(), 1);

    runtime.system.inc_nonce(&"alice".to_string());
    let _res = runtime.balances.transfer(&"alice".to_string(), &"bob".to_string(), 30).map_err(|e| println!("{}", e));

    runtime.system.inc_nonce(&"alice".to_string());
    let _res = runtime.balances.transfer(&"alice".to_string(), &"charlie".to_string(), 20).map_err(|e| println!("{}", e));

    println!("{:#?}", runtime);

}