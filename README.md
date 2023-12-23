# DIGITAL ARTWORK MARKETPLACE 

## Table of Contents

- [Overview](#overview)
  - [Key Features](#key-features)
  - [Why It's Unique](#why-its-unique)
  - [Functions](#functions)
  - [Screenshots](#screenshots)
- [Getting Started](#getting-started)
  - [Build and Deploy](#build-and-deploy)
  - [Frontend Testing](#frontend-testing)
- [Output](#Output)

## Overview
Welcome to Decentralized Digital Art Marketplace where artists can showcase and sell their digital artwork. Here, Rust is used for smart contract development on Solana, and TypeScript is used for the frontend.

#### Key Features:
* Artwork Minting:
   - Artists can mint their digital artwork as non-fungible tokens (NFTs) on the Solana blockchain.
   - Each artwork gets a unique token ID and is stored on the blockchain.
* Artwork Showcase:
   - Users can browse and view the digital artwork listed on the marketplace.
   - Artwork details include the artwork title, description, and price.
* Decentralized Transactions:
   - Users can purchase digital artwork directly using Solana's native cryptocurrency (SOL).
   - Smart contracts handle the transparent and secure transfer of ownership.

#### Why It's Unique:
- NFTs and Digital Art: The project leverages the rising trend of non-fungible tokens (NFTs) and allows artists to tokenize their digital artwork on the Solana blockchain, providing a decentralized platform for art trading.
- Decentralized Transactions: Using Solana for transactions ensures a decentralized and secure environment for buying and selling digital art, without relying on traditional payment processors.

### Functions
* mint_artwork : Mint a new network as NFT.
* purchase_artwork : Purchase a artwork and transfer ownership.

### Screenshots
<img src="https://github.com/Anuprita579/Digital_Artwork_MarketPlace/assets/141035951/ece8a486-8088-4a72-900b-3d31f97a7f79" alt="Screenshot 1" height="500px" width="300px">
<img src="https://github.com/Anuprita579/Digital_Artwork_MarketPlace/assets/141035951/0c87c87a-0946-4945-a408-aadcd5347b54" alt="Screenshot 2" height="500px" width="300px">
<img src="https://github.com/Anuprita579/Digital_Artwork_MarketPlace/assets/141035951/b86afd85-bd1e-4d94-8589-744fab09f20f" alt="Screenshot 3" height="500px" width="300px">
<img src="https://github.com/Anuprita579/Digital_Artwork_MarketPlace/assets/141035951/670a7592-0f46-4ffc-9fb6-f469e92f1625" alt="Screenshot 4" height="500px" width="300px">

## Getting Started
Follow these steps to set up the project locally.
PROGRAM ID : 5Kpd48j4CGrU3qHohyuQVFmb63rkhK1BvxV15Cti6AUw

### Build and Deploy
* Build the contract
  * Open a new terminal window.
  * Enter "cd program" command in the terminal.
  * Then paste the following code in the terminal.
```
cargo build-sbf
```
* Set up your config
  * Paste the following code in the terminal.
```
solana config set --url devnet
```
* Get dev tokens
  * To deploy a contract, you need tokens. Get tokens by pasting the following command in your terminal.
```
solana airdrop 1 
```
* You can check your balance by using the following command.
```
solana balance
```
* Deploy the Contract
  * Use the following command for deployment.
```
solana program deploy target/deploy/digital_art_marketplace.so
```

### Frontend Testing
* Open a new terminal window.
* Enter "cd program_client" command in the terminal.
* Then paste the following code in the terminal.
```
yarn install
```
* Run app.ts 
  * Paste the following code in the terminal.
```
npx ts-node app.ts 5Kpd48j4CGrU3qHohyuQVFmb63rkhK1BvxV15Cti6AUw
```

## OUTPUT
![Screenshot 2023-12-23 233422](https://github.com/Anuprita579/Digital_Artwork_MarketPlace/assets/141035951/506ae3dc-d321-4ec7-9383-8e577ffee0c5)
