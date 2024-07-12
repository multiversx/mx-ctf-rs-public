#![no_std]

use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait BumpCommon {
    #[view]
    #[storage_mapper("bumps")]
    fn bumps(&self, bumper: &ManagedAddress) -> SingleValueMapper<BigUint>;

    fn perform_bump(&self, bumper: &ManagedAddress) {
        self.bumps(bumper).update(|bumps| *bumps += 1u32);
    }

    #[endpoint(donateBumps)]
    fn donate_bumps(&self, receiver: ManagedAddress) {
        let caller = self.blockchain().get_caller();
        let caller_bumps = self.bumps(&caller).take();
        self.bumps(&receiver).update(|bumps| *bumps += caller_bumps);
    }
}
