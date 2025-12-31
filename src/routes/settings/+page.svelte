<script lang="ts">
  import { onMount } from "svelte";
  import { getDbPath, setDbPath, backupDb, restoreDb } from "$lib/api/settings";
  import { open, save } from "@tauri-apps/plugin-dialog";
  import { settings } from "$lib/stores/settings";
  import { Button } from "$lib/components/ui";

  let dbPath = "";
  let loading = true;
  let processing = false;

  onMount(async () => {
    await loadSettings();
  });

  async function loadSettings() {
    loading = true;
    try {
      const res = await getDbPath();
      if (res.success && res.data) {
        dbPath = res.data;
      }
    } catch (e) {
      console.error("Failed to load DB path:", e);
    } finally {
      loading = false;
    }
  }

  async function handleSelectFolder() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Select Database Folder",
      });

      if (selected) {
        const folderPath = Array.isArray(selected) ? selected[0] : selected;
        if (!folderPath) return;

        const separator = navigator.userAgent.includes("Win") ? "\\" : "/";
        const newDbPath = folderPath.endsWith(separator)
          ? folderPath + "armor.db"
          : folderPath + separator + "armor.db";

        if (
          !confirm(
            `Switch database to: ${newDbPath}?\n\nApplication will need to restart.`
          )
        ) {
          return;
        }

        processing = true;
        console.log("Calling setDbPath with:", newDbPath);
        const res = await setDbPath(newDbPath);
        console.log("setDbPath result:", res);

        if (res.success) {
          console.log("DB path updated successfully.");
          // Optimistically update the UI since backend config might take a restart to reload
          dbPath = newDbPath;

          alert("Database location updated! Please restart the application.");
        } else {
          console.error("Set DB Path failed:", res.error);
          alert("Failed to update location: " + res.error);
        }
        processing = false;
      }
    } catch (e) {
      alert("Error selecting folder: " + e);
      processing = false;
    }
  }

  async function handleBackup() {
    try {
      const savePath = await save({
        title: "Backup Database",
        defaultPath: "armor_backup.db",
        filters: [
          {
            name: "SQLite Database",
            extensions: ["db"],
          },
        ],
      });

      if (!savePath) return;

      processing = true;
      const res = await backupDb(savePath);

      if (res.success) {
        alert("Database backed up successfully!");
      } else {
        alert("Backup failed: " + res.error);
      }
    } catch (e) {
      alert("Error: " + e);
    } finally {
      processing = false;
    }
  }

  async function handleRestore() {
    if (!confirm("Restoring will OVERWRITE your current database. Continue?"))
      return;

    try {
      const selected = await open({
        title: "Select Backup File",
        multiple: false,
        filters: [
          {
            name: "SQLite Database",
            extensions: ["db", "sqlite"],
          },
        ],
      });

      if (!selected) return;
      const path = Array.isArray(selected) ? selected[0] : selected;
      if (!path) return;

      processing = true;
      const res = await restoreDb(path);

      if (res.success) {
        alert("Database restored! Please restart the application.");
      } else {
        alert("Restore failed: " + res.error);
      }
    } catch (e) {
      alert("Error: " + e);
    } finally {
      processing = false;
    }
  }
</script>

