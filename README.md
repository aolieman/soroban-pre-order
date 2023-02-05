# Soroban Pre-Order Contract

Are you selling a product or service that will take some time to produce or procure? Would you like to get paid through a blockchain, but don't want the hassle of processing refunds manually? Then this Soroban pre-order contract (SPOC) may be for you.

SPOC is in an early stage of development. It is ready to welcome external contributors but not yet ready for end-users. The initial contract development is expected to take several months, after which it will enter an experimental phase for some time to come. After all, [Soroban](https://soroban.stellar.org/) itself isn't production-ready yet.

## Motivation

The idea for SPOC originated with a need that [Stellarcarbon](https://www.stellarcarbon.io) expects to have in the future. Stellarcarbon buys carbon credits from carefully selected project developers, which can involve a lot of back and forth communication and can lead to relatively long procurement processes. When discussing the volume of credits to be purchased from a particular project, it would be very helpful to know how much demand there is for these credits.

Similar considerations apply to other tokenized real-world assets. For instance, someone making boutique hand-knitted sweaters—perhaps with a matching NFT—would like to make these to order. Others who tokenize real estate may want to know which kinds of properties and areas are most desirable for their audience of investors.

## Requirements

In its most basic form, a pre-order contract is an escrow between a buyer and a seller, to facilitate redemption/delivery of a product or service at a later date. This pre-order contract introduces several safeguards that are meant to protect both the buyer and the seller:

1. the seller can set up the contract for a single product or range of products
1. the buyer can independently verify with which parameters the contract has been initialized
1. the buyers payment is locked in the contract for a specified amount of time
1. the buyer's signature is required to redeem the purchase
1. after the time lock expires, the buyer can refund their own payment
1. the fulfillment queue is "first come, first served"
1. an order that is not refunded after the time lock expires maintains its position in the queue
1. the contract must allow for a perpetual sale, but may implement a maximum queue size
1. the seller can cancel and refund all orders for a given product type

These requirements are somewhat opinionated and are not a "neutral" representation of a pre-order process. I do, however, believe that these requirements are compatible with existing legal frameworks in many jurisdictions. The configurable time lock in particular can be used to make it easier for the buyer to get a refund, or to let the seller ask for a stronger commitment before the buyer places an order.

## Engineering Challenges

There is very little existing work to base SPOC on. Although the pre-order use case in various shapes and forms is common in traditional e-commerce, it seems to have little precedent in smart contracts.

Specifically for implementation in Soroban, I expect the most significant challenge to be the development of an efficient fulfillment queue. The current example contracts for Soroban, e.g. those submitted to Sorabanathon, operate with relatively simple and mostly flat data structures. Implementing a queue for use in Soroban is not straightforward because `CONTRACT_DATA` values are (de)serialized [in their entirety](https://soroban.stellar.org/docs/learn/persisting-data#granularity) when accessed. For this contract, they will need to be keyed "by buyer" for most functions. To enable the "first come, first served" mechanism efficiently, orders will likely also need to be represented in an index for each product type. This leads to the interesting challenge of keeping two on-chain data structures perfectly synchronized.

## Prior Art

I'm not aware of any existing Soroban contract that can be adapted to a pre-order sales process. The closest match is the crowdfund contract in the [Soroban Example dApp](https://github.com/stellar/soroban-example-dapp). A crowdfund is a very particular type of pre-order that has a target budget to reach, and a single deadline at which the offer expires. SPOC—in contrast—implements a queue-based sales process without a monetary target and with an individual time lock for each order that is placed.

There is a smart contract implemented in Solidity by [OtoCo](https://otoco.gitbook.io/otoco/untitled-1), a service that provides legal wrappers for DAOs. They have a [launchpool](https://github.com/otoco-io/OtoGo-SmartContracts/blob/main/contracts/launchpool.sol) contract that is meant to raise capital, and which models a pre-order token sale as a queue.

## Contributions

This project welcomes contributions under the GPLv3 license. The strong copyleft license should not get in the way of any commercial applications because of the modularity and transparency inherent in Soroban contracts. It is meant to ensure that the Stellar community can benefit from all improvements and adaptations.

A modified version of the SDF Code of Conduct will apply to this git repository and all spaces managed by the maintainers of this project.