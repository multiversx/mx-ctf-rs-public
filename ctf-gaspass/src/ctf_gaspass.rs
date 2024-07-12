#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

#[multiversx_sc::contract]
pub trait CtfGaspass: bump_common::BumpCommon {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[endpoint]
    fn gaspass(&self) -> bool {
        let gas_left = self.blockchain().get_gas_left();
        let passed = gas_left == 500_000;
        let caller = self.blockchain().get_caller();
        if passed {
            self.perform_bump(&caller);
        } else {
            let caller = self.blockchain().get_caller();
            self.bumps(&caller).clear();
        };
        self.gaspass_event(&caller, passed);
        passed
    }

    #[event("gaspass")]
    fn gaspass_event(&self, #[indexed] caller: &ManagedAddress, #[indexed] passed: bool);
}
