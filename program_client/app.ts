import {
    PublicKey,
    Keypair,
    Connection,
    TransactionInstruction,
} from "@solana/web3.js";
import * as fs from "fs/promises";
import * as path from "path";
import * as os from "os";

// Import your generated program ID and methods
import { initializeClient, mintArtwork, purchaseArtwork } from "./index";

interface PurchaseArtworkArgs {
    buyerInfo: { publicKey: PublicKey };
    artworkAccountInfo: TransactionInstruction;
    sellerInfo: { publicKey: PublicKey };
    feePayerInfo: { publicKey: PublicKey };
    feePayer: PublicKey;
    artworkId: bigint;
}

async function main(feePayer: Keypair) {
    const args = process.argv.slice(2);
    const connection = new Connection("https://api.devnet.solana.com", {
        commitment: "confirmed",
    });
    const progId = new PublicKey(args[0]!);
    initializeClient(progId, connection);

    // Mint a new artwork
    const newArtworkAccount = await mintArtwork({
        feePayer: feePayer.publicKey,
        newArtworkSeedCreator: Keypair.generate().publicKey,
    });
    
    // Showcase the artwork
    console.log("Artwork Minted:", newArtworkAccount.programId.toBase58());
    console.log("Artwork Details:", newArtworkAccount.data);

    // Generate Keypair instances for buyer, seller, and fee payer
    const buyerKeypair = Keypair.generate();
    const sellerKeypair = Keypair.generate();
    const feePayerKeypair = Keypair.generate();

    // Provide the necessary parameters for purchaseArtwork method
    const buyerInfo =  { publicKey: buyerKeypair.publicKey };
    const artworkAccountInfo = newArtworkAccount;
    const sellerInfo =  { publicKey: sellerKeypair.publicKey };
    const feePayerInfo = { publicKey: feePayerKeypair.publicKey }; 
    const artworkId = BigInt(1); // Assume the ID of the minted artwork

    const purchaseArgs: PurchaseArtworkArgs = {
        buyerInfo,
        artworkAccountInfo,
        sellerInfo,
        feePayerInfo,
        feePayer: feePayer.publicKey, 
        artworkId
    };
    
    // Purchase the artwork
    await purchaseArtwork(purchaseArgs);

    console.log("Artwork Purchased!");

}

fs.readFile(path.join(os.homedir(), ".config/solana/id.json")).then((file) =>
    main(Keypair.fromSecretKey(new Uint8Array(JSON.parse(file.toString()))))
);


