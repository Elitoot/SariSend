# SariSend

Micro-credit repayment tracker for sari-sari stores using Stellar USDC.

## Problem

Sari-sari stores in the Philippines lose cash flow because customer debts are tracked manually and payments are delayed or forgotten.

## Solution

Customers repay debt using Stellar USDC while a Soroban smart contract automatically updates balances and repayment status instantly.

## Timeline

- Week 1: Smart contract development
- Week 2: Mobile frontend integration
- Week 3: Wallet + QR payment testing
- Week 4: Testnet deployment demo

## Stellar Features Used

- USDC payments
- Soroban smart contracts
- Trustlines

## Vision and Purpose

SariSend helps informal neighborhood businesses digitize community credit systems with transparent, low-cost stablecoin payments.

## Prerequisites

- Rust latest stable
- Soroban CLI v22+
- Stellar testnet account

## Build

```bash
soroban contract build