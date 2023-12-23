// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

import {PublicKey} from "@solana/web3.js";

export type ArtworkSeeds = {
    creator: PublicKey, 
};

export const deriveArtworkPDA = (
    seeds: ArtworkSeeds,
    programId: PublicKey
): [PublicKey, number] => {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from("artwork"),
            seeds.creator.toBuffer(),
        ],
        programId,
    )
};

export const deriveCustomAccountInfoPDA = (programId: PublicKey): [PublicKey, number] => {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from("account_info"),
        ],
        programId,
    )
};

