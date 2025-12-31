<script lang="ts">
  import { onMount } from "svelte";
  import { open, save } from "@tauri-apps/plugin-dialog";
  import { readTextFile, writeTextFile } from "@tauri-apps/plugin-fs";
  import {
    listKeys,
    deleteKey,
    generateKey,
    importKey,
    exportKey,
  } from "$lib/api/keys";
  import type { KeyMetadata } from "$lib/types/key";

  let keys: KeyMetadata[] = [];
  let loading = true;
  let error = "";
  let showGenerateModal = false;

  // Generate key form
  let name = "";
  let email = "";
  let comment = "";
  let keyType: "rsa4096" | "ed25519" = "rsa4096";
  let datePart = "";
  let timePart = "12:00";
  let neverExpire = true;
  let passphrase = "";
  let confirmPass = "";
  let generating = false;

  onMount(async () => {
    await loadKeys();
  });

  async function loadKeys() {
    loading = true;
    error = "";
    try {
      const result = await listKeys();
      if (result.success && result.data) {
        keys = result.data;
      } else {
        error = result.error || "Failed to load keys";
      }
    } catch (e) {
      error = "Failed to load keys: " + e;
    } finally {
      loading = false;
    }
  }

  async function handleDelete(fingerprint: string) {
    console.log("handleDelete called with fingerprint:", fingerprint);

    console.log("Calling deleteKey API...");
    try {
      const result = await deleteKey(fingerprint);
      console.log("deleteKey result:", result);
      if (result.success) {
        alert("Key deleted successfully!");
        await loadKeys();
      } else {
        alert("Failed to delete: " + (result.error || "Unknown error"));
      }
    } catch (e) {
      console.error("Delete error:", e);
      alert("Error: " + String(e));
    }
  }

  async function handleImport() {
    try {
      console.log("Opening import dialog...");
      const selected = await open({
        title: "Import PGP Key",
        filters: [{ name: "PGP Key", extensions: ["asc", "gpg", "txt"] }],
        multiple: false,
      });

      if (!selected) return;
      const path = Array.isArray(selected) ? selected[0] : selected;
      if (!path) return;

      loading = true;
      console.log("Reading file:", path);
      const content = await readTextFile(path);
      console.log("Importing key content...");
      const res = await importKey(content);

      if (res.success) {
        alert("Key imported successfully!");
        await loadKeys();
      } else {
        alert("Failed to import key: " + res.error);
      }
    } catch (e) {
      console.error("Import error:", e);
      alert("Import error: " + e);
    } finally {
      loading = false;
    }
  }

  async function handleExport(fingerprint: string, isPrivate: boolean) {
    alert(`EXPORT CLICKED! Type: ${isPrivate ? "PRIVATE" : "PUBLIC"}`);
    console.log(
      "handleExport called with fingerprint:",
      fingerprint,
      "isPrivate:",
      isPrivate
    );
    try {
      console.log("Calling exportKey API...");
      const res = await exportKey(fingerprint, isPrivate);
      console.log("exportKey result:", res);

      if (!res.success || !res.data) {
        alert("Failed to get key content: " + (res.error || "Unknown error"));
        return;
      }

      const typeStr = isPrivate ? "private" : "public";
      const savePath = await save({
        title: `Export ${typeStr.charAt(0).toUpperCase() + typeStr.slice(1)} Key`,
        defaultPath: `${typeStr}-key-${fingerprint.substring(0, 8)}.asc`,
        filters: [{ name: "ASCII Armor", extensions: ["asc"] }],
      });

      if (!savePath) {
        console.log("Save path cancelled");
        alert("Save cancelled");
        return;
      }

      console.log("Writing to file:", savePath);
      await writeTextFile(savePath, res.data);
      alert(`Key exported successfully to:\n${savePath}`);
    } catch (e) {
      console.error("Export error:", e);
      alert("Error: " + String(e));
    }
  }

  async function handleGenerate() {
    if (!name || !email) {
      alert("Please fill in Name and Email");
      return;
    }

    if (passphrase !== confirmPass) {
      alert("Passphrases do not match");
      return;
    }

    // Optional: Warn if passphrase is empty?
    // No, user explicitly asked for it to be optional.

    try {
      generating = true;
      console.log("Generating key for:", name, email);

      let expiryTimestamp = null;
      if (!neverExpire && datePart && timePart) {
        const dateTimeString = `${datePart}T${timePart}`;
        expiryTimestamp = Math.floor(new Date(dateTimeString).getTime() / 1000);
      }

      const res = await generateKey({
        name,
        email,
        comment,
        keyType,
        passphrase,
        expiryTimestamp,
      });

      console.log("Generation result:", res);

      if (res.success) {
        showGenerateModal = false;
        resetForm();
        await listKeys().then((r) => {
          if (r.success && r.data) keys = r.data;
        });
        alert("Key generated successfully!");
      } else {
        alert("Error generating key: " + res.error);
      }
    } catch (e) {
      console.error("Generation error:", e);
      alert("An unexpected error occurred: " + e);
    } finally {
      generating = false;
    }
  }

  function resetForm() {
    name = "";
    email = "";
    comment = "";
    passphrase = "";
    confirmPass = "";
    datePart = "";
    timePart = "12:00";
    neverExpire = true;
    keyType = "rsa4096";
  }

  // Expose functions to window for onclick HTML attributes
  if (typeof window !== "undefined") {
    (window as any).exportPublicKey = (fingerprint: string) => {
      console.log("WINDOW exportPublicKey called:", fingerprint);
      handleExport(fingerprint, false);
    };
    (window as any).exportPrivateKey = (fingerprint: string) => {
      console.log("WINDOW exportPrivateKey called:", fingerprint);
      handleExport(fingerprint, true);
    };
    (window as any).deleteThisKey = (fingerprint: string) => {
      console.log("WINDOW deleteThisKey called:", fingerprint);
      handleDelete(fingerprint);
    };
  }
