// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

import type {Schema} from 'borsh';
import type {Decoded} from "./utils";
import {PublicKey} from "@solana/web3.js";
import { deserialize } from "borsh";

/// Represents a digital artwork as an NFT.
export interface Artwork {
  title: string;
  description: string;
  price: bigint;
  creator: PublicKey;
}

export const decodeArtwork = (decoded: Decoded): Artwork => ({
    title: decoded["title"] as string,
    description: decoded["description"] as string,
    price: decoded["price"] as bigint,
    creator: new PublicKey(decoded["creator"] as Uint8Array),
});

export const ArtworkSchema: Schema =  {
    struct: {
        title: "string",
        description: "string",
        price: "u64",
        creator: { array: { type: "u8", len: 32 } },
    }
};

/// Represents Solana account information.
export interface CustomAccountInfo {
  lamports: bigint;
  owner: PublicKey;
}

export const decodeCustomAccountInfo = (decoded: Decoded): CustomAccountInfo => ({
    lamports: decoded["lamports"] as bigint,
    owner: new PublicKey(decoded["owner"] as Uint8Array),
});

export const CustomAccountInfoSchema: Schema =  {
    struct: {
        lamports: "u64",
        owner: { array: { type: "u8", len: 32 } },
    }
};



