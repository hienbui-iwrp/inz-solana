import * as anchor from "@coral-xyz/anchor";
import { InzMkp } from "../target/types/inz_mkp";
import idl from "../target/idl/inz_mkp.json";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  createApproveInstruction,
  getAssociatedTokenAddress,
} from "@solana/spl-token";
import {
  Connection,
  Keypair,
  PublicKey,
  clusterApiUrl,
  Transaction,
  sendAndConfirmRawTransaction,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import bs58 from "bs58";
import secp256k1 from "secp256k1";
import { keccak_256 } from "js-sha3";
require("dotenv").config();

const programID = new PublicKey(process.env.MPK_PROGRAM_ID);
const feeWallet = new PublicKey(process.env.FEE_WALLET);
const mint = new PublicKey("Fh7QVRCLHNxDxazredN5te2NJmGS1JoYsXAXbJfdba9v");

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
  const program = new anchor.Program(idl as InzMkp, programID, provider);

  console.log("programId: ", program.programId);

  const from = Keypair.fromSecretKey(
    bs58.decode(process.env.PRIVATE_KEY_SOLANA_2)
  );

  const to = Keypair.fromSecretKey(
    bs58.decode(process.env.PRIVATE_KEY_SOLANA_1)
  );

  console.log("mint: ", mint.toString());

  // get token account to hold nft
  const fromTokenAccount = await getAssociatedTokenAddress(
    mint,
    from.publicKey
  );
  console.log("fromTokenAccount: ", fromTokenAccount.toString());

  const toTokenAccount = await getAssociatedTokenAddress(mint, to.publicKey);
  console.log("toTokenAccount: ", toTokenAccount.toString());

  const tx = await program.methods
    .transferNft()
    .accounts({
      mint: mint,
      fromTokenAccount: fromTokenAccount,
      from: from.publicKey,
      toTokenAccount: toTokenAccount,
      to: to.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SYSTEM_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    })
    .signers([from])
    .rpc();
  console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet`);
};

main();
