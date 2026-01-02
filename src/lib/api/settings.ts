import { invoke } from "@tauri-apps/api/core";
import type { OperationResult } from "$lib/types/key";

export async function getDbPath(): Promise<OperationResult<string>> {
  try {
    return await invoke("get_db_path");
  } catch (e) {
    return { success: false, error: String(e) };
  }
}

export async function setDbPath(path: string): Promise<OperationResult<boolean>> {
  try {
    return await invoke("set_db_path", { path });
  } catch (e) {
    return { success: false, error: String(e) };
  }
}

export async function backupDb(targetPath: string): Promise<OperationResult<boolean>> {
  try {
    return await invoke("backup_db", { targetPath: targetPath });
  } catch (e) {
    return { success: false, error: String(e) };
  }
}

export async function restoreDb(sourcePath: string): Promise<OperationResult<boolean>> {
  try {
    return await invoke("restore_db", { sourcePath: sourcePath });
  } catch (e) {
    return { success: false, error: String(e) };
  }
}

export async function isFirstRun(): Promise<OperationResult<boolean>> {
  try {
    return await invoke("is_first_run");
  } catch (e) {
    return { success: false, error: String(e) };
  }
}

export async function completeOnboarding(): Promise<OperationResult<boolean>> {
  try {
    return await invoke("complete_onboarding");
  } catch (e) {
    return { success: false, error: String(e) };
  }
}
