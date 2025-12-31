<script lang="ts">
  import { onMount } from "svelte";
  import { getDbPath, setDbPath, backupDb, restoreDb } from "$lib/api/settings";
  import { open, save } from "@tauri-apps/plugin-dialog";
  import { settings } from "$lib/stores/settings";

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
      console.log("getDbPath result:", res);
      if (res.success && res.data) {
        dbPath = res.data;
        console.log("DB Path loaded:", dbPath);
      }
    } catch (e) {
      console.error("Failed to load DB path:", e);
    } finally {
      loading = false;
    }
  }

  async function handleSelectFolder() {
    try {
      console.log("Opening folder selection dialog...");
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Select Database Folder",
      });

      console.log("Dialog result:", selected);

      if (selected) {
        // selected is string or string[]
        const folderPath = Array.isArray(selected) ? selected[0] : selected;
        if (!folderPath) {
          alert("No folder path received!");
          return;
        }

        console.log("Selected folder:", folderPath);
        // Correct path construction for all OS
        // Note: In a real cross-platform app, we'd want to handle separators carefully.
        // But for MVP on Mac/Linux/Windows, '/' usually works or is normalized by Tauri/Rust.
        const separator = navigator.userAgent.includes("Win") ? "\\" : "/";
        const newDbPath = folderPath.endsWith(separator)
          ? folderPath + "armor.db"
          : folderPath + separator + "armor.db";

        console.log("New DB Path:", newDbPath);

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
          console.log("DB path updated successfully, reloading settings...");
          console.log("Previous DB path:", dbPath);

          // Reload the DB path to show the new location
          await loadSettings();

          console.log("After reload, new DB path:", dbPath);
          alert("Database location updated! Please restart the application.");
        } else {
          console.error("Set DB Path failed:", res.error);
          alert("Failed to update location: " + res.error);
        }
        processing = false;
      }
    } catch (e) {
      console.error("Error regarding folder selection:", e);
      alert("Error selecting folder: " + e);
      processing = false;
    }
  }

  async function handleReset() {
    if (
      !confirm(
        "Reset to default database location?\n\nApplication will need to restart."
      )
    )
      return;
    alert(
      "Reset functionality not fully implemented in backend yet. Please manually select the default folder."
    );
  }

  async function handleBackup() {
    try {
      console.log("Starting backup...");
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

      console.log("Backup target:", savePath);
      processing = true;
      const res = await backupDb(savePath);

      if (res.success) {
        alert("Database backed up successfully!");
      } else {
        console.error("Backup failed:", res.error);
        alert("Backup failed: " + res.error);
      }
    } catch (e) {
      console.error("Backup error:", e);
      alert("Error: " + e);
    } finally {
      processing = false;
    }
  }

  async function handleRestore() {
    if (!confirm("Restoring will OVERWRITE your current database. Continue?"))
      return;

    try {
      console.log("Starting restore...");
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

      console.log("Restore source:", path);
      processing = true;
      const res = await restoreDb(path);

      if (res.success) {
        alert("Database restored! Please restart the application.");
      } else {
        console.error("Restore failed:", res.error);
        alert("Restore failed: " + res.error);
      }
    } catch (e) {
      console.error("Restore error:", e);
      alert("Error: " + e);
    } finally {
      processing = false;
    }
  }
</script>

