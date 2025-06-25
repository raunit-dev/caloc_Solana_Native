import * as borsh from 'borsh';
import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction } from '@solana/web3.js';

class CounterAccount { // Class for the counter state account
    count = 0;

    constructor({count}: {count: number}) {
        this.count = count;
    }
}

const schema = { struct: { const: 'u32'}}; // Schema for the counter state account

const GREETING_SERIALIZED = borsh.serialize(
     schema,
     new CounterAccount({count: 0})
);
const GREETING_SIZE = GREETING_SERIALIZED.length;

let counterAccountKeypair: Keypair;
let adminKeypair: Keypair;

adminKeypair = Keypair.generate();//Generates a new random keypair using cryptographically secure randomness (via Ed25519).
   counterAccountKeypair = new Keypair(); // Creates an object Creates a Keypair object without properly initializing it with real keys.
   const connection = new Connection("https://api.devnet.solana.com", "confirmed");
   const res = await connection.requestAirdrop(adminKeypair.publicKey, 2 * LAMPORTS_PER_SOL);
   await connection.confirmTransaction(res);


test("counter does increase", async () => {
   

   const programId = new PublicKey("9fMw19JQGh63bioSwxujdLMgqDUiRzLuvTnFTaTdjWws");
   const lamports = await connection.getMinimumBalanceForRentExemption(GREETING_SIZE);

   const createCounterAccIx = SystemProgram.createAccount({
     fromPubkey: adminKeypair.publicKey,
     lamports,
     newAccountPubkey: counterAccountKeypair.publicKey,
     programId: programId,
     space: GREETING_SIZE,
   });

   const tx = new Transaction();
   tx.add(createCounterAccIx);

   const txHash = await connection.sendTransaction(tx,[adminKeypair, counterAccountKeypair]);
   await connection.confirmTransaction(txHash);

   const counterAccount = await connection.getAccountInfo(counterAccountKeypair.publicKey);
   if (!counterAccount) {
     throw new Error("Counter account not found");
   }
   const counter = borsh.deserialize(schema, counterAccount.data) as CounterAccount;
   console.log(counter.count);
   expect(counter.count).toBe(0);

})