</script>

<div class="container">
  <div class="page-header">
    <h1>🔑 PGP Keys</h1>
    <button class="btn-secondary" on:click={handleImport}>
      📥 Import Key
    </button>
    <button class="btn-primary" on:click={() => (showGenerateModal = true)}>
      + Generate New Key
    </button>
  </div>

  {#if loading}
    <div class="loading">Loading keys...</div>
  {:else if error}
    <div class="error">{error}</div>
  {:else if keys.length === 0}
    <div class="empty">
      <p>No keys found</p>
      <p>Click "Generate New Key" to create your first PGP key</p>
    </div>
  {:else}
    <div class="keys-list">
      {#each keys as key}
        <div class="key-item">
          <div class="key-main">
            <div class="key-icon-wrapper">
              <span class="key-type-icon">{key.is_private ? "🔐" : "🔓"}</span>
            </div>
            <div class="key-content">
              <h3 class="key-name">{key.user_id.name}</h3>
              <p class="key-email">{key.user_id.email}</p>
              <div class="key-meta">
                <span class="meta-item">🔑 {key.key_type}</span>
                <span class="meta-item"
                  >🔖 {key.fingerprint.substring(0, 16)}...</span
                >
                <span class="meta-item"
                  >📅 {new Date(key.created_at).toLocaleDateString()}</span
                >
              </div>
            </div>
          </div>

          <div style="display: flex; gap: 8px;">
            <button
              style="padding: 12px 20px; background: #000; color: #fff; border: none; border-radius: 8px; cursor: pointer; font-size: 14px; font-weight: 500; z-index: 999;"
              onclick="window.exportPublicKey('{key.fingerprint}'); return false;"
            >
              📤 Public
            </button>

            {#if key.is_private}
              <button
                style="padding: 12px 20px; background: #000; color: #fff; border: none; border-radius: 8px; cursor: pointer; font-size: 14px; font-weight: 500; z-index: 999;"
                onclick="window.exportPrivateKey('{key.fingerprint}'); return false;"
              >
                🔐 Private
              </button>
            {/if}

            <button
              style="padding: 12px 20px; background: #fff; color: #000; border: 1px solid #ddd; border-radius: 8px; cursor: pointer; font-size: 14px; font-weight: 500; z-index: 999;"
              onclick="window.deleteThisKey('{key.fingerprint}'); return false;"
            >
              🗑️ Delete
            </button>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

{#if showGenerateModal}
  <div class="modal-overlay" on:click={() => (showGenerateModal = false)}>
    <div class="modal" on:click|stopPropagation>
      <div class="modal-header">
        <h2>Generate New PGP Key</h2>
        <button class="close-btn" on:click={() => (showGenerateModal = false)}
          >×</button
        >
      </div>

      <div class="modal-body">
        <div class="form-group">
          <label for="name">Full Name *</label>
          <input
            id="name"
            type="text"
            bind:value={name}
            placeholder="John Doe"
          />
        </div>

        <div class="form-group">
          <label for="email">Email Address *</label>
          <input
            id="email"
            type="email"
            bind:value={email}
            placeholder="john@example.com"
          />
        </div>

        <div class="form-group">
          <label for="comment">Comment (optional)</label>
          <input
            id="comment"
            type="text"
            bind:value={comment}
            placeholder="Work Key"
          />
        </div>

        <div class="form-group">
          <label>Key Type</label>
          <div class="radio-group">
            <label class="radio-card {keyType === 'rsa4096' ? 'selected' : ''}">
              <input type="radio" bind:group={keyType} value="rsa4096" />
              <div class="radio-info">
                <span class="radio-title">RSA 4096</span>
                <span class="radio-desc">Recommended for compatibility</span>
              </div>
            </label>
            <label class="radio-card {keyType === 'ed25519' ? 'selected' : ''}">
              <input type="radio" bind:group={keyType} value="ed25519" />
              <div class="radio-info">
                <span class="radio-title">Ed25519</span>
                <span class="radio-desc">Modern, faster, smaller keys</span>
              </div>
            </label>
          </div>
        </div>

        <div class="form-group">
          <label for="expiry">Key Expiration</label>
          <div class="expiry-row">
            <div class="date-time-inputs">
              <input
                type="date"
                bind:value={datePart}
                disabled={neverExpire}
                min={new Date().toISOString().split("T")[0]}
                class="input-date"
              />
              <input
                type="time"
                bind:value={timePart}
                disabled={neverExpire}
                class="input-time"
              />
            </div>
            <label class="checkbox-label">
              <input type="checkbox" bind:checked={neverExpire} />
              <span>Never expire</span>
            </label>
          </div>
        </div>

        <div class="form-group">
          <label for="pass">Passphrase (optional)</label>
          <input
            id="pass"
            type="password"
            bind:value={passphrase}
            placeholder="Leave empty for no protection"
          />
        </div>

        <div class="form-group">
          <label for="confirm">Confirm Passphrase</label>
          <input
            id="confirm"
            type="password"
            bind:value={confirmPass}
            placeholder="Re-enter passphrase"
          />
        </div>

        <div class="alert">
          <strong>Note:</strong> Using a passphrase is recommended to protect your
          private key.
        </div>
      </div>

      <div class="modal-footer">
        <button
          class="btn-secondary"
          on:click={() => {
            showGenerateModal = false;
            resetForm();
          }}
        >
          Cancel
        </button>
        <button
          class="btn-primary"
          on:click={handleGenerate}
          disabled={generating}
        >
          {generating ? "Generating..." : "Generate Key"}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 24px;
  }

  .nav {
    display: flex;
    gap: 16px;
    margin-bottom: 24px;
    padding: 16px;
    background: white;
    border-radius: 8px;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
  }

  .nav-link {
    padding: 8px 16px;
    text-decoration: none;
    color: #4b5563;
    border-radius: 6px;
    font-weight: 500;
  }

  .nav-link.active {
    background: #eff6ff;
    color: #3b82f6;
  }

  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 24px;
  }

  .page-header h1 {
    font-size: 28px;
    color: #1f2937;
    margin: 0;
  }

  .loading,
  .error,
  .empty {
    text-align: center;
    padding: 48px;
    background: white;
    border-radius: 8px;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
  }

  .error {
    color: #ef4444;
    background: #fef2f2;
    border: 1px solid #fee2e2;
  }

  .empty p {
    color: #6b7280;
    margin-bottom: 16px;
  }

  /* Modern White List Layout */
  .keys-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .key-item {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 12px;
    padding: 20px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 20px;
    transition: all 0.2s;
  }

  .key-item:hover {
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
    border-color: #d1d5db;
  }

  .key-main {
    display: flex;
    gap: 16px;
    align-items: center;
    flex: 1;
    min-width: 0;
  }

  .key-icon-wrapper {
    flex-shrink: 0;
    width: 48px;
    height: 48px;
    background: #f3f4f6;
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .key-type-icon {
    font-size: 24px;
  }

  .key-content {
    flex: 1;
    min-width: 0;
  }

  .key-name {
    font-size: 16px;
    font-weight: 600;
    color: #111827;
    margin: 0 0 4px 0;
  }

  .key-email {
    font-size: 14px;
    color: #6b7280;
    margin: 0 0 8px 0;
  }

  .key-meta {
    display: flex;
    gap: 16px;
    flex-wrap: wrap;
  }

  .meta-item {
    font-size: 13px;
    color: #9ca3af;
  }

  .key-buttons {
    display: flex;
    gap: 8px;
    flex-shrink: 0;
  }

  .key-btn {
    padding: 8px 16px;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    border: none;
    transition: all 0.2s;
    font-family: inherit;
    white-space: nowrap;
  }

  .key-btn:hover {
    transform: translateY(-1px);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  }

  .key-btn:active {
    transform: translateY(0);
  }

  .key-btn.primary {
    background: #3b82f6;
    color: white;
  }

  .key-btn.primary:hover {
    background: #2563eb;
  }

  .key-btn.warning {
    background: #f59e0b;
    color: white;
  }

  .key-btn.warning:hover {
    background: #d97706;
  }

  .key-btn.danger {
    background: #ef4444;
    color: white;
  }

  .key-btn.danger:hover {
    background: #dc2626;
  }

  /* OLD STYLES - Keep for modal */
  .keys-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
    gap: 16px;
  }

  .key-card {
    background: white;
    padding: 20px;
    border-radius: 8px;
    border: 1px solid #e5e7eb;
    transition: all 0.2s;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
  }

  .key-card:hover {
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  }

  .key-header {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 16px;
  }

  .key-icon {
    font-size: 32px;
  }

  .key-info h3 {
    color: #1f2937;
    margin: 0 0 4px 0;
    font-size: 16px;
    font-weight: 600;
  }

  .key-info p {
    color: #6b7280;
    margin: 0;
    font-size: 14px;
  }

  .key-details {
    margin-bottom: 16px;
    background: #f9fafb;
    border-radius: 6px;
    padding: 8px 12px;
  }

  .detail-row {
    display: flex;
    justify-content: space-between;
    padding: 6px 0;
    border-bottom: 1px solid #e5e7eb;
  }

  .detail-row:last-child {
    border-bottom: none;
  }

  .label {
    color: #6b7280;
    font-size: 13px;
  }

  .badge {
    background: #dbeafe;
    color: #1e40af;
    padding: 2px 8px;
    border-radius: 999px;
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
  }

  code {
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas,
      monospace;
    font-size: 11px;
    color: #4b5563;
    background: #f3f4f6;
    padding: 2px 4px;
    border-radius: 4px;
  }

  .key-actions {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
    margin-top: 16px;
    position: relative;
    z-index: 10;
  }

  .key-actions button {
    position: relative;
    z-index: 11;
  }

  .key-actions-row {
    display: flex;
    gap: 12px;
    margin-top: 16px;
    padding-top: 16px;
    border-top: 1px solid #e5e7eb;
  }

  .action-btn {
    flex: 1;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    padding: 10px 12px;
    background: #f3f4f6;
    color: #374151;
    border: 1px solid #e5e7eb;
    border-radius: 8px;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
    font-family: inherit;
  }

  .action-btn:hover {
    background: #e5e7eb;
    transform: translateY(-1px);
  }

  .action-btn.orange {
    background: #fef3c7;
    color: #92400e;
  }

  .action-btn.orange:hover {
    background: #fde68a;
  }

  .action-btn.red {
    background: #fee2e2;
    color: #991b1b;
  }

  .action-btn.red:hover {
    background: #fecaca;
  }

  .btn-primary {
    background: #2563eb;
    color: white;
    border: none;
    padding: 10px 20px;
    border-radius: 6px;
    cursor: pointer;
    font-size: 14px;
    font-weight: 500;
    transition: background 0.2s;
  }

  .btn-primary:hover {
    background: #1d4ed8;
  }

  .btn-primary:disabled {
    background: #93c5fd;
    cursor: not-allowed;
  }

  .btn-secondary {
    background: white;
    color: #374151;
    border: 1px solid #d1d5db;
    padding: 10px 20px;
    border-radius: 6px;
    cursor: pointer;
    font-size: 14px;
    font-weight: 500;
    transition: all 0.2s;
  }

  .btn-secondary:hover {
    background: #f9fafb;
    border-color: #9ca3af;
  }

  .btn-danger {
    background: white;
    color: #ef4444;
    border: 1px solid #fee2e2;
    padding: 8px 16px;
    border-radius: 6px;
    cursor: pointer;
    font-size: 13px;
    font-weight: 500;
    transition: all 0.2s;
  }

  .btn-danger:hover {
    background: #fef2f2;
    border-color: #fca5a5;
  }

  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(2px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal {
    background: white;
    border-radius: 12px;
    max-width: 500px;
    width: 90%;
    max-height: 90vh;
    overflow-y: auto;
    box-shadow:
      0 20px 25px -5px rgba(0, 0, 0, 0.1),
      0 10px 10px -5px rgba(0, 0, 0, 0.04);
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 20px 24px;
    border-bottom: 1px solid #e5e7eb;
  }

  .modal-header h2 {
    margin: 0;
    font-size: 20px;
    color: #111827;
    font-weight: 600;
  }

  .close-btn {
    background: none;
    border: none;
    font-size: 24px;
    cursor: pointer;
    color: #9ca3af;
    padding: 4px;
    border-radius: 4px;
    line-height: 1;
    transition: color 0.2s;
  }

  .close-btn:hover {
    color: #4b5563;
    background: #f3f4f6;
  }

  .modal-body {
    padding: 24px;
  }

  .form-group {
    margin-bottom: 20px;
  }

  .form-group label {
    display: block;
    margin-bottom: 8px;
    color: #374151;
    font-size: 14px;
    font-weight: 500;
  }

  .form-group input {
    width: 100%;
    padding: 10px 12px;
    border: 1px solid #d1d5db;
    border-radius: 6px;
    font-size: 14px;
    transition: border-color 0.2s;
  }

  .form-group input:focus {
    outline: none;
    border-color: #2563eb;
    ring: 2px solid #3b82f6;
  }

  .radio-group {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 12px;
  }

  .radio-card {
    display: flex;
    align-items: flex-start;
    gap: 10px;
    padding: 12px;
    border: 1px solid #d1d5db;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s;
    background: white;
  }

  .radio-card:hover {
    border-color: #3b82f6;
    background: #f0f9ff;
  }

  .radio-card.selected {
    border-color: #2563eb;
    background: #eff6ff;
    box-shadow: 0 0 0 2px rgba(37, 99, 235, 0.1);
  }

  .radio-card input {
    width: auto;
    margin-top: 3px;
    accent-color: #2563eb;
  }

  .radio-info {
    display: flex;
    flex-direction: column;
  }

  .radio-title {
    font-weight: 600;
    font-size: 14px;
    color: #1f2937;
  }

  .radio-desc {
    font-size: 12px;
    color: #6b7280;
  }

  .alert {
    background: #fffbeb;
    border: 1px solid #fcd34d;
    padding: 12px 16px;
    border-radius: 6px;
    color: #92400e;
    font-size: 13px;
    display: flex;
    align-items: flex-start;
    gap: 8px;
  }

  .modal-footer {
    padding: 16px 24px;
    border-top: 1px solid #e5e7eb;
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    background: #f9fafb;
    border-bottom-left-radius: 12px;
    border-bottom-right-radius: 12px;
  }

  .expiry-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 16px;
  }

  .date-time-inputs {
    display: flex;
    gap: 8px;
    flex: 1;
  }

  .input-date {
    flex: 2;
  }

  .input-time {
    flex: 1;
  }

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 14px;
    color: #4b5563;
    cursor: pointer;
    white-space: nowrap;
    user-select: none;
  }

  .checkbox-label input[type="checkbox"] {
    width: 16px;
    height: 16px;
    margin: 0;
    accent-color: #2563eb;
  }
</style>
