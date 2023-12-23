// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

import * as pda from "./pda";
import * as T from "./types";
import {
    Commitment,
    Connection,
    GetAccountInfoConfig,
    Keypair,
    PublicKey,
    sendAndConfirmTransaction,
    SystemProgram,
    Transaction,
    TransactionInstruction,
    TransactionSignature,
} from "@solana/web3.js";
import {deserialize, serialize} from "borsh";


let _programId: PublicKey;
let _connection: Connection;

export const initializeClient = (
    programId: PublicKey,
    connection: Connection
) => {
    _programId = programId;
    _connection = connection;
};

export enum DigitalArtMarketplaceInstruction {
/**
 * Mint a new artwork as an NFT.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 * 1. `[writable]` new_artwork: {@link Artwork} 
 * 2. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - new_artwork_seed_creator: {@link PublicKey} Auto-generated, from input new_artwork of type [Artwork] set the seed named creator, required by the type
 */
    MintArtwork = 0,

/**
 * Purchase an artwork and transfer ownership.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 * 1. `[]` buyer_info: {@link CustomAccountInfo} 
 * 2. `[]` artwork_account_info: {@link CustomAccountInfo} 
 * 3. `[]` seller_info: {@link CustomAccountInfo} 
 * 4. `[]` fee_payer_info: {@link CustomAccountInfo} 
 *
 * Data:
 * - artwork_id: {@link BigInt} The ID of the artwork to be purchased.
 */
    PurchaseArtwork = 1,
}

export type MintArtworkArgs = {
    feePayer: PublicKey;
    newArtworkSeedCreator: PublicKey;
};


/**
 * ### Returns a {@link TransactionInstruction}
 * Mint a new artwork as an NFT.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 * 1. `[writable]` new_artwork: {@link Artwork} 
 * 2. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - new_artwork_seed_creator: {@link PublicKey} Auto-generated, from input new_artwork of type [Artwork] set the seed named creator, required by the type
 */
export const mintArtwork = (args: MintArtworkArgs): TransactionInstruction => {
    const data = serialize(
        {
            struct: {
                id: "u8",
                new_artwork_seed_creator: { array: { type: "u8", len: 32 } },
            },
        },
        {
            id: DigitalArtMarketplaceInstruction.MintArtwork,
            new_artwork_seed_creator: args.newArtworkSeedCreator.toBytes(),
        }
    );

    const [newArtworkPubkey] = pda.deriveArtworkPDA({
        creator: args.newArtworkSeedCreator,
    }, _programId);

    return new TransactionInstruction({
        data: Buffer.from(data),
        keys: [
            {pubkey: args.feePayer, isSigner: true, isWritable: true},
            {pubkey: newArtworkPubkey, isSigner: false, isWritable: true},
            {pubkey: new PublicKey("11111111111111111111111111111111"), isSigner: false, isWritable: false},
        ],
        programId: _programId,
    });
};

/**
 * ### Returns a {@link TransactionSignature}
 * Mint a new artwork as an NFT.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 * 1. `[writable]` new_artwork: {@link Artwork} 
 * 2. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - new_artwork_seed_creator: {@link PublicKey} Auto-generated, from input new_artwork of type [Artwork] set the seed named creator, required by the type
 */
export const mintArtworkSendAndConfirm = async (
    args: Omit<MintArtworkArgs, "feePayer"> & { 
        signers: { feePayer: Keypair, }
 }
): Promise<TransactionSignature> => {
    const trx = new Transaction();


    trx.add(mintArtwork({
        ...args,
        feePayer: args.signers.feePayer.publicKey,
    }));

    return await sendAndConfirmTransaction(
        _connection,
        trx,
        [args.signers.feePayer, ]
    );
};

export type PurchaseArtworkArgs = {
    feePayer: PublicKey;
    artworkId: bigint;
};