<div class="container">
  <h1>⚙️ Settings</h1>

  <section class="card">
    <h2>Database Storage</h2>
    <p class="desc">Manage where your PGP keys and data are stored.</p>

    <div class="setting-row">
      <div class="setting-info">
        <label>Current Location</label>
        <code class="path-display">{loading ? "Loading..." : dbPath}</code>
      </div>

      <div class="setting-actions">
        <button
          class="btn-primary"
          on:click={handleSelectFolder}
          disabled={processing}
        >
          {processing ? "Updating..." : "Change Location"}
        </button>
      </div>
    </div>

    <div class="info-box">
      <p>
        <strong>Note:</strong> Changing the location will require a restart. The
        new location will start with a fresh database if one doesn't exist.
      </p>
    </div>

    <div class="setting-row" style="margin-top: 24px;">
      <div class="setting-info">
        <label>Database Maintenance</label>
        <p class="sub-label">
          Backup your data or restore from a previous backup.
        </p>
      </div>
      <div class="setting-actions">
        <button
          class="btn-secondary"
          on:click={handleBackup}
          disabled={processing}>Backup</button
        >
        <button
          class="btn-secondary"
          on:click={handleRestore}
          disabled={processing}>Restore</button
        >
      </div>
    </div>
  </section>

  <section class="card" style="margin-top: 24px;">
    <h2>Appearance</h2>
    <p class="desc">Customize the look and feel of Armor.</p>

    <div class="setting-row">
      <div class="setting-info">
        <label>Navigation Position</label>
        <p class="sub-label">
          Choose between a sidebar or a top navigation bar.
        </p>
      </div>
      <div class="setting-actions">
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <!-- svelte-ignore a11y-no-static-element-interactions -->
        <div class="toggle-group">
          <div
            class="toggle-option"
            class:active={$settings.navbarPosition === "left"}
            on:click={() => ($settings.navbarPosition = "left")}
          >
            Sidebar (Left)
          </div>
          <div
            class="toggle-option"
            class:active={$settings.navbarPosition === "top"}
            on:click={() => ($settings.navbarPosition = "top")}
          >
            Topbar (Top)
          </div>
        </div>
      </div>
    </div>

    <div class="setting-row">
      <div class="setting-info">
        <label>Navigation Color</label>
        <p class="sub-label">Custom background color for the navigation bar.</p>
      </div>
      <div class="setting-actions">
        <input
          type="color"
          bind:value={$settings.navbarColor}
          class="color-picker"
        />
        <span class="color-value">{$settings.navbarColor}</span>
        <button
          class="btn-secondary"
          on:click={() => ($settings.navbarColor = "#ffffff")}>Reset</button
        >
      </div>
    </div>
  </section>
</div>

<style>
  .container {
    max-width: 800px;
    margin: 0 auto;
    padding: 24px;
  }

  h1 {
    color: #1f2937;
    margin-bottom: 32px;
  }

  .card {
    background: white;
    padding: 24px;
    border-radius: 12px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
    border: 1px solid #e5e7eb;
  }

  h2 {
    font-size: 18px;
    color: #111827;
    margin: 0 0 8px 0;
  }

  .desc {
    color: #6b7280;
    margin-bottom: 24px;
    font-size: 14px;
  }

  .sub-label {
    font-size: 12px;
    color: #9ca3af;
    margin: 0;
  }

  .setting-row {
    margin-bottom: 24px;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .setting-info {
    margin-bottom: 0;
    flex: 1;
  }

  label {
    display: block;
    font-size: 14px;
    font-weight: 500;
    color: #374151;
    margin-bottom: 2px;
  }

  .path-display {
    display: block;
    background: #f3f4f6;
    padding: 12px;
    border-radius: 6px;
    font-family: monospace;
    font-size: 13px;
    color: #4b5563;
    word-break: break-all;
    border: 1px solid #e5e7eb;
    margin-top: 8px;
  }

  .setting-actions {
    display: flex;
    gap: 12px;
    align-items: center;
  }

  .btn-primary {
    background: #2563eb;
    color: white;
    border: none;
    padding: 8px 16px;
    border-radius: 6px;
    font-weight: 500;
    cursor: pointer;
    font-size: 14px;
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
    background: #fff;
    border: 1px solid #d1d5db;
    color: #374151;
    padding: 6px 12px;
    border-radius: 6px;
    font-size: 13px;
    cursor: pointer;
  }

  .info-box {
    background: #eff6ff;
    border: 1px solid #dbeafe;
    padding: 12px;
    border-radius: 6px;
    font-size: 13px;
    color: #1e40af;
  }

  .info-box p {
    margin: 0;
  }

  /* Toggle Group */
  .toggle-group {
    display: flex;
    background: #f3f4f6;
    padding: 4px;
    border-radius: 8px;
    border: 1px solid #e5e7eb;
  }

  .toggle-option {
    padding: 6px 16px;
    border-radius: 6px;
    font-size: 13px;
    font-weight: 500;
    color: #6b7280;
    cursor: pointer;
    transition: all 0.2s;
  }

  .toggle-option.active {
    background: white;
    color: #2563eb;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
  }

  .color-picker {
    width: 40px;
    height: 32px;
    padding: 0;
    border: 1px solid #d1d5db;
    border-radius: 4px;
    cursor: pointer;
  }

  .color-value {
    font-family: monospace;
    font-size: 13px;
    color: #6b7280;
  }
</style>
