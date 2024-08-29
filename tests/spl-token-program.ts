import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MyTokenProgram } from "../target/types/my_token_program";
import {
  TOKEN_PROGRAM_ID,
  MINT_SIZE,
  createAssociatedTokenAccountInstruction,
  getAssociatedTokenAddress,
  createInitializeMintInstruction,
} from "@solana/spl-token";
import { assert } from "chai";

// Utility function to get the provider
const getProvider = () => anchor.AnchorProvider.env();

// Utility function to create a new transaction
const createTransaction = () => new anchor.web3.Transaction();

describe("my-token-program", () => {
  anchor.setProvider(getProvider());
  console.log("Workspace Programs:", anchor.workspace);
  console.log("Program:", anchor.workspace.MyTokenProgram);

  // Retrieve the MyTokenProgram struct from our smart contract
  const program = anchor.workspace.MyTokenProgram as Program<MyTokenProgram>;

  // Generate a random keypair that will represent our token mint
  const mintKey: anchor.web3.Keypair = anchor.web3.Keypair.generate();

  // AssociatedTokenAccount for anchor's workspace wallet
  let associatedTokenAccount: anchor.web3.PublicKey | undefined;

  it("Mint a token", async () => {
    try {
      const provider = getProvider();
      const walletPublicKey = provider.wallet.publicKey;
      
      // Get the amount of SOL needed to pay rent for our Token Mint
      const lamports = await provider.connection.getMinimumBalanceForRentExemption(MINT_SIZE);

      // Get the ATA for a token and the account that we want to own the ATA
      associatedTokenAccount = await getAssociatedTokenAddress(mintKey.publicKey, walletPublicKey);

      // Create the mint transaction
      const mintTransaction = createTransaction().add(
        anchor.web3.SystemProgram.createAccount({
          fromPubkey: walletPublicKey,
          newAccountPubkey: mintKey.publicKey,
          space: MINT_SIZE,
          programId: TOKEN_PROGRAM_ID,
          lamports,
        }),
        createInitializeMintInstruction(mintKey.publicKey, 0, walletPublicKey, walletPublicKey),
        createAssociatedTokenAccountInstruction(walletPublicKey, associatedTokenAccount, walletPublicKey, mintKey.publicKey)
      );

      // Send and confirm the transaction
      await provider.sendAndConfirm(mintTransaction, [mintKey]);

      // Log mint details
      const mintInfo = await program.provider.connection.getParsedAccountInfo(mintKey.publicKey);
      console.log("Mint info:", mintInfo);

      // Execute our code to mint our token into our specified ATA
      await program.methods.mintToken().accounts({
        mint: mintKey.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        tokenAccount: associatedTokenAccount,
        authority: walletPublicKey,
      }).rpc();

      // Verify the minted token amount on the ATA for our anchor wallet
      const minted = (await program.provider.connection.getParsedAccountInfo(associatedTokenAccount)).value.data.parsed.info.tokenAmount.amount;
      assert.equal(minted, 10);

    } catch (error) {
      console.error("Error during minting:", error);
    }
  });

  it("Transfer token", async () => {
    try {
      const provider = getProvider();
      const walletPublicKey = provider.wallet.publicKey;
      
      // Generate a keypair for the wallet that will receive the token
      const toWallet = anchor.web3.Keypair.generate();

      // Get the ATA for a token on the receiver wallet
      const toATA = await getAssociatedTokenAddress(mintKey.publicKey, toWallet.publicKey);

      // Create the transfer transaction
      const transferTransaction = createTransaction().add(
        createAssociatedTokenAccountInstruction(walletPublicKey, toATA, toWallet.publicKey, mintKey.publicKey)
      );

      // Send and confirm the transaction
      await provider.sendAndConfirm(transferTransaction, []);

      // Execute our transfer smart contract
      await program.methods.transferToken().accounts({
        tokenProgram: TOKEN_PROGRAM_ID,
        from: associatedTokenAccount,
        fromAuthority: walletPublicKey,
        to: toATA,
      }).rpc();

      // Verify the transferred token amount on the ATA for the receiver wallet
      const minted = (await program.provider.connection.getParsedAccountInfo(associatedTokenAccount)).value.data.parsed.info.tokenAmount.amount;
      assert.equal(minted, 5);

    } catch (error) {
      console.error("Error during token transfer:", error);
    }
  });
});