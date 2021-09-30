
import { Account, Connection, PublicKey, SystemProgram, TransactionInstruction, Transaction, SYSVAR_RENT_PUBKEY } from '@solana/web3.js';
import { deserializePermission, serializeUpdatePermissionInstruction, splitBuffer } from "./serialization.mjs";
import { Command } from "commander";
import { base58_to_binary } from 'base58-js';

const signerAccount = new Account(new Uint8Array([64,26,82,89,7,207,32,204,43,235,63,151,123,16,233,79,100,116,87,112,223,34,117,14,87,189,199,51,187,200,57,83,229,235,248,218,204,175,70,229,70,166,99,88,218,103,183,188,103,198,119,82,180,62,43,126,179,239,125,84,136,36,196,109]));
const programID = new PublicKey("3UoGwdgL4NZBgmvTGoLxugh1TmGRcKvpFjz2aTwYW3wz");
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
 