import { serialize, deserialize } from 'borsh';
import { PublicKey } from '@solana/web3.js';


/**
 * Borsh serializations
 */

class Assignable {
  constructor(properties) {
      Object.keys(properties).map((key) => {
          this[key] = properties[key];
      });
  }
}

/**
 * Update permission instruction
 */
const UpdatePermissionKind = 1;
export class UpdatePermissionInstruction extends Assignable { }
const updatePermissionSchema = new Map([[UpdatePermissionInstruction, { kind: 'struct', fields: [['kind', 'u8'],['role', 'u8']] }]]);

export const serializeUpdatePermissionInstruction = ( role) => {
  const kind = UpdatePermissionKind;
  const ix = new UpdatePermissionInstruction({ kind, role });
  return serialize(updatePermissionSchema, ix);  
}

export class PermissionRole extends Assignable { toString() {
  return `key: ${new PublicKey(Buffer.from(this.key)).toBase58()}, role: ${this.role.toString()}`
} }
//export class PermissionRole extends Assignable { }

//export const PermissionRoleSchema = new Map([[PermissionRole, {kind: 'struct', fields: [['key',[32]],['role','u8']]}]]);

/**
 * Permission State
 */
export class PermissionState extends Assignable { }
export const PermissionStateSchema = new Map([
  [PermissionState, { kind: 'struct', fields: [['is_initialized', 'u8'], ['roles',[PermissionRole, 8]]] }],
  [PermissionRole, {kind: 'struct', fields: [['key',[32]],['role','u8']]}],
]);

export const splitBuffer = (b) => {
  let initialized = b.slice(0, 1);
  let split = b.slice(1, b.length);
  return {prefix: initialized, array: split}
} 


const jsArrayPrefix = new Uint8Array([8,0,0,0])
export const deserializePermission = (i, b) => deserialize(PermissionStateSchema, PermissionState, Buffer.concat([i, jsArrayPrefix, b]))
