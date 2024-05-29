# Casper Shorts Trading Competition

In preparation for the mainnet launch of Casper Shorts, we will hold a trading
competition on Casper Testnet.

We're not gonna lay! We want to test Casper Shorts in battle conditions before
the mainnet launch. We expect two groups of participants: __traders__ and
__hackers__. Traders will try to make money by trading, while hackers will try
to break the system with code. We ecourage both groups to participate. The code
is open source, so you can see how it works and try to break it.

## TL;DR

Quick facts about the competition:
- Start Date: `2024-07-01 00:00:00 UTC`.
- Duration: `30 days`.
- Network: `casper-test`
- Market Contract: `hash-0123...cdef`
- Price Pool: `$1000`
- Telegram: [Casper Shorts](https://t.me/casper_shorts)

## Competition Rules

1. Will run on the `casper-test` network.
2. Will start on `2024-06-01 00:00:00 UTC` and end on `2024-06-30 23:59:59 UTC`.
3. The price pool is `$1000`.
4. The competition is open and free to everyonee, even Casper Shorts developers.
5. Transfers of `stCSPR` between accounts will be disabled during the
   competition, to prevent collusion.
6. Anyone can use the `faucet` to get the initial `1M` of `stCSPR` tokens.
7. Competitors will be judged by the highest balance of `stCSPR` at the end of
   the competition.
8. Prices will be distributed to the top five participants:
   - 1st: `$500`
   - 2nd: `$250`
   - 3rd: `$150`
   - 4th: `$75`
   - 5th: `$25`
9. Bad behavior on Telegram channel will result in disqualification.
10. Organizers reserve the (God Mode) rights to update Competition Rules at
    anytime.

## Trading Bots

The best way to win the competition is to write a trading bot. We have prepared
an example of one in Rust. It can do tricks already! You can use it as a
starting point for more complex strategies.

## Faucet

Faucet is done via the `faucet()` method in the `WCSPR` contract. You can call
it using a trading bot, website or by hand.