# CTF challenges:

## 1. Bump challenge

A very simple contract, where anybody can bump a counter, but only once per call. The challenge for each player is to get the counter as large as possible under their own name.

The transaction sender is irrelevant, the name should be sent as an argument to every transaction.

### Strategies

1. At first, we expect players to send some of these transactions via the wallet. The transactions are very easy to format, but there are only so many bumps one can do by hand.

2. Then, they should figure out to automate the sending of transactions. Many programming languages are possible, but I will recommend learning to wrk with Rust interactors, since they are the closest to the SC development experience.

3. Players wating to boost their number even more are expected to deploy another contract in the same shard and boost several times in a single transaction, using synchronous calls.

### Tested skills

This task tests the players' ability to launch as many transactions as possible. This game is also one of the easiest ways to incentives people to flood the network.

Advanced players will also need to write and deploy a simple smart contract. It remains to be seen if this is the winning strategy.


## 2. Coinflip

The coinflip contract flips a coin and returns a reward 50% of the time. The problem is that blockchains don't really offer true randomness, so this function can be hacked.


### Inspiration

EVM: https://github.com/0xJuancito/the-ethernaut-solutions?tab=readme-ov-file#03---coinflip


### Variants

There are several ways for us to implement this:
- bump style
- sending/consuming some tokens

I'd say the bump style is more straightforward and easier for us to maintain.

### Implementation

Same as bump, but a randomness condition is added. In case of win it bumps up, otherwise it bumps down.

### Strategies

Sheer luck and brute force will qualify, but won't win.

This time the only valid strategy is to deploy another contract in the same shard, the new (hacker) contract checks the randomness condition beforehand, and only calls the coinflip if it knows it is going to win.


### Tested skills

- understanding of blocks, rounds, block nonces, pseudo-randomness
- ability to write and deploy a simple contract
- ability to spam said contract with transactions

## 3. GasPass

A contract that will yield a reward only if the gas left at a certain point in its execution is an exact value.

Bump-style rewards are once again the easiest ones.

### Implementation

Same as bump, but a gas left condition is added. Players will need to send the exact amount of gas, so that the condition passes.

### Strategies

Players will need to find the right amount of gas. The best way to do so is to rewrite our contract to return the remaining gas at the exact point where the condition happens. They can then check it either using Mandos, or on-chain, by deploying it.

Sending a single transaction from a wallet is enough this time.

