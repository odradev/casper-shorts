# Casper Sorts - Perpetual Swap Smart Contract

## Introduction
Perpetual swap is a contract between two parties to bet on the price of an
asset. One party goes long and the other goes short.

## Oracle
The Market Contract is connected to the 3party oracle that provides current asset
price denominated in USD.

## Mechanics
There are two main components of the system:
- The Market contract that manages the funds.
- Two token contracts (Long and Short) that represents the long and short position.

When user deposits funds, all goes into the Market. User has to choose if he
wants to go long or short. The user receives the corresponding token that
represents the ownership of deposited funds. The Market keeps track of the total
amount of the asset for the long and short positions.

When the price changes the Market changes the amount of the asset for the long
and short positions. 

## Market Contract

The Market contract is initialized with:
- Long Token address,
- Short Token address,

The Market contract has the following functions:
- deposit_long(&self, amount: U256, price_data: OracleProof)
- deposit_short(&self, amount: U256, price_data: OracleProof)
- withdraw_long(&self, amount: U256, price_data: OracleProof)
- withdraw_short(&self, amount: U256, price_data: OracleProof)
- get_long_balance(&self, addr: Address) -> U256
- get_short_balance(&self, addr: Address) -> U256
- get_total_deposit(&self) -> U256
- get_partial_deposits(&self) -> (U256, U256)
- get_collected_fee(&self) -> U256
- withdraw_fee(&self, amount: U256)

## State Definition

The system is defined as:

$State = (P, L_{Long}, L_{Short}, T_{Long}, T_{Short})$

where:
- $P$ is the current price of the asset,
- $L_{Long}$ is the total amount of the asset for the long position,
- $L_{Short}$ is the total amount of the asset for the short position,
- $T_{Long}$ is the total supply for the long token,
- $T_{Short}$ is the total supply for the short token.

In addition:

- $D$ - liquidity deposit for the long or short position,
- $W$ - tokens amount to be exchanges for the asset.
- $L$ - the amount of the asset for the long or short position.
- $S$ - the total supply of the long or short token.


## Mechanics

Whenever new price $Price_{i+1}$ is received the system updates the $State$,
by rebalancing the liquidity between long and short positions.

If the new price is lower then the previous price, first the short position
is reduced and then the long position is increased with the same amount.
And vice versa for the price increase.

Then the system updates the total supply of the long or short token.

## Price goes up

Short position is losing liquidity and it is transferred to the long position.
The formula is intended to cap the adjustment to the total available liquidity,
using the `min` function. 

$\Delta L_{Short}(i + 1) = L_{Short}(i) \times min(1, \frac{P(i+1)}{P(i)} - 1)$

$L_{Short}(i+1) = L_{Short}(i) - \Delta L_{Short}$

$L_{Long}(i+1) = L_{Long}(i) + \Delta L_{Short}$

## Price goes down

Long position is losing liquidity and it is transferred to the short position.

$\Delta L_{Long}(i + 1) = L_{Long}(i) \times min(1, 1 - \frac{P(i)}{P(i+1)})$

$L_{Short}(i+1) = L_{Short}(i) + \Delta L_{Long}$

$L_{Long}(i+1) = L_{Long}(i) - \Delta L_{Long}$

## Token deposit

When user deposits funds $D$ into the short or long position, the system
calculates the new amount of the position and the new total supply of the long
or short token.

$L(i+1) = L(i) + D$

$\Delta T(i+1) = T(i) \times (\frac{L(i+1)}{L(i)} - 1)$

$T(i+1) = T(i) + \Delta S$

## Token withdraw

When user withdraws $W$ of $LONG$ or $SHORT$ tokens, the system calculates the
new amount of the position and the new total supply of the token.

$T(i+1) = T(i) - W$

$\Delta L(i+1) = L(i) \times (1 - \frac{T(i)}{T(i+1)})$

$L(i+1) = L(i) - \Delta L$

## Example 1: Price goes up by more then 100%

Given:

$P(i) = 0.01 \text{USD/CSPR}$

$P(i+1) = 0.03 \text{ USD/CSPR}$

$L_{Short}(i) = 100 \text { CSPR}$

$L_{Long}(i) = 200 \text { CSPR}$

Then:

$\Delta L_{Short}(i+1) = 100 \times min(1, \frac{0.03}{0.01} - 1) = 100 \times 1 = 100 \text{ CSPR}$ 

$L_{Short}(i+1) = 100 - 100 = 0 \text{ CSPR}$

$L_{Long}(i+1) = 200 + 100 = 300 \text{ CSPR}$

## Example 2: Price goes up by less then 100%

Given:

$P(i) = 0.01 \text{USD/CSPR}$

$P(i+1) = 0.014 \text{ USD/CSPR}$

$L_{Short}(i) = 100 \text { CSPR}$

$L_{Long}(i) = 200 \text { CSPR}$

Then:

$\Delta L_{Short}(i+1) = 100 \times min(1, \frac{0.014}{0.01} - 1) = 100 \times 0.4 = 40 \text { CSPR}$

$L_{Short}(i+1) = 100 - 40 = 60 \text { CSPR}$

$L_{Long}(i+1) = 200 + 40 = 240 \text { CSPR}$

## Example 3: Price goes down

Given:

$P(i) = 0.02 \text{USD/CSPR}$

$P(i+1) = 0.015 \text{ USD/CSPR}$

$L_{Short}(i) = 100 \text { CSPR}$

$L_{Long}(i) = 200 \text { CSPR}$

Then:

$\Delta L_{Long}(i+1) = 200 \times min(1, 1 - \frac{0.015}{0.02}) = 200 \times 0.25 = 50 \text{ CSPR}$ 

$L_{Short}(i+1) = 100 + 50 = 150 \text{ CSPR}$

$L_{Long}(i+1) = 200 - 50 = 150 \text{ CSPR}$

## Example 4: Liquidity deposit

Given:

$L(i) = 200 \text { CSPR}$

$D = 100 \text { CSPR}$

$T(i) = 1000 \text{ LONG}$

Then:

$L(i+1) = 200 + 100 = 300 \text{ CSPR}$

$\Delta T(i+1) = 1000 \times (\frac{300}{200} - 1) = 1000 \times 0.5 = 500 \text{ LONG}$

$T_{Long}(i+1) = 1000 + 500 = 1500 \text{ LONG}$

## Example 5: Liquidity withdraw

Given:

$L(i) = 400 \text { CSPR}$

$W = 100 \text { LONG}$

$T(i) = 1000 \text{ LONG}$

Then:

$T(i+1) = 1000 - 100 = 900 \text{ LONG}$

$\Delta L(i+1) = 400 \times (1 - \frac{900}{1000}) = 400 \times 0.1 = 40 \text{ CSPR}$

$L(i+1) = 400 - 40 = 360 \text{ CSPR}$
