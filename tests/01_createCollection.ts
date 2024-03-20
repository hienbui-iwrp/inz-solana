import * as anchor from "@coral-xyz/anchor";
import { InzCreator } from "../target/types/inz_creator";
import idl from "../target/idl/inz_creator.json";
import {
  Connection,
  Keypair,
  PublicKey,
  clusterApiUrl,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
} from "@solana/spl-token";
import {
  MPL_TOKEN_METADATA_PROGRAM_ID,
  findMasterEditionPda,
  findMetadataPda,
  mplTokenMetadata,
} from "@metaplex-foundation/mpl-token-metadata";
import { publicKey } from "@metaplex-foundation/umi";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import bs58 from "bs58";
require("dotenv").config();

const programID = new PublicKey(process.env.CREATOR_PROGRAM_ID);

const main = async () => {
  // LIST KEYPAIR
  const SYSTEM_PROGRAM_ID = new PublicKey("11111111111111111111111111111111");

  //   SET PROGRAM
  const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
  const wallet = new NodeWallet(
    Keypair.fromSecretKey(bs58.decode(process.env.PRIVATE_KEY_DEPLOYER))
  );

  const provider = new anchor.AnchorProvider(connection, wallet, {
    preflightCommitment: "recent",
    commitment: "processed",
  });
  const program = new anchor.Program(idl as InzCreator, programID, provider);
  console.log("programId: ", program.programId);

  const umi = createUmi(clusterApiUrl("devnet")).use(mplTokenMetadata());
  // select role
  const owner = Keypair.fromSecretKey(
    bs58.decode(process.env.PRIVATE_KEY_SOLANA_2)
  );

  console.log("owner: ", owner.publicKey);

  // generate address for nft
  const newMint = Keypair.generate();
  console.log("new Mint: ", newMint.publicKey.toString());

  // get token account to hold nft
  const tokenAccount = await getAssociatedTokenAddress(
    newMint.publicKey,
    owner.publicKey
  );
  console.log("tokenAccount: ", tokenAccount.toString());

  // get derive metadata account of nft collection
  let metadataAccount = findMetadataPda(umi, {
    mint: publicKey(newMint.publicKey),
  })[0];
  console.log("metadataAccount: ", metadataAccount);

  // get derive config account of nft collection
  let masterEditionAccount = findMasterEditionPda(umi, {
    mint: publicKey(newMint.publicKey),
  })[0];
  console.log("masterEditionAccount: ", masterEditionAccount);

  // get config of campaign
  const configAccount = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("config"), newMint.publicKey.toBuffer()],
    programID
  )[0];
  console.log("configAccount: ", configAccount);

  const name = "TICKET";
  const symbol = "TICKET";
  const uri =
    "https://static.innovaz.io/nft/metadata/65ae2eecbc73838f5feea43c/1.json";
  const nftTypes = [
    {
      id: 1,
      price: new anchor.BN(0.01 * LAMPORTS_PER_SOL),
      supply: new anchor.BN(30),
      minted: new anchor.BN(0),
    },
    {
      id: 2,
      price: new anchor.BN(0.02 * LAMPORTS_PER_SOL),
      supply: new anchor.BN(0),
      minted: new anchor.BN(0),
    },
  ];
  const callbackData = "My callback data";

  const txHash = await program.methods
    .createCollection(nftTypes, name, symbol, uri, callbackData)
    .accounts({
      mint: newMint.publicKey,
      tokenAccount: tokenAccount,
      owner: owner.publicKey,
      metadataAccount: metadataAccount,
      masterEditionAccount: masterEditionAccount,
      configAccount: configAccount,
      systemProgram: SYSTEM_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      tokenMetadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
    .signers([owner, newMint])
    .rpc();

  console.log(`https://explorer.solana.com/tx/${txHash}?cluster=devnet`);

  const configData = await program.account.collectionConfig.fetch(
    configAccount
  );
  console.log("configData: ", configData);
};

main();
