#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

const KEY_BASELINE: u64 = 3_000_000;

#[multiversx_sc::contract]
pub trait CtfGaspass: bump_common::BumpCommon {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[endpoint]
    fn gaspass(&self) -> bool {
        let gas_left = self.blockchain().get_gas_left();
        let caller = self.blockchain().get_caller();
        let the_key = KEY_BASELINE + self.personal_key(&caller);
        let passed = gas_left == the_key;

        if passed {
            self.perform_bump(&caller);
        } else {
            self.bumps(&caller).clear();
        };
        self.gaspass_event(&caller, passed);
        passed
    }

    fn personal_key(&self, caller: &ManagedAddress) -> u64 {
        let bytes = caller.to_byte_array();
        bytes.iter().map(|&b| b as u64).sum()
    }

    #[event("gaspass")]
    fn gaspass_event(&self, #[indexed] caller: &ManagedAddress, #[indexed] passed: bool);
}
