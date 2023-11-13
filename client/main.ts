import { Keypair, Connection, Transaction, TransactionInstruction, sendAndConfirmTransaction, PublicKey } from '@solana/web3.js';

async function main() {
    const pg = {
        PROGRAM_ID: new PublicKey('5yLCyQuWSBumGH86PXs83t56VxL2qUvR1B8orKdWcik3'), // Replace with your actual program ID
        connection: new Connection('http://localhost:8899', 'confirmed'), // Adjust connection details
        wallet: {
            keypair: Keypair.fromSecretKey(Uint8Array.from([87,209,249,233,118,124,231,30,35,200,50,10,255,42,235,167,234,168,102,255,204,14,231,29,82,195,202,85,115,187,133,74,164,9,36,231,50,180,254,123,86,88,28,226,203,10,167,32,59,88,149,79,194,179,57,115,50,92,41,198,243,169,216,82])),
        },
    };

    console.log(pg.PROGRAM_ID.toString());
// Create transaction
const tx = new web3.Transaction({
  ...blockhashInfo,
});
    const blockhashInfo = await pg.connection.getLatestBlockhash();
  
console.log(blockhashInfo);
    // Define the accounts
    const resultAccountAddress = new PublicKey('C3Kx5M6n9VsFLqp5fmPDLrstyQBDACL12bcqqbKDu4Ks'); 
    console.log(resultAccountAddress.toString());
    const programAccount = new PublicKey('E4E583qkFHGiF452z3Rd64MPWDKHkSEeGzZ153caM8yL'); 
    console.log(programAccount.toString());
    // Add the instruction to the transaction
    tx.add(
        new TransactionInstruction({
            keys: [
                { pubkey: resultAccountAddress.toString(), isSigner: false, isWritable: true },
            ],
            programId: pg.PROGRAM_ID,
            data: Buffer.from([1, 10, 01]), 
        })
    );

    // Sign the transaction
    tx.sign(pg.wallet.keypair);

    // Send the transaction to the Solana cluster
    const txHash = await sendAndConfirmTransaction(pg.connection, tx, [pg.wallet.keypair]);
    console.log('Transaction hash:', txHash);
}

main();
