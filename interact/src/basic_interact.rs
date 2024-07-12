mod basic_interact_config;

mod ctf_bump_proxy;
mod ctf_coinflip_proxy;
mod ctf_gaspass_proxy;

use basic_interact_config::Config;

use multiversx_sc_snippets::imports::*;

const INTERACTOR_SCENARIO_TRACE_PATH: &str = "interactor_trace.scen.json";

#[tokio::main]
async fn main() {
    env_logger::init();

    let _basic_interact = CtfBumpInteract::init().await;
}

#[allow(unused)]
struct CtfBumpInteract {
    interactor: Interactor,
    wallet_address: Bech32Address,
}

impl CtfBumpInteract {
    async fn init() -> Self {
        let config = Config::load_config();
        let mut interactor = Interactor::new(config.gateway())
            .await
            .with_tracer(INTERACTOR_SCENARIO_TRACE_PATH)
            .await;
        let wallet_address = interactor.register_wallet(test_wallets::mike());

        Self {
            interactor,
            wallet_address: wallet_address.into(),
        }
    }
}
