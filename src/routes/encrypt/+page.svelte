<script lang="ts">
  import { notify } from "$lib/utils/notify";
  import { onMount } from "svelte";
  import { open, save } from "@tauri-apps/plugin-dialog";
  import type { KeyMetadata } from "$lib/types/key";
  import { listKeys } from "$lib/api/keys";
  import { encryptFile } from "$lib/api/crypto";
  import { onFileDrop } from "$lib/utils/fileDrop";
  import { Button } from "$lib/components/ui";
  import { settings } from "$lib/stores/settings";

  // State
  let files: string[] = [];
  let recipients: KeyMetadata[] = [];
  let selectedRecipients: string[] = [];
  let loading = false;
  let processing = false;

  $: selectedName =
    recipients.find((k) => k.fingerprint === selectedRecipients[0])?.user_id
      .name || "Unknown";

  onMount(() => {
    loadRecipients();
    return onFileDrop(addFiles);
  });

  async function loadRecipients() {
    loading = true;
    const result = await listKeys();
    if (result.success && result.data) {
      recipients = result.data;
    } else {
      notify("Failed to load keys: " + result.error);
    }
    loading = false;
  }

  /** Adds paths, skipping duplicates. */
  function addFiles(paths: string[]) {
    files = [...new Set([...files, ...paths])];
  }

  async function selectFile() {
    try {
      const selected = await open({ multiple: true, directory: false });
      if (selected) addFiles(Array.isArray(selected) ? selected : [selected]);
    } catch (e) {
      console.error("File selection error:", e);
      notify("Error selecting file: " + e);
    }
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
  }

  function removeFile(index: number) {
    files = files.filter((_, i) => i !== index);
  }

  function selectKey(fingerprint: string) {
    selectedRecipients = [fingerprint];
  }

  async function handleEncrypt() {
    if (files.length === 0) return notify("Please select files to encrypt");
    if (selectedRecipients.length === 0)
      return notify("Please select at least one recipient");

    processing = true;
    const failed: string[] = [];
    const failedPaths: string[] = [];
    let done = 0;
    try {
      for (const filePath of files) {
        const filename = filePath.split(/[\\/]/).pop() || "encrypted";
        const savePath = await save({
          title: `Save Encrypted File for ${filename}`,
          defaultPath: `${filename}.gpg`,
          filters: [
            { name: "PGP Encrypted", extensions: ["gpg", "pgp", "asc"] },
          ],
        });
        if (!savePath) continue;

        const res = await encryptFile({
          inputPath: filePath,
          outputPath: savePath,
          recipientFingerprints: selectedRecipients,
          armor: savePath.toLowerCase().endsWith(".asc"),
        });
        if (res.success) done++;
        else {
          failed.push(`${filename}: ${res.error}`);
          failedPaths.push(filePath);
        }
      }

      if (failed.length > 0) {
        notify(`Encrypted ${done} file(s). Failed:\n${failed.join("\n")}`);
        // Keep only the failed files so the user can retry
        files = failedPaths;
      } else if (done > 0) {
        notify(`Encrypted ${done} file(s) successfully!`, "info");
        files = [];
        selectedRecipients = [];
      }
    } catch (e) {
      notify("Encryption error: " + e);
    } finally {
      processing = false;
    }
  }
</script>

<div
  style="max-width: 1000px; margin: 0 auto; padding: 24px; padding-bottom: 100px;"
