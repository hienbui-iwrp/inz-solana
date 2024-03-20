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

  const seller = new NodeWallet(
    Keypair.fromSecretKey(bs58.decode(process.env.PRIVATE_KEY_SOLANA_1))
  );

  const buyer = Keypair.fromSecretKey(
    bs58.decode(process.env.PRIVATE_KEY_SOLANA_2)
  );

  console.log("mint: ", mint.toString());

  // get token account to hold nft
  const fromTokenAccount = await getAssociatedTokenAddress(
    mint,
    seller.publicKey
  );
  console.log("fromTokenAccount: ", fromTokenAccount.toString());

  const toTokenAccount = await getAssociatedTokenAddress(mint, buyer.publicKey);
  console.log("toTokenAccount: ", toTokenAccount.toString());

  const systemConfig = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("system")],
    programID
  )[0];
  console.log("systemConfig: ", systemConfig);

  //   ---- listing -----
  const transaction = new Transaction().add(
    createApproveInstruction(
      fromTokenAccount,
      systemConfig,
      seller.publicKey,
      1
    )
  );
  const latestBlockhash = await connection.getLatestBlockhash();

  transaction.recentBlockhash = latestBlockhash.blockhash;
  transaction.feePayer = seller.publicKey;
  const signedTransaction = await seller.signTransaction(transaction);

  let rawTransaction = signedTransaction.serialize();

  const listingLog = await sendAndConfirmRawTransaction(
    connection,
    rawTransaction
  );
  console.log(
    "listing log: ",
    `https://explorer.solana.com/tx/${listingLog}?cluster=devnet`
  );

  // trade
  const price = 0.01 * LAMPORTS_PER_SOL;
  console.log("price: ", price);
  // from backend
  const signature = sign_address(mint, price);

  const tx = await program.methods
    .trade(new anchor.BN(price), signature.signature, signature.recoveryId)
    .accounts({
      mint: mint,
      fromTokenAccount: fromTokenAccount,
      seller: seller.publicKey,
      toTokenAccount: toTokenAccount,
      buyer: buyer.publicKey,
      systemConfig: systemConfig,
      feeWallet: feeWallet,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SYSTEM_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    })
    .signers([buyer])
    .rpc();
  console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet`);
};

const sign_address = (mint: PublicKey, price: number) => {
  const backend = Keypair.fromSecretKey(
    bs58.decode(process.env.PRIVATE_KEY_BACKEND)
  );
  const secp256k1PrivateKey = backend.secretKey.slice(0, 32);

  // Derive the public key
  let secp256k1PublicKey = secp256k1
    .publicKeyCreate(secp256k1PrivateKey, false)
    .slice(1);

  console.log("##### secp256k1PublicKey", secp256k1PublicKey);

  let message = Buffer.from(mint.toString() + price);

  let messageHash = Buffer.from(keccak_256.update(message).digest());
  let { signature, recid: recoveryId } = secp256k1.ecdsaSign(
    messageHash,
    secp256k1PrivateKey
  );

  console.log("##### signature\n", signature);
  console.log("##### recoveryId\n", recoveryId);
  return {
    signature: signature,
    recoveryId: recoveryId,
  };
};

main();
