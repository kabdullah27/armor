import { invoke } from '@tauri-apps/api/core';
import type { OperationResult, EncryptionResult, DecryptionResult } from '../types/key';

export async function encryptFile(params: {
  inputPath: string;
  outputPath: string;
  recipientFingerprints: string[];
  signWith?: string;
  passphrase?: string;
  armor: boolean;
}): Promise<OperationResult<EncryptionResult>> {
  return invoke('encrypt_file', {
    inputPath: params.inputPath,
    outputPath: params.outputPath,
    recipientFingerprints: params.recipientFingerprints,
    signWith: params.signWith,
    passphrase: params.passphrase,
    armor: params.armor,
  });
}

export async function decryptFile(params: {
  inputPath: string;
  outputPath: string;
  passphrase?: string;
}): Promise<OperationResult<DecryptionResult>> {
  return invoke('decrypt_file', {
    inputPath: params.inputPath,
    outputPath: params.outputPath,
    passphrase: params.passphrase,
  });
}