<div style="max-width: 800px; margin: 0 auto; padding: 24px;">
  <div style="margin-bottom: 32px;">
    <h1 style="font-size: 24px; font-weight: 600; color: #111; margin: 0;">
      ⚙️ Settings
    </h1>
  </div>

  <div
    style="background: white; border-radius: 12px; border: 1px solid #e5e7eb; padding: 24px; margin-bottom: 24px;"
  >
    <h2
      style="font-size: 18px; font-weight: 600; color: #111827; margin: 0 0 8px 0;"
    >
      Database Storage
    </h2>
    <p style="color: #6b7280; margin: 0 0 24px 0; font-size: 14px;">
      Manage where your PGP keys and data are stored.
    </p>

    <div
      style="display: flex; gap: 24px; align-items: flex-start; margin-bottom: 24px; flex-wrap: wrap;"
    >
      <div style="flex: 1; min-width: 250px;">
        <label
          style="display: block; font-size: 14px; font-weight: 500; color: #374151; margin-bottom: 8px;"
          >Current Location</label
        >
        <code
          style="display: block; background: #f3f4f6; padding: 12px; border-radius: 6px; font-family: monospace; font-size: 13px; color: #4b5563; word-break: break-all; border: 1px solid #e5e7eb;"
        >
          {loading ? "Loading..." : dbPath}
        </code>
      </div>

      <div style="margin-top: 28px;">
        <Button
          variant="primary"
          on:click={handleSelectFolder}
          disabled={processing}
        >
          {processing ? "Updating..." : "Change Location"}
        </Button>
      </div>
    </div>

    <div
      style="background: #eff6ff; border: 1px solid #dbeafe; padding: 12px; border-radius: 8px; font-size: 13px; color: #1e40af; margin-bottom: 24px;"
    >
      <p style="margin: 0;">
        <strong>Note:</strong> Changing the location will require a restart. The
        new location will start with a fresh database if one doesn't exist.
      </p>
    </div>

    <div
      style="border-top: 1px solid #e5e7eb; padding-top: 24px; display: flex; justify-content: space-between; align-items: center;"
    >
      <div>
        <label
          style="display: block; font-size: 14px; font-weight: 500; color: #374151; margin-bottom: 4px;"
          >Database Maintenance</label
        >
        <p style="font-size: 12px; color: #9ca3af; margin: 0;">
          Backup your data or restore from a previous backup.
        </p>
      </div>
      <div style="display: flex; gap: 8px;">
        <Button
          variant="secondary"
          on:click={handleBackup}
          disabled={processing}>Backup</Button
        >
        <Button
          variant="secondary"
          on:click={handleRestore}
          disabled={processing}>Restore</Button
        >
      </div>
    </div>
  </div>

  <div
    style="background: white; border-radius: 12px; border: 1px solid #e5e7eb; padding: 24px;"
  >
    <h2
      style="font-size: 18px; font-weight: 600; color: #111827; margin: 0 0 8px 0;"
    >
      Appearance
    </h2>
    <p style="color: #6b7280; margin: 0 0 24px 0; font-size: 14px;">
      Customize the look and feel of Armor.
    </p>

    <div
      style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px;"
    >
      <div>
        <label
          style="display: block; font-size: 14px; font-weight: 500; color: #374151; margin-bottom: 4px;"
          >Navigation Position</label
        >
        <p style="font-size: 12px; color: #9ca3af; margin: 0;">
          Choose between a sidebar or a top navigation bar.
        </p>
      </div>
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <!-- svelte-ignore a11y-no-static-element-interactions -->
      <div
        style="display: flex; background: #f3f4f6; padding: 4px; border-radius: 8px; border: 1px solid #e5e7eb;"
      >
        <div
          style="padding: 6px 16px; border-radius: 6px; font-size: 13px; font-weight: 500; cursor: pointer; transition: all 0.2s; {$settings.navbarPosition ===
          'left'
            ? 'background: white; color: #2563eb; box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);'
            : 'color: #6b7280;'}"
          on:click={() => ($settings.navbarPosition = "left")}
        >
          Sidebar (Left)
        </div>
        <div
          style="padding: 6px 16px; border-radius: 6px; font-size: 13px; font-weight: 500; cursor: pointer; transition: all 0.2s; {$settings.navbarPosition ===
          'top'
            ? 'background: white; color: #2563eb; box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);'
            : 'color: #6b7280;'}"
          on:click={() => ($settings.navbarPosition = "top")}
        >
          Topbar (Top)
        </div>
      </div>
    </div>

    <div
      style="display: flex; justify-content: space-between; align-items: center;"
    >
      <div>
        <label
          style="display: block; font-size: 14px; font-weight: 500; color: #374151; margin-bottom: 4px;"
          >Navigation Color</label
        >
        <p style="font-size: 12px; color: #9ca3af; margin: 0;">
          Custom background color for the navigation bar.
        </p>
      </div>
      <div style="display: flex; gap: 12px; align-items: center;">
        <input
          type="color"
          bind:value={$settings.navbarColor}
          style="width: 40px; height: 32px; padding: 0; border: 1px solid #d1d5db; border-radius: 4px; cursor: pointer;"
        />
        <span style="font-family: monospace; font-size: 13px; color: #6b7280;"
          >{$settings.navbarColor}</span
        >
        <Button
          variant="secondary"
          on:click={() => ($settings.navbarColor = "#ffffff")}>Reset</Button
        >
      </div>
    </div>
  </div>
</div>
