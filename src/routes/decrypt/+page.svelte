<script lang="ts">
  import { onMount } from "svelte";
  import { open, save, message } from "@tauri-apps/plugin-dialog";
  import { invoke } from "@tauri-apps/api/core";
  import { listKeys } from "$lib/api/keys";
  import { Button, Input } from "$lib/components/ui";
  import { settings } from "$lib/stores/settings";

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
      console.log("Decrypt page loaded keys:", res);
      if (res.success && res.data) {
        // Filter for private keys (ones we can use to decrypt)
        console.log("All keys:", res.data);
        myKeys = res.data.filter((k: any) => k.is_private);
        console.log("Private keys filtered:", myKeys);
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
        // Handle string or array return types from open()
        const path = Array.isArray(selected) ? selected[0] : selected;
        // In some versions/configs it might be an object, but usually it's a path string
        // We'll safely cast or check
        if (typeof path === "string") {
          file = path;
        } else if (path && "path" in (path as any)) {
          file = (path as any).path;
        } else {
          // Fallback or error
          file = String(path);
        }
      }
    } catch (e) {
      console.error(e);
      await message("Error selecting file: " + e, {
        title: "Error",
        kind: "error",
      });
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

      const res: any = await invoke("decrypt_file_cmd", {
        inputPath: file,
        outputPath: savePath,
        passphrase: passphrase,
        targetFingerprint: selectedKeyFingerprint, // Pass the selected key fingerprint
      });

      if (res.success) {
        result = `File decrypted successfully to: ${savePath}`;
        await message("Decryption successful!", {
          title: "Success",
          kind: "info",
        });
        file = "";
      } else {
        await message("Decryption failed: " + res.error, {
          title: "Error",
          kind: "error",
        });
      }
    } catch (e) {
      await message("Error: " + e, { title: "Error", kind: "error" });
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
      🔓 Decrypt File
    </h1>
    <p style="color: #6b7280; margin: 0;">
      Decrypt PGP encrypted files using your private keys
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
          Encrypted File
        </h2>
      </div>

      <div
        style="flex: 1; padding: 24px; display: flex; flex-direction: column;"
        on:drop={handleDrop}
        on:dragover={handleDragOver}
        role="region"
        aria-label="File upload area"
      >
        {#if !file}
          <div
            style="flex: 1; padding: 40px; text-align: center; border: 2px dashed #e5e7eb; border-radius: 12px; cursor: pointer; display: flex; flex-direction: column; align-items: center; justify-content: center; transition: all 0.2s;"
            on:click={selectFile}
            role="button"
            tabindex="0"
            on:keydown={(e) => e.key === "Enter" && selectFile()}
          >
            <div style="font-size: 40px; margin-bottom: 16px; opacity: 0.5;">
              🔐
            </div>
            <h3
              style="margin: 0 0 8px 0; font-size: 16px; font-weight: 500; color: #111;"
            >
              Drag & Drop Encrypted File
            </h3>
            <p style="margin: 0 0 24px 0; font-size: 14px; color: #9ca3af;">
              or click to browse
            </p>
            <Button
              variant="secondary"
              on:click={(e) => {
                e.stopPropagation();
                selectFile();
              }}>Choose File</Button
            >
          </div>
        {:else}
          <div
            style="padding: 12px 16px; background: #f3f4f6; border-radius: 6px; display: flex; justify-content: space-between; align-items: center; gap: 12px;"
          >
            <div
              style="display: flex; align-items: center; gap: 12px; overflow: hidden;"
            >
              <span style="font-size: 16px;">🔒</span>
              <span
                style="white-space: nowrap; overflow: hidden; text-overflow: ellipsis; color: #374151; font-size: 14px;"
                title={file}
              >
                {file.split(/[\\/]/).pop()}
              </span>
            </div>
            <button
              style="background: none; border: none; color: #ef4444; cursor: pointer; font-size: 18px; padding: 4px;"
              on:click={removeFile}>×</button
            >
          </div>
        {/if}
      </div>
    </div>

    <!-- Step 2: Key & Passphrase -->
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
          Unlock Key
        </h2>
      </div>

      <div
        style="flex: 1; padding: 24px; display: flex; flex-direction: column;"
      >
        {#if loadingKeys}
          <div
            style="flex: 1; display: flex; align-items: center; justify-content: center; color: #6b7280;"
          >
            Loading keys...
          </div>
        {:else if myKeys.length === 0}
          <div
            style="flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; text-align: center; color: #6b7280; padding: 20px;"
          >
            <div style="font-size: 32px; margin-bottom: 16px; opacity: 0.5;">
              🚫
            </div>
            <h3 style="margin: 0 0 8px 0; font-size: 16px; color: #374151;">
              No Private Keys
            </h3>
            <p style="margin: 0 0 24px 0; font-size: 14px; max-width: 200px;">
              You need a private key to decrypt files.
            </p>
            <a href="/keys" style="text-decoration: none;">
              <Button variant="primary">Generate Key</Button>
            </a>
          </div>
        {:else}
          <div style="margin-bottom: 24px;">
            <label
              style="display: block; font-weight: 500; margin-bottom: 8px; font-size: 14px; color: #374151;"
              >Select Key (for context)</label
            >
            <div
              style="max-height: 200px; overflow-y: auto; border: 1px solid #e5e7eb; border-radius: 8px;"
            >
              {#each myKeys as key}
                <!-- svelte-ignore a11y-click-events-have-key-events -->
                <div
                  style="display: flex; align-items: center; gap: 12px; padding: 10px 14px; cursor: pointer; border-bottom: 1px solid #f3f4f6; transition: background 0.1s; {selectedKeyFingerprint ===
                  key.fingerprint
                    ? 'background: #eff6ff; border-left: 3px solid #3b82f6;'
                    : 'border-left: 3px solid transparent;'}"
                  on:click={() => (selectedKeyFingerprint = key.fingerprint)}
                  role="button"
                  tabindex="0"
                >
                  <span style="font-size: 18px;">🔑</span>
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
          </div>

          <Input
            label="Passphrase"
            type="password"
            bind:value={passphrase}
            placeholder="Enter passphrase for selected key"
          />
          <p
            style="font-size: 12px; color: #9ca3af; margin-top: -12px; margin-bottom: 0;"
          >
            Leave empty if key has no passphrase
          </p>
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
      {#if result}
        <span style="color: #059669; font-weight: 500;">{result}</span>
      {:else}
        Ready to decrypt
      {/if}
    </div>
    <Button
      variant="primary"
      disabled={!file ||
        (myKeys.length > 0 && !selectedKeyFingerprint) ||
        processing}
      on:click={handleDecrypt}
    >
      {processing ? "Decrypting..." : "🔓 Decrypt File"}
    </Button>
  </div>
</div>
