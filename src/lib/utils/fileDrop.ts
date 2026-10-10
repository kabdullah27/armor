import { getCurrentWebview } from "@tauri-apps/api/webview";

/**
 * Subscribes to native file drops on the app window.
 * In Tauri v2 the HTML5 `drop` event doesn't expose file system paths
 * (`File.path` is undefined), so dropped files must be read from the webview event.
 * Returns a cleanup function.
 */
export function onFileDrop(handler: (paths: string[]) => void): () => void {
  let unlisten: (() => void) | undefined;
  let disposed = false;

  let webview;
  try {
    webview = getCurrentWebview();
  } catch (e) {
    // Not running inside Tauri (e.g. plain browser preview)
    console.warn("File drop unavailable:", e);
    return () => {};
  }

  webview
    .onDragDropEvent((event) => {
      if (event.payload.type === "drop" && event.payload.paths.length > 0) {
        handler(event.payload.paths);
      }
    })
    .then((fn) => {
      if (disposed) fn();
      else unlisten = fn;
    })
    .catch((e) => console.error("Failed to register file drop listener:", e));

  return () => {
    disposed = true;
    unlisten?.();
  };
}
