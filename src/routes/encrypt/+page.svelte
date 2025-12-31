<script lang="ts">
  import { onMount } from "svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import { listKeys } from "$lib/api/keys";
  import type { KeyMetadata } from "$lib/types/key";
  import { invoke } from "@tauri-apps/api/core"; // Added for onMount invoke

  // State
  // State
  let files: string[] = [];
  let recipients: any[] = [];
  let selectedRecipients: string[] = [];
  let armor = false; // Default to binary
  let loading = false;
  let processing = false; // Renamed from encrypting to match usage

  onMount(async () => {
    loading = true;
    try {
      const result: any = await invoke("list_keys");
      if (result.status === "ok" || result.success) {
        // Allow all keys as recipients (users often want to encrypt to themselves or valid public keys)
        recipients = result.data;
      } else {
        alert("Failed to load keys: " + result.message);
      }
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  });

  async function selectFile() {
    try {
      const selected = await open({
        multiple: true,
        directory: false,
      });

      if (selected) {
        if (Array.isArray(selected)) {
          files = [...files, ...selected];
        } else {
          files = [...files, selected];
        }
      }
    } catch (e) {
      console.error("File selection error:", e);
      alert("Error selecting file: " + e);
    }
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    e.stopPropagation();
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    e.stopPropagation();

    if (
      e.dataTransfer &&
      e.dataTransfer.files &&
      e.dataTransfer.files.length > 0
    ) {
      const droppedFiles = Array.from(e.dataTransfer.files);
      // In Tauri, the File object usually exposes the path, but check just in case
      // Note: This relies on the WebView exposing the full path
      const paths = droppedFiles
        .map((f) => {
          // @ts-ignore
          return f.path || f.name; // Fallback to name if path missing (will fail in backend but show error)
        })
        .filter((p) => !!p);

      files = [...files, ...paths];
    }
  }

  function removeFile(index: number) {
    files = files.filter((_, i) => i !== index);
  }

  function selectKey(fingerprint: string) {
    // Single selection - hanya satu key yang bisa dipilih
    selectedRecipients = [fingerprint];
  }

  async function handleEncrypt() {
    if (files.length === 0) return alert("Please select files to encrypt");
    if (selectedRecipients.length === 0)
      return alert("Please select at least one recipient");

    try {
      processing = true;

      // Import dialog for save
      const { save } = await import("@tauri-apps/plugin-dialog");
      const { invoke } = await import("@tauri-apps/api/core");

      for (const filePath of files) {
        // Just extract filename from path for suggestion
        const filename = filePath.split(/[\\/]/).pop() || "encrypted.gpg";

        const savePath = await save({
          title: `Save Encrypted File for ${filename}`,
          defaultPath: `${filename}.gpg`,
          filters: [
            { name: "PGP Encrypted", extensions: ["gpg", "pgp", "asc"] },
          ],
        });

        if (!savePath) {
          console.log("Save cancelled for", filePath);
          continue;
        }

        // Call backend
        // Note: Backend expecting: input_path, output_path, recipient_fingerprints, armor
        // We need to implement encrypt_file_cmd in backend properly or map it here.
        // Assuming the backend command signature from crypto.rs
        const res: any = await invoke("encrypt_file_cmd", {
          inputPath: filePath,
          outputPath: savePath,
          recipientFingerprints: selectedRecipients,
          armor: savePath.endsWith(".asc"), // Auto-detect armor based on extension
        });

        if (!res.success) {
          alert(`Failed to encrypt ${filename}: ${res.error}`);
          // Don't break loop, try others? Or break?
        }
      }

      alert("Encryption process completed!");
      files = [];
      selectedRecipients = [];
    } catch (e) {
      // console.error(e);
      alert("Encryption error: " + e);
    } finally {
      processing = false;
    }
  }
</script>

<div class="container">
  <div class="page-header">
    <h1>🔒 Encrypt Files</h1>
    <p class="subtitle">Secure your files with PGP encryption</p>
  </div>

  <div class="workflow-grid">
    <!-- File Selection -->
    <!-- File Selection -->
    <div class="card">
      <div class="card-header">
        <div class="step-badge">1</div>
        <h2>Files to Encrypt</h2>
      </div>

      <div class="card-body" on:drop={handleDrop} on:dragover={handleDragOver}>
        {#if files.length === 0}
          <div class="upload-area" on:click={selectFile}>
            <div class="upload-icon">📄</div>
            <h3>Drag & Drop Files Here</h3>
            <p class="text-sm text-gray-400 mb-4">or click to browse</p>
            <button class="btn-secondary">Choose Files</button>
          </div>
        {:else}
          <div class="file-list-header">
            <span>{files.length} file(s) selected</span>
            <button class="btn-text" on:click={selectFile}>+ Add More</button>
          </div>
          <div class="file-list">
            {#each files as file, i}
              <div class="file-item">
                <span class="file-icon">📄</span>
                <span class="file-name" title={file}
                  >{file.split(/[\\/]/).pop()}</span
                >
                <button
                  class="remove-btn"
                  on:click|stopPropagation={() => removeFile(i)}>×</button
                >
              </div>
            {/each}
          </div>
        {/if}
      </div>
    </div>

    <!-- Step 2: Select Key -->
    <div class="card">
      <div class="card-header">
        <div class="step-badge">2</div>
        <h2>Select Key</h2>
      </div>

      {#if loading}
        <div class="loading">Loading keys...</div>
      {:else if recipients.length === 0}
        <div class="empty-keys">
          <div class="empty-icon">📭</div>
          <h3>No Keys Found</h3>
          <p>
            You need at least one public key to encrypt files.
          </p>
          <a href="/keys" class="btn-primary small">Generate or Import Keys</a>
        </div>
      {:else}
        <div class="card-body p-4">
          <label class="form-label">Select Key</label>
          <div class="keys-list-mini">
            {#each recipients as key}
              <div
                class="key-option"
                class:selected={selectedRecipients.includes(key.fingerprint)}
                on:click={() => selectKey(key.fingerprint)}
                role="button"
                tabindex="0"
                on:keydown={(e) => e.key === 'Enter' && selectKey(key.fingerprint)}
              >
                <span class="icon">🔑</span>
                <div class="info">
                  <span class="name">{key.user_id.name}</span>
                  <span class="fp">{key.fingerprint.substring(0, 8)}...</span>
                </div>
              </div>
            {/each}
          </div>
        </div>
      {/if}
    </div>
  </div>

  <div class="action-bar">
    <div class="summary">
      Encrypting <strong>{files.length}</strong> file(s) with
      <strong>{selectedRecipients.length > 0 ? 1 : 0}</strong> key
    </div>
    <button
      class="btn-primary large"
      disabled={files.length === 0 ||
        selectedRecipients.length === 0 ||
        processing}
      on:click={handleEncrypt}
    >
      {processing ? "Encrypting..." : "🔒 Encrypt Files"}
    </button>
  </div>
</div>

<style>
  .container {
    max-width: 1000px;
    margin: 0 auto;
    padding: 24px;
    padding-bottom: 100px;
  }

  .nav {
    display: flex;
    gap: 16px;
    margin-bottom: 32px;
  }

  .nav-link {
    text-decoration: none;
    color: #6b7280;
    font-weight: 500;
  }

  .nav-link.active {
    color: #3b82f6;
    font-weight: 600;
  }

  .page-header h1 {
    margin: 0 0 8px 0;
    color: #1f2937;
  }

  .subtitle {
    color: #6b7280;
    margin: 0 0 32px 0;
  }

  .workflow-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 24px;
  }

  @media (max-width: 768px) {
    .workflow-grid {
      grid-template-columns: 1fr;
    }
  }

  .card {
    background: white;
    border-radius: 12px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
    overflow: hidden;
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .card-header {
    padding: 16px 20px;
    border-bottom: 1px solid #f3f4f6;
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .card-header h2 {
    margin: 0;
    font-size: 18px;
    color: #1f2937;
  }

  .step-badge {
    background: #e0e7ff;
    color: #4338ca;
    width: 28px;
    height: 28px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 600;
    font-size: 14px;
  }

  /* File Upload Styles */
  .upload-area {
    padding: 40px;
    text-align: center;
    border-bottom: 1px solid #f3f4f6;
    cursor: pointer;
    background: #f9fafb;
    transition: all 0.2s;
  }

  .upload-area:hover {
    background: #f3f4f6;
  }

  .upload-icon {
    font-size: 48px;
    margin-bottom: 16px;
  }

  .btn-secondary {
    margin-top: 16px;
    background: white;
    border: 1px solid #d1d5db;
    padding: 8px 16px;
    border-radius: 6px;
    cursor: pointer;
  }

  .file-list {
    padding: 0;
    max-height: 300px;
    overflow-y: auto;
  }

  .file-item {
    padding: 12px 20px;
    border-bottom: 1px solid #f3f4f6;
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 14px;
  }

  .file-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 90%;
  }

  .remove-btn {
    background: none;
    border: none;
    color: #ef4444;
    font-size: 18px;
    cursor: pointer;
  }

  /* Recipient Styles */
  .recipients-list {
    max-height: 400px;
    overflow-y: auto;
  }

  .recipient-item {
    padding: 12px 20px;
    display: flex;
    gap: 12px;
    align-items: center;
    border-bottom: 1px solid #f3f4f6;
    cursor: pointer;
    transition: background 0.2s;
  }

  .recipient-item:hover {
    background: #f9fafb;
  }

  .recipient-item.selected {
    background: #eff6ff;
  }

  .recipient-info {
    display: flex;
    flex-direction: column;
  }

  .name {
    font-weight: 500;
    color: #1f2937;
  }

  .email {
    font-size: 13px;
    color: #6b7280;
  }

  .fingerprint {
    font-size: 11px;
    color: #9ca3af;
    background: #f3f4f6;
    padding: 2px 4px;
    border-radius: 4px;
    width: fit-content;
    margin-top: 2px;
  }

  .action-bar {
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    background: white;
    padding: 20px;
    border-top: 1px solid #e5e7eb;
    display: flex;
    justify-content: space-between;
    align-items: center;
    box-shadow: 0 -4px 6px -1px rgba(0, 0, 0, 0.05);
    z-index: 100;
  }

  .summary {
    font-size: 16px;
    color: #374151;
  }

  .btn-primary.large {
    background: #3b82f6;
    color: white;
    border: none;
    padding: 12px 32px;
    border-radius: 8px;
    font-weight: 600;
    font-size: 16px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-primary:hover:not(:disabled) {
    background: #2563eb;
    transform: translateY(-1px);
    box-shadow: 0 4px 6px -1px rgba(59, 130, 246, 0.3);
  }

  .btn-primary:disabled {
    background: #9ca3af;
    cursor: not-allowed;
  }
  /* Empty State Styles */
  .empty-keys {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 40px 20px;
    text-align: center;
    color: #6b7280;
    height: 100%;
  }

  .empty-icon {
    font-size: 48px;
    margin-bottom: 16px;
    opacity: 0.5;
  }

  .empty-keys h3 {
    margin: 0 0 8px 0;
    color: #374151;
    font-size: 18px;
  }

  .empty-keys p {
    margin: 0 0 24px 0;
    max-width: 300px;
    font-size: 14px;
  }

  .btn-primary.small {
    background: #3b82f6;
    color: white;
    padding: 8px 16px;
    border-radius: 6px;
    text-decoration: none;
    font-size: 14px;
    font-weight: 500;
    transition: background 0.2s;
  }

  .btn-primary.small:hover {
    background: #2563eb;
  }

  .card-body {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .file-list-header {
    padding: 12px 20px;
    border-bottom: 1px solid #f3f4f6;
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 13px;
    color: #6b7280;
    background: #f9fafb;
  }

  .btn-text {
    background: none;
    border: none;
    color: #2563eb;
    font-weight: 500;
    cursor: pointer;
    font-size: 13px;
  }

  .btn-text:hover {
    text-decoration: underline;
  }

  .file-icon {
    font-size: 16px;
    margin-right: 8px;
  }

  /* Keys List Mini - sama dengan decrypt page */
  .p-4 {
    padding: 20px;
  }

  .form-label {
    display: block;
    font-weight: 500;
    margin-bottom: 8px;
    font-size: 14px;
    color: #374151;
  }

  .keys-list-mini {
    max-height: 200px;
    overflow-y: auto;
    border: 1px solid #e5e7eb;
    border-radius: 6px;
  }

  .key-option {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 12px;
    cursor: pointer;
    border-bottom: 1px solid #f3f4f6;
    transition: background 0.1s;
  }

  .key-option:last-child {
    border-bottom: none;
  }

  .key-option:hover {
    background: #f9fafb;
  }

  .key-option.selected {
    background: #eff6ff;
    border-left: 3px solid #3b82f6;
  }

  .key-option .icon {
    font-size: 18px;
  }

  .key-option .info {
    display: flex;
    flex-direction: column;
  }

  .key-option .name {
    font-weight: 500;
    font-size: 14px;
  }

  .key-option .fp {
    font-size: 11px;
    color: #9ca3af;
  }
</style>
