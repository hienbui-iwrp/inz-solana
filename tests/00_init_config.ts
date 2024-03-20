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

  // select role
  const buyer = Keypair.fromSecretKey(
    bs58.decode(process.env.PRIVATE_KEY_SOLANA_2)
  );

  const caller = provider.wallet;

  console.log("wallet: ", wallet.publicKey);
  console.log("caller: ", caller.publicKey);
  console.log("buyer: ", buyer.publicKey);

  const admin = wallet;
  const feeWallet = new PublicKey(process.env.FEE_WALLET);
  const [systemConfig, _] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("system")],
    programID
  );

  console.log("systemConfig: ", systemConfig);

  const platformFee = new anchor.BN(0.05 * LAMPORTS_PER_SOL);

  const txHash = await program.methods
    .initConfig(platformFee)
    .accounts({
      admin: admin.publicKey,
      systemConfig: systemConfig,
      feeWallet: feeWallet,
      systemProgram: SYSTEM_PROGRAM_ID,
    })
    .signers([admin.payer])
    .rpc();

  console.log(`https://explorer.solana.com/tx/${txHash}?cluster=devnet`);

  const configData = await program.account.systemConfig.fetch(systemConfig);
  console.log("configData: ", configData);
};

main();
