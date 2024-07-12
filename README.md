# Capture the flag

Let's see who can get the highest score!

This repo contains 3 challenges:
1. [Bump](ctf-bump/README.md)
2. [Coinflip](ctf-coinflip/README.md)
3. [GasPass](ctf-gaspass/README.md)

All these challenges can be completed using nothing but Rust interactors and smart contracts, but of couse, you are free to use any technology you prefer.

## Setup

Make sure to have your own account, and use a `.pem` file in your interactor, then communicate to the team which one is your wallet.

For convenience, we are also providing an interactor stub. The challenge contracts are already deployed to devnet, you can find their addresses in the interactor stub config:

```
gateway = 'https://devnet-gateway.multiversx.com'
bump_address = "erd1qqqqqqqqqqqqqpgq8ry8k8vk70wwpsjyey7xk8f57qcmpadwa4sq5tcnf9"
coinflip_address = "erd1qqqqqqqqqqqqqpgqu7ym00xsszp2mf5dh3xz798rtqjz43jta4sqzpt82e"
gaspass_address = "erd1qqqqqqqqqqqqqpgqqx2l65rtymptmyzdaqyr9mmdpm5cgrxga4sqld6mrz"
```

Best of luck!
