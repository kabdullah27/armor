<script lang="ts">
  import { onMount } from "svelte";
  import { open, save } from "@tauri-apps/plugin-dialog";
  import { invoke } from "@tauri-apps/api/core";
  import { listKeys } from "$lib/api/keys";

  let file = "";
  let passphrase = "";
  let processing = false;
  let result = "";
  let myKeys: any[] = [];
  let selectedKeyFingerprint: string | null = null;
  let loadingKeys = false;

  onMount(async () => {
    loadingKeys = true;
    try {
      const res = await listKeys();
      if (res.success && res.data) {
        // Filter for private keys (ones we can use to decrypt)
        myKeys = res.data.filter((k: any) => k.is_private);
        if (myKeys.length > 0) {
          selectedKeyFingerprint = myKeys[0].fingerprint;
        }
      }
    } catch (e) {
      console.error(e);
    } finally {
      loadingKeys = false;
    }
  });

  async function selectFile() {
    try {
      const selected = await open({
        multiple: false,
        directory: false,
        filters: [
          {
            name: "Encrypted Files",
            extensions: ["gpg", "pgp", "asc", "enc"],
          },
        ],
      });

      if (selected) {
        file = typeof selected === "string" ? selected : selected.path;
      }
    } catch (e) {
      console.error(e);
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

    if (e.dataTransfer?.files.length) {
      // @ts-ignore
      const f = e.dataTransfer.files[0];
      // @ts-ignore
      file = f.path || f.name;
    }
  }

  function removeFile() {
    file = "";
  }

  async function handleDecrypt() {
    if (!file) return;

    processing = true;
    result = "";

    try {
      // Ask where to save the decrypted file
      const originalName = file.split(/[\\/]/).pop() || "decrypted";
      const cleanName = originalName.replace(/\.(gpg|pgp|asc|enc)$/i, "");

      const savePath = await save({
        title: "Save Decrypted File",
        defaultPath: cleanName,
      });

      if (!savePath) {
        processing = false;
        return;
      }

      // Call backend
      // Note: We are using the 'decrypt_file_cmd' which needs implementation update in Rust
      const res: any = await invoke("decrypt_file_cmd", {
        // Updated to match backend command name if different
        inputPath: file,
        outputPath: savePath,
        passphrase: passphrase,
      });

      if (res.success) {
        result = `File decrypted successfully to: ${savePath}`;
        alert("Decryption successful!");
        file = "";
      } else {
        alert("Decryption failed: " + res.error);
      }
    } catch (e) {
      alert("Error: " + e);
    } finally {
      processing = false;
    }
  }
</script>

<div class="container">
  <div class="page-header">
    <h1>🔓 Decrypt File</h1>
    <p class="subtitle">Decrypt PGP encrypted files using your private keys</p>
  </div>

  <div class="workflow-grid">
    <!-- Step 1: File Selection -->
    <div class="card">
      <div class="card-header">
        <div class="step-badge">1</div>
        <h2>Encrypted File</h2>
      </div>

      <div class="card-body" on:drop={handleDrop} on:dragover={handleDragOver}>
        {#if !file}
          <div class="upload-area" on:click={selectFile}>
            <div class="upload-icon">🔐</div>
            <h3>Drag & Drop Encrypted File</h3>
            <p class="text-sm text-gray-400 mb-4">or click to browse</p>
            <button class="btn-secondary">Choose File</button>
          </div>
        {:else}
          <div class="file-list">
            <div class="file-item">
              <span class="file-icon">🔒</span>
              <span class="file-name" title={file}
                >{file.split(/[\\/]/).pop()}</span
              >
              <button class="remove-btn" on:click={removeFile}>×</button>
            </div>
          </div>
        {/if}
      </div>
    </div>

    <!-- Step 2: Key & Passphrase -->
    <div class="card">
      <div class="card-header">
        <div class="step-badge">2</div>
        <h2>Unlock Key</h2>
      </div>

      <div class="card-body p-4">
        {#if loadingKeys}
          <div class="loading">Loading keys...</div>
        {:else if myKeys.length === 0}
          <div class="empty-keys">
            <div class="empty-icon">🚫</div>
            <h3>No Private Keys</h3>
            <p>You need a private key to decrypt files.</p>
            <a href="/keys" class="btn-primary small">Generate Key</a>
          </div>
        {:else}
          <div class="form-group">
            <label>Select Key (for context)</label>
            <div class="keys-list-mini">
              {#each myKeys as key}
                <div
                  class="key-option"
                  class:selected={selectedKeyFingerprint === key.fingerprint}
                  on:click={() => (selectedKeyFingerprint = key.fingerprint)}
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

          <div class="form-group mt-4">
            <label>Passphrase</label>
            <input
              type="password"
              bind:value={passphrase}
              placeholder="Enter passphrase for selected key"
              class="w-full p-2 border rounded"
            />
            <p class="text-xs text-gray-500 mt-1">
              Leave empty if key has no passphrase
            </p>
          </div>
        {/if}
      </div>
    </div>
  </div>

  <div class="action-bar">
    <div class="summary">
      {#if result}
        <span class="text-green-600 font-medium">{result}</span>
      {:else}
        Ready to decrypt
      {/if}
    </div>
    <button
      class="btn-primary large"
      disabled={!file ||
        (myKeys.length > 0 && !selectedKeyFingerprint) ||
        processing}
      on:click={handleDecrypt}
    >
      {processing ? "Decrypting..." : "🔓 Decrypt File"}
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
  }

  .card-body {
    flex: 1;
    display: flex;
    flex-direction: column;
  }
  .p-4 {
    padding: 20px;
  }

  /* Upload Area */
  .upload-area {
    padding: 40px;
    text-align: center;
    cursor: pointer;
    background: #f9fafb;
    flex: 1;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }
  .upload-icon {
    font-size: 40px;
    margin-bottom: 16px;
  }

  .file-list {
    padding: 12px;
  }
  .file-item {
    display: flex;
    background: #f3f4f6;
    padding: 10px;
    border-radius: 6px;
    align-items: center;
    gap: 10px;
  }
  .file-name {
    flex: 1;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }
  .remove-btn {
    border: none;
    background: none;
    color: red;
    cursor: pointer;
    font-size: 18px;
  }

  .btn-secondary {
    background: white;
    border: 1px solid #d1d5db;
    padding: 8px 16px;
    border-radius: 6px;
    cursor: pointer;
  }

  /* Keys List */
  .keys-list-mini {
    max-height: 200px;
    overflow-y: auto;
    border: 1px solid #e5e7eb;
    border-radius: 6px;
    margin-top: 8px;
  }
  .key-option {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 12px;
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

  .info {
    display: flex;
    flex-direction: column;
  }
  .name {
    font-weight: 500;
    font-size: 14px;
  }
  .fp {
    font-size: 11px;
    color: #9ca3af;
  }

  .form-group label {
    display: block;
    font-weight: 500;
    margin-bottom: 4px;
    font-size: 14px;
  }
  input[type="password"] {
    width: 100%;
    padding: 10px;
    border: 1px solid #d1d5db;
    border-radius: 6px;
    margin-top: 4px;
  }

  /* Action Bar */
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
    z-index: 100;
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
  }
  .btn-primary:disabled {
    background: #9ca3af;
    cursor: not-allowed;
  }

  /* Empty State */
  .empty-keys {
    text-align: center;
    padding: 20px;
    color: #6b7280;
  }
  .empty-icon {
    font-size: 32px;
    margin-bottom: 8px;
  }
  .btn-primary.small {
    display: inline-block;
    background: #3b82f6;
    color: white;
    padding: 6px 12px;
    border-radius: 6px;
    text-decoration: none;
    margin-top: 10px;
    font-size: 13px;
  }
</style>
