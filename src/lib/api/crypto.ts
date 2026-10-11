import { invoke } from '@tauri-apps/api/core';
import type { OperationResult, EncryptionResult, DecryptionResult } from '../types/key';

export async function encryptFile(params: {
  inputPath: string;
  outputPath: string;
  recipientFingerprints: string[];
  armor: boolean;
}): Promise<OperationResult<EncryptionResult>> {
  try {
    return await invoke('encrypt_file_cmd', params);
  } catch (e) {
    return { success: false, error: String(e) };
  }
}

export async function decryptFile(params: {
  inputPath: string;
  outputPath: string;
  passphrase: string;
  targetFingerprint?: string | null;
}): Promise<OperationResult<DecryptionResult>> {
  try {
    return await invoke('decrypt_file_cmd', params);
  } catch (e) {
    return { success: false, error: String(e) };
  }
}
