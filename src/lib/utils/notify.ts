import { message } from "@tauri-apps/plugin-dialog";

/**
 * Non-blocking replacement for window.alert().
 * alert() is synchronous and, inside the Tauri webview, can block the JS thread
 * without a visible dialog, which freezes the whole UI.
 */
export function notify(text: string, kind: "info" | "warning" | "error" = "error") {
  const title = kind === "info" ? "Armor" : kind === "warning" ? "Warning" : "Error";
  return message(text, { title, kind }).catch((e) => console.error(text, e));
}