>
  <div style="margin-bottom: 32px;">
    <h1
      style="font-size: 24px; font-weight: 600; color: #111; margin: 0 0 8px 0;"
    >
      🔒 Encrypt Files
    </h1>
    <p style="color: #6b7280; margin: 0;">
      Secure your files with PGP encryption
    </p>
  </div>

  <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 24px;">
    <!-- Step 1: File Selection -->
    <div
      style="background: white; border-radius: 12px; border: 1px solid #e5e7eb; overflow: hidden; display: flex; flex-direction: column; height: 100%;"
    >
      <div
        style="padding: 16px 20px; border-bottom: 1px solid #f3f4f6; display: flex; align-items: center; gap: 12px; background: #fafafa;"
      >
        <div
          style="background: #e0e7ff; color: #4338ca; width: 28px; height: 28px; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-weight: 600; font-size: 14px;"
        >
          1
        </div>
        <h2
          style="margin: 0; font-size: 16px; font-weight: 600; color: #1f2937;"
        >
          Files to Encrypt
        </h2>
      </div>

      <div
        style="flex: 1; padding: 24px; display: flex; flex-direction: column;"
        on:dragover={handleDragOver}
        role="region"
        aria-label="File upload area"
      >
        {#if files.length === 0}
          <div
            style="flex: 1; padding: 40px; text-align: center; border: 2px dashed #e5e7eb; border-radius: 12px; cursor: pointer; display: flex; flex-direction: column; align-items: center; justify-content: center; transition: all 0.2s;"
            on:click={selectFile}
            role="button"
            tabindex="0"
            on:keydown={(e) => e.key === "Enter" && selectFile()}
          >
            <div style="font-size: 48px; margin-bottom: 16px; opacity: 0.5;">
              📄
            </div>
            <h3
              style="margin: 0 0 8px 0; font-size: 16px; font-weight: 500; color: #111;"
            >
              Drag & Drop Files Here
            </h3>
            <p style="margin: 0 0 24px 0; font-size: 14px; color: #9ca3af;">
              or click to browse
            </p>
            <Button
              variant="secondary"
              on:click={(e) => {
                e.stopPropagation();
                selectFile();
              }}>Choose Files</Button
            >
          </div>
        {:else}
          <div
            style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px;"
          >
            <span style="font-size: 13px; color: #6b7280; font-weight: 500;"
              >{files.length} file(s) selected</span
            >
            <Button variant="secondary" on:click={selectFile}>+ Add More</Button
            >
          </div>
          <div
            style="flex: 1; overflow-y: auto; max-height: 400px; border: 1px solid #e5e7eb; border-radius: 8px;"
          >
            {#each files as file, i}
              <div
                style="padding: 12px 16px; border-bottom: 1px solid #f3f4f6; display: flex; justify-content: space-between; align-items: center; font-size: 14px;"
              >
                <div
                  style="display: flex; align-items: center; gap: 12px; overflow: hidden;"
                >
                  <span style="font-size: 16px;">📄</span>
                  <span
                    style="white-space: nowrap; overflow: hidden; text-overflow: ellipsis; color: #374151;"
                    title={file}
                  >
                    {file.split(/[\\/]/).pop()}
                  </span>
                </div>
                <button
                  style="background: none; border: none; color: #ef4444; cursor: pointer; font-size: 18px; padding: 4px;"
                  on:click|stopPropagation={() => removeFile(i)}>×</button
                >
              </div>
            {/each}
          </div>
        {/if}
      </div>
    </div>

    <!-- Step 2: Select Key -->
    <div
      style="background: white; border-radius: 12px; border: 1px solid #e5e7eb; overflow: hidden; display: flex; flex-direction: column; height: 100%;"
    >
      <div
        style="padding: 16px 20px; border-bottom: 1px solid #f3f4f6; display: flex; align-items: center; gap: 12px; background: #fafafa;"
      >
        <div
          style="background: #e0e7ff; color: #4338ca; width: 28px; height: 28px; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-weight: 600; font-size: 14px;"
        >
          2
        </div>
        <h2
          style="margin: 0; font-size: 16px; font-weight: 600; color: #1f2937;"
        >
          Select Recipient
        </h2>
      </div>

      <div
        style="flex: 1; padding: 24px; display: flex; flex-direction: column; overflow: hidden;"
      >
        {#if loading}
          <div
            style="flex: 1; display: flex; align-items: center; justify-content: center; color: #6b7280;"
          >
            Loading keys...
          </div>
        {:else if recipients.length === 0}
          <div
            style="flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; text-align: center; color: #6b7280; padding: 20px;"
          >
            <div style="font-size: 48px; margin-bottom: 16px; opacity: 0.5;">
              📭
            </div>
            <h3 style="margin: 0 0 8px 0; font-size: 16px; color: #374151;">
              No Keys Found
            </h3>
            <p style="margin: 0 0 24px 0; font-size: 14px; max-width: 200px;">
              You need at least one public key to encrypt files.
            </p>
            <a href="/keys" style="text-decoration: none;">
              <Button variant="primary">Generate or Import Keys</Button>
            </a>
          </div>
        {:else}
          <div
            style="flex: 1; overflow-y: auto; border: 1px solid #e5e7eb; border-radius: 8px;"
          >
            {#each recipients as key}
              <!-- svelte-ignore a11y-click-events-have-key-events -->
              <div
                style="display: flex; align-items: center; gap: 12px; padding: 12px 16px; cursor: pointer; border-bottom: 1px solid #f3f4f6; transition: background 0.1s; {selectedRecipients.includes(
                  key.fingerprint
                )
                  ? 'background: #eff6ff; border-left: 3px solid #3b82f6;'
                  : 'border-left: 3px solid transparent;'}"
                on:click={() => selectKey(key.fingerprint)}
                role="button"
                tabindex="0"
              >
                <span style="font-size: 20px;"
                  >{key.is_private ? "🔐" : "🔑"}</span
                >
                <div style="display: flex; flex-direction: column;">
                  <span
                    style="font-weight: 500; font-size: 14px; color: #1f2937;"
                    >{key.user_id.name}</span
                  >
                  <span style="font-size: 12px; color: #6b7280;"
                    >{key.fingerprint.substring(0, 8)}...</span
                  >
                </div>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    </div>
  </div>

  <div
    style="position: fixed; bottom: 0; left: {$settings.navbarPosition ===
    'left'
      ? '240px'
      : '0'}; right: 0; background: white; padding: 20px 32px; border-top: 1px solid #e5e7eb; display: flex; justify-content: space-between; align-items: center; box-shadow: 0 -4px 6px -1px rgba(0, 0, 0, 0.05); z-index: 100;"
  >
    <div style="font-size: 14px; color: #374151;">
      Encrypting <strong>{files.length}</strong> file(s) for
      <strong
        >{selectedRecipients.length > 0
          ? selectedName
          : "No recipient"}</strong
      >
    </div>
    <Button
      variant="primary"
      disabled={files.length === 0 ||
        selectedRecipients.length === 0 ||
        processing}
      on:click={handleEncrypt}
    >
      {processing ? "Encrypting..." : "🔒 Encrypt Files"}
    </Button>
  </div>
</div>
