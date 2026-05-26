# SplitBound Pay
Transparent household financial coordination and automated escrow utility settlements powered by Soroban.

## Problem & Solution
* **Problem:** Maya, a young professional sharing a three-bedroom apartment in Jakarta, faces constant stress and late utility penalties because her roommates fail to coordinate their individual shares of the electricity and internet bills before the strict monthly payment deadline.
* **Solution:** Maya creates a shared utility pool using a Soroban smart contract where roommates deposit their exact shares, tracking entries transparently on Stellar's low-fee ledger to automatically pay the utility provider once the full bill is collected.

## Timeline
* **Week 1:** Core Soroban contract state-logic development, assertion rules, and testing baseline implementation.
* **Week 2:** Frontend integration linking user payment states with Freighter browser wallet extensions and building user-facing dashboards.

## Stellar Features Used
* **Soroban Smart Contracts:** Governs roommate tracking mapping structures, block deadline limits, and coordinates programmatic milestone release escrows.
* **XLM/USDC Transfers:** Offers low-fee, near-instant payment splitting processing to make micro-deposits economically practical.
* **Trustlines:** Secures accurate, compliant handling of standard digital stable assets across roommate accounts.

## Vision and Purpose
To eliminate domestic financial anxiety, remove individual debt liability, and avoid utility late fees within shared households by replacing manual tracking with decentralized, trustless smart contracts.

## Prerequisites
* Rust toolchain version 1.70.0+
* Soroban CLI / Stellar CLI version 20.0.0+
* Target `wasm32-unknown-unknown` added to your Rust compilation environment

## How to Build
To compile the smart contract into an optimized WebAssembly (WASM) binary, run the following command in your terminal:
```bash
soroban contract build

## ACCOUNT
CB4TUKK3E4P2DLLBLRPQY7HABB3BSUVFMC6G472ZW6OCBRSOLCEIEG73
https://stellar.expert/explorer/testnet/contract/CB4TUKK3E4P2DLLBLRPQY7HABB3BSUVFMC6G472ZW6OCBRSOLCEIEG73


