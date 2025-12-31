import { invoke } from "@tauri-apps/api/core";
import type { KeyMetadata, OperationResult } from "$lib/types/key";

export async function listKeys(): Promise<OperationResult<KeyMetadata[]>> {
  try {
    return await invoke("list_keys");
  } catch (e) {
    return { success: false, error: String(e) };
  }
}

export async function deleteKey(fingerprint: string): Promise<OperationResult<boolean>> {
  try {
    return await invoke("delete_key", { fingerprint });
  } catch (e) {
    return { success: false, error: String(e) };
  }
}

export async function generateKey(params: {
  name: string;
  email: string;
  passphrase: string;
  comment?: string;
  keyType: string;
  expiryTimestamp?: number;
}): Promise<OperationResult<KeyMetadata>> {
  try {
    return await invoke("generate_key", params);
  } catch (e) {
    return { success: false, error: String(e) };
  }
}

export async function importKey(keyText: string): Promise<OperationResult<KeyMetadata>> {
  try {
    return await invoke("import_key", { keyText });
  } catch (e) {
    return { success: false, error: String(e) };
  }
}

export async function exportKey(fingerprint: string, exportPrivate: boolean): Promise<OperationResult<string>> {
  try {
    return await invoke("export_key", { fingerprint: fingerprint, exportPrivate: exportPrivate });
  } catch (e) {
    return { success: false, error: String(e) };
  }
}
