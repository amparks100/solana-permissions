
import { Account, Connection, PublicKey, SystemProgram, TransactionInstruction, Transaction, SYSVAR_RENT_PUBKEY } from '@solana/web3.js';
import { deserializePermission, serializeUpdatePermissionInstruction, splitBuffer } from "./serialization.mjs";
import { Command } from "commander";
import { base58_to_binary } from 'base58-js';

const signerAccount = new Account(new Uint8Array([64,26,82,89,7,207,32,204,43,235,63,151,123,16,233,79,100,116,87,112,223,34,117,14,87,189,199,51,187,200,57,83,229,235,248,218,204,175,70,229,70,166,99,88,218,103,183,188,103,198,119,82,180,62,43,126,179,239,125,84,136,36,196,109]));
const programID = new PublicKey("FUCdr1YJouPsXEET44N8L3YQtZ2fYBxR16K7ENkWFdTH");
const connection = new Connection("http://localhost:8899", 'singleGossip');

const size = (33*8)+1;

/**
 * Init Permission
 * Initialize a new Permission with a program state account.
 * Outputs the Permission address which identified the Permission instance for future calls.
 */
const initPermission = async () => {
  const permissionAccount = new Account();
  const createProgramAccountIx = SystemProgram.createAccount({
    space: size,
    lamports: await connection.getMinimumBalanceForRentExemption(size, 'singleGossip'),
    fromPubkey: signerAccount.publicKey,
    newAccountPubkey: permissionAccount.publicKey,
    programId: programID
  });

  const initIx = new TransactionInstruction({
    programId: programID,
    keys: [
        { pubkey: signerAccount.publicKey, isSigner: true, isWritable: false },
        { pubkey: permissionAccount.publicKey, isSigner: false, isWritable: true },
        { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},
    ],
    data: Buffer.from([0]),
  })
  
  const tx = new Transaction().add(createProgramAccountIx, initIx);
  let str = await connection.sendTransaction(tx, [signerAccount, permissionAccount], {skipPreflight: false, preflightCommitment: 'singleGossip'});

  console.log("Permission Address:", permissionAccount.publicKey.toBase58());
  process.exit(0)
}

/**
 * Update Permission
 * @param address address of permission program state
 * @param updateAccountAdress address of permission program state
 * @param role u32 value of role
 */
const setValue = async (address, updateAccountAdress, role) => {
  console.log("hi")


  const keys = [
    { pubkey: signerAccount.publicKey, isSigner: true, isWritable: false },
    { pubkey: address, isSigner: false, isWritable: true },
    { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},
    { pubkey: updateAccountAdress, isSigner: false, isWritable: false },
  ]

  const setIx = new TransactionInstruction({
    programId: programID,
    keys,
    data: serializeUpdatePermissionInstruction(role),
  })
  console.log("Instruction", setIx)

  const tx = new Transaction().add(setIx);
  console.log("Sending Transaction") 
  const txSignature = await connection.sendTransaction(
      tx, 
      [signerAccount], 
      {skipPreflight: false, preflightCommitment: 'singleGossip'});
  
  console.log("Transaction:", txSignature)  

  await connection.confirmTransaction(txSignature)
  const sendResult = await connection.getTransaction(txSignature, {commitment: 'confirmed'})

  console.log("SendResult ", sendResult)

  // for await (let nodeKey of fullPath) await dumpNode(connection, nodeKey);

  process.exit(0);
}

/**
 * Get Permissions for signer account
 * Outputs value on stdout if found. Otherwis exits with code 1.
 * @param address address of signer account
 * @param accountKey address of account to check permissions
 */
const getPermissions = async (address, accountKey) => {
  const stateKey = new PublicKey(address);

  const programState = (await connection.getAccountInfo(stateKey, 'singleGossip')).data;
  //console.log("state length", programState.length);
  const split = splitBuffer(programState)
  const decodedState = deserializePermission( split.prefix, split.array );
  //console.log("decoded state", decodedState);

  const accountBuffer = base58_to_binary(accountKey);
  //console.log("bufffer", accountBuffer);

  for( const account of decodedState.roles ) {
    //console.log("account", account.key);
    if(account.key.toString() === accountBuffer.toString()){
      console.log("Role ", account.role);
    }
  }
}

// /**
//  * Internal functions
//  */
// const _setValue = async (hamt, key, value) => {
//   const result = await lookup(connection, hamt, key);

//   const baseKeys = [
//     { pubkey: signerAccount.publicKey, isSigner: true, isWritable: false },
//     { pubkey: hamt, isSigner: false, isWritable: false },
//     { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},
//   ]

//   const nodeKeys = result.path.map(pubkey => ({ pubkey: pubkey, isSigner: false, isWritable: false }))
//   nodeKeys[nodeKeys.length - 1].isWritable = true

//   const nodeRent = await connection.getMinimumBalanceForRentExemption(HAMTNodeSize, 'singleGossip');
//   const collisionAccounts = Array(result.collisions).fill().map(()=>new Account());
//   const collisionInstructions = collisionAccounts.map((acc)=>SystemProgram.createAccount({
//     space: HAMTNodeSize,
//     lamports: nodeRent,
//     fromPubkey: signerAccount.publicKey,
//     newAccountPubkey: acc.publicKey,
//     programId: programID
//   }));
//   const collisionKeys = collisionAccounts.map(acc=>({ pubkey: acc.publicKey, isSigner: false, isWritable: true }))

//   result.rent = nodeRent * result.collisions;
//   result.collisionAccounts = collisionAccounts;

//   const valueBN = BigInt(value)
//   const setIx = new TransactionInstruction({
//     programId: programID,
//     keys: [...baseKeys, ...nodeKeys, ...collisionKeys],
//     data: serializeSetValueInstruction(key, valueBN),
//   })

//   const tx = new Transaction().add(...collisionInstructions, setIx);
//   result.txSignature = await connection.sendTransaction(
//       tx, 
//       [signerAccount, ...collisionAccounts], 
//       {skipPreflight: false, preflightCommitment: 'singleGossip'});
//   return result
// }

// const _setValueForBench = async (hamt, key, value) => {
//   const computeRegexp = /consumed (\d+) of \d+ compute units/
//   try {
//     const start = Date.now();
//     const result =  await _setValue(hamt, key, value)
//     const confirm = await connection.confirmTransaction(result.txSignature)
//     result.millis = Date.now() - start;

//     if (confirm.value.err) throw err

//     const txData = await connection.getTransaction(result.txSignature, {commitment: 'confirmed'})
//     result.fee =  txData.meta.fee
//     result.compute = parseInt(txData.meta.logMessages
//       .filter(l=>l.match(computeRegexp))[0]
//       .match(computeRegexp)[1])
    
//     return result
//   } catch (error) {
//     console.log(`Error setting ${key}`)
//     return { error, key, value }
//   }
// }

/**
 * Command CLI
 */
 const program = new Command();
 program
   .command('init')
   .description('create a new Permission state account')
   .action(initPermission);
 program
   .command('set <stateAccount> <permissionAccount> <role>')
   .description('updates the permission and role for account')
   .action(setValue);
 program
   .command('get <stateAccount> <accountKey>')
   .description('retrieves permission for the account')
   .action(getPermissions);
 program.parse(process.argv)
 