/**
 * ### Returns a {@link TransactionInstruction}
 * Purchase an artwork and transfer ownership.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 * 1. `[]` buyer_info: {@link CustomAccountInfo} 
 * 2. `[]` artwork_account_info: {@link CustomAccountInfo} 
 * 3. `[]` seller_info: {@link CustomAccountInfo} 
 * 4. `[]` fee_payer_info: {@link CustomAccountInfo} 
 *
 * Data:
 * - artwork_id: {@link BigInt} The ID of the artwork to be purchased.
 */
export const purchaseArtwork = (args: PurchaseArtworkArgs): TransactionInstruction => {
    const data = serialize(
        {
            struct: {
                id: "u8",
                artwork_id: "u64",
            },
        },
        {
            id: DigitalArtMarketplaceInstruction.PurchaseArtwork,
            artwork_id: args.artworkId,
        }
    );

    const [buyerInfoPubkey] = pda.deriveCustomAccountInfoPDA(_programId);
    const [artworkAccountInfoPubkey] = pda.deriveCustomAccountInfoPDA(_programId);
    const [sellerInfoPubkey] = pda.deriveCustomAccountInfoPDA(_programId);
    const [feePayerInfoPubkey] = pda.deriveCustomAccountInfoPDA(_programId);

    return new TransactionInstruction({
        data: Buffer.from(data),
        keys: [
            {pubkey: args.feePayer, isSigner: true, isWritable: true},
            {pubkey: buyerInfoPubkey, isSigner: false, isWritable: false},
            {pubkey: artworkAccountInfoPubkey, isSigner: false, isWritable: false},
            {pubkey: sellerInfoPubkey, isSigner: false, isWritable: false},
            {pubkey: feePayerInfoPubkey, isSigner: false, isWritable: false},
        ],
        programId: _programId,
    });
};

/**
 * ### Returns a {@link TransactionSignature}
 * Purchase an artwork and transfer ownership.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 * 1. `[]` buyer_info: {@link CustomAccountInfo} 
 * 2. `[]` artwork_account_info: {@link CustomAccountInfo} 
 * 3. `[]` seller_info: {@link CustomAccountInfo} 
 * 4. `[]` fee_payer_info: {@link CustomAccountInfo} 
 *
 * Data:
 * - artwork_id: {@link BigInt} The ID of the artwork to be purchased.
 */
export const purchaseArtworkSendAndConfirm = async (
    args: Omit<PurchaseArtworkArgs, "feePayer"> & { 
        signers: { feePayer: Keypair, }
 }
): Promise<TransactionSignature> => {
    const trx = new Transaction();


    trx.add(purchaseArtwork({
        ...args,
        feePayer: args.signers.feePayer.publicKey,
    }));

    return await sendAndConfirmTransaction(
        _connection,
        trx,
        [args.signers.feePayer, ]
    );
};

// Getters

export const getArtwork = async (
    publicKey: PublicKey,
    commitmentOrConfig: Commitment | GetAccountInfoConfig | undefined = "processed"
): Promise<T.Artwork | undefined> => {
    const buffer = await _connection.getAccountInfo(publicKey, commitmentOrConfig);

    if (!buffer) {
        return undefined
    }

    if (buffer.data.length <= 0) {
        return undefined
    }

    return T.decodeArtwork(deserialize(T.ArtworkSchema, buffer.data) as Record<string, unknown>);
}

export const getCustomAccountInfo = async (
    publicKey: PublicKey,
    commitmentOrConfig: Commitment | GetAccountInfoConfig | undefined = "processed"
): Promise<T.CustomAccountInfo | undefined> => {
    const buffer = await _connection.getAccountInfo(publicKey, commitmentOrConfig);

    if (!buffer) {
        return undefined
    }

    if (buffer.data.length <= 0) {
        return undefined
    }

    return T.decodeCustomAccountInfo(deserialize(T.CustomAccountInfoSchema, buffer.data) as Record<string, unknown>);
}


// Websocket Events

