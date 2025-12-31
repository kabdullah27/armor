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
  import { Button, Input, Modal } from "$lib/components/ui";

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
    // Removed confirm dialog as per previous user request

    try {
      const result = await deleteKey(fingerprint);
      if (result.success) {
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
      const selected = await open({
        title: "Import PGP Key",
        filters: [{ name: "PGP Key", extensions: ["asc", "gpg", "txt"] }],
        multiple: false,
      });

      if (!selected) return;
      const path = Array.isArray(selected) ? selected[0] : selected;
      if (!path) return;

      loading = true;
      const content = await readTextFile(path);
      const res = await importKey(content);

      if (res.success) {
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
    try {
      const res = await exportKey(fingerprint, isPrivate);

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

      if (!savePath) return;

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

    try {
      generating = true;
      let expiryTimestamp = undefined;
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

      if (res.success) {
        showGenerateModal = false;
        resetForm();
        await listKeys().then((r) => {
          if (r.success && r.data) keys = r.data;
        });
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
</script>

<div style="max-width: 1200px; margin: 0 auto; padding: 32px 24px;">
  <!-- Header -->
  <div
    style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 32px;"
  >
    <h1 style="font-size: 24px; font-weight: 600; color: #111; margin: 0;">
      🔑 PGP Keys
    </h1>
    <div style="display: flex; gap: 12px;">
      <Button variant="secondary" on:click={handleImport}>📥 Import Key</Button>
      <Button
        variant="primary"
        on:click={() => {
          resetForm();
          showGenerateModal = true;
        }}>+ Generate New Key</Button
      >
    </div>
  </div>

  {#if loading}
    <div style="text-align: center; padding: 48px; color: #6b7280;">
      Loading keys...
    </div>
  {:else if error}
    <div
      style="padding: 24px; background: #fee2e2; color: #b91c1c; border-radius: 12px; border: 1px solid #fecaca;"
    >
      {error}
    </div>
  {:else if keys.length === 0}
    <div
      style="text-align: center; padding: 64px 24px; background: #f9fafb; border-radius: 16px; border: 1px dashed #e5e7eb;"
    >
      <p style="font-size: 16px; color: #374151; margin-bottom: 8px;">
        No keys found
      </p>
      <p style="font-size: 14px; color: #6b7280;">
        Click "Generate New Key" to create your first PGP key
      </p>
    </div>
  {:else}
    <div style="display: flex; flex-direction: column; gap: 16px;">
      {#each keys as key}
        <div
          style="background: white; border: 1px solid #e5e7eb; border-radius: 12px; padding: 24px; display: flex; justify-content: space-between; align-items: center; gap: 24px;"
        >
          <div style="display: flex; gap: 20px; align-items: flex-start;">
            <div
              style="width: 48px; height: 48px; background: {key.is_private
                ? '#eff6ff'
                : '#f3f4f6'}; display: flex; align-items: center; justify-content: center; font-size: 24px; border-radius: 12px;"
            >
              {key.is_private ? "🔐" : "🔓"}
            </div>
            <div>
              <h3
                style="font-size: 16px; font-weight: 600; color: #111827; margin: 0 0 4px 0;"
              >
                {key.user_id.name}
              </h3>
              <p style="font-size: 14px; color: #6b7280; margin: 0 0 8px 0;">
                {key.user_id.email}
              </p>
              <div
                style="display: flex; gap: 12px; font-size: 12px; color: #9ca3af;"
              >
                <span>🔑 {key.key_type}</span>
                <span>🔖 {key.fingerprint.substring(0, 16)}...</span>
                <span
                  >Created: {new Date(
                    key.created_at
                  ).toLocaleDateString()}</span
                >
                {#if key.expires_at}
                  <span style="color: #ef4444;"
                    >Expires: {new Date(
                      key.expires_at
                    ).toLocaleDateString()}</span
                  >
                {:else}
                  <span style="color: #059669;">Never Expires</span>
                {/if}
              </div>
            </div>
          </div>

          <div style="display: flex; gap: 8px;">
            <Button
              variant="primary"
              on:click={() => handleExport(key.fingerprint, false)}
              >📤 Public</Button
            >
            {#if key.is_private}
              <Button
                variant="primary"
                on:click={() => handleExport(key.fingerprint, true)}
                >🔐 Private</Button
              >
            {/if}
            <Button
              variant="danger"
              on:click={() => handleDelete(key.fingerprint)}>🗑️ Delete</Button
            >
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

{#if showGenerateModal}
  <Modal
    bind:show={showGenerateModal}
    title="Generate New PGP Key"
    maxWidth="600px"
  >
    <div style="display: flex; flex-direction: column; gap: 4px;">
      <Input
        id="name"
        label="Full Name"
        required={true}
        bind:value={name}
        placeholder="John Doe"
      />

      <Input
        id="email"
        label="Email Address"
        required={true}
        type="email"
        bind:value={email}
        placeholder="john@example.com"
      />

      <Input
        id="comment"
        label="Comment (optional)"
        bind:value={comment}
        placeholder="Work Key"
      />

      <div style="margin-bottom: 20px;">
        <label
          style="display: block; font-size: 14px; font-weight: 500; color: #374151; margin-bottom: 8px;"
          >Key Type</label
        >
        <div style="display: flex; gap: 12px;">
          <label
            style="flex: 1; padding: 16px; border: 2px solid {keyType ===
            'rsa4096'
              ? '#000'
              : '#e5e7eb'}; border-radius: 8px; cursor: pointer; display: flex; align-items: center; gap: 12px; transition: all 0.2s;"
          >
            <input
              type="radio"
              bind:group={keyType}
              value="rsa4096"
              style="width: 18px; height: 18px; cursor: pointer; accent-color: black;"
            />
            <div style="flex: 1;">
              <div style="font-size: 14px; font-weight: 600; color: #111827;">
                RSA 4096
              </div>
              <div style="font-size: 13px; color: #6b7280;">Recommended</div>
            </div>
          </label>
          <label
            style="flex: 1; padding: 16px; border: 2px solid {keyType ===
            'ed25519'
              ? '#000'
              : '#e5e7eb'}; border-radius: 8px; cursor: pointer; display: flex; align-items: center; gap: 12px; transition: all 0.2s;"
          >
            <input
              type="radio"
              bind:group={keyType}
              value="ed25519"
              style="width: 18px; height: 18px; cursor: pointer; accent-color: black;"
            />
            <div style="flex: 1;">
              <div style="font-size: 14px; font-weight: 600; color: #111827;">
                Ed25519
              </div>
              <div style="font-size: 13px; color: #6b7280;">Modern & Fast</div>
            </div>
          </label>
        </div>
      </div>

      <div style="margin-bottom: 20px;">
        <label
          style="display: block; font-size: 14px; font-weight: 500; color: #374151; margin-bottom: 8px;"
          >Key Expiration</label
        >
        <div style="display: flex; gap: 12px; align-items: center;">
          <input
            type="date"
            bind:value={datePart}
            disabled={neverExpire}
            min={new Date().toISOString().split("T")[0]}
            style="flex: 1; padding: 10px 14px; border: 1px solid #d1d5db; border-radius: 8px; font-size: 14px; color: #111827; outline: none; opacity: {neverExpire
              ? '0.5'
              : '1'};"
          />
          <input
            type="time"
            bind:value={timePart}
            disabled={neverExpire}
            style="padding: 10px 14px; border: 1px solid #d1d5db; border-radius: 8px; font-size: 14px; color: #111827; outline: none; opacity: {neverExpire
              ? '0.5'
              : '1'};"
          />
          <label
            style="display: flex; align-items: center; gap: 8px; white-space: nowrap; cursor: pointer;"
          >
            <input
              type="checkbox"
              bind:checked={neverExpire}
              style="width: 18px; height: 18px; cursor: pointer; accent-color: black;"
            />
            <span style="font-size: 14px; color: #374151;">Never expire</span>
          </label>
        </div>
      </div>

      <Input
        id="pass"
        label="Passphrase (optional)"
        type="password"
        bind:value={passphrase}
        placeholder="Leave empty for no protection"
      />

      <Input
        id="confirm"
        label="Confirm Passphrase"
        type="password"
        bind:value={confirmPass}
        placeholder="Re-enter passphrase"
      />

      <div
        style="padding: 12px 16px; background: #fffbeb; border: 1px solid #fde68a; border-radius: 8px; font-size: 14px; color: #92400e;"
      >
        <strong>Note:</strong> Using a passphrase is recommended to protect your
        private key.
      </div>
    </div>

    <svelte:fragment slot="footer">
      <Button
        variant="secondary"
        on:click={() => {
          showGenerateModal = false;
          resetForm();
        }}
      >
        Cancel
      </Button>
      <Button variant="primary" on:click={handleGenerate} disabled={generating}>
        {generating ? "Generating..." : "Generate Key"}
      </Button>
    </svelte:fragment>
  </Modal>
{/if}
