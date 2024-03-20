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

  const admin = Keypair.fromSecretKey(
    bs58.decode(process.env.PRIVATE_KEY_SOLANA_1)
  );

  // get config of campaign
  const systemConfig = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("system")],
    programID
  )[0];
  console.log("systemConfig: ", systemConfig);

  const newFee = new anchor.BN(0.01 * LAMPORTS_PER_SOL);

  const txHash = await program.methods
    .setPlatformFee(newFee)
    .accounts({
      admin: admin.publicKey,
      configAccount: systemConfig,
      systemProgram: SYSTEM_PROGRAM_ID,
    })
    .signers([admin])
    .rpc();

  console.log(`https://explorer.solana.com/tx/${txHash}?cluster=devnet`);

  const configData = await program.account.systemConfig.fetch(systemConfig);
  console.log("configData: ", configData);
};

main();
