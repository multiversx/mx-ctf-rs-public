#![no_std]

use multiversx_sc::imports::*;

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait CtfCoinflip: bump_common::BumpCommon {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[endpoint]
    fn coinflip(&self) -> bool {
        let lucky = self.flip_coin();
        let caller = self.blockchain().get_caller();
        if lucky {
            self.perform_bump(&caller);
        } else {
            let caller = self.blockchain().get_caller();
            self.bumps(&caller).clear();
        };
        self.coinflip_event(&caller, lucky);
        lucky
    }

    fn flip_coin(&self) -> bool {
        let block_nonce = self.blockchain().get_block_nonce();
        block_nonce & 1 == block_nonce & 2
    }

    #[event("coinflip")]
    fn coinflip_event(&self, #[indexed] caller: &ManagedAddress, #[indexed] lucky: bool);
}
