<script lang="ts">
  import { onMount } from "svelte";
  import { Button } from "$lib/components/ui";
  import { isFirstRun, completeOnboarding, setDbPath } from "$lib/api/settings";
  import { open, ask, message } from "@tauri-apps/plugin-dialog";

  let appVersion = "1.0.0 MVP";

  onMount(async () => {
    await checkFirstRun();
  });

  async function checkFirstRun() {
    try {
      const res = await isFirstRun();
      if (res.success && res.data) {
        // It is first run
        const answer = await ask(
          "Welcome to Armor! Would you like to set a custom location for your database storage? (Recommended for organized file management)",
          {
            title: "First Run Setup",
            kind: "info",
          }
        );

        if (answer) {
          const selected = await open({
            directory: true,
            multiple: false,
            title: "Select Database Folder",
          });

          if (selected) {
            const folderPath = Array.isArray(selected) ? selected[0] : selected;
            if (folderPath) {
              // The backend now handles directory inputs intelligently by appending 'armor.db'
              // so we can just pass the folder path directly.
              const setRes = await setDbPath(folderPath);
              if (setRes.success) {
                await message("Database location set successfully!", {
                  title: "Success",
                  kind: "info",
                });
              } else {
                await message(
                  "Failed to set database location: " + setRes.error,
                  { title: "Error", kind: "error" }
                );
              }
            }
          }
        }
        await completeOnboarding();
      }
    } catch (e) {
      console.error("First run check failed:", e);
    }
  }
</script>

<div
  style="max-width: 1200px; margin: 0 auto; padding: 48px 24px; text-align: center;"
>
  <div style="margin-bottom: 64px;">
    <h1
      style="font-size: 48px; font-weight: 800; color: #111; margin: 0 0 16px 0; letter-spacing: -1px;"
    >
      🔒 ARMOR
    </h1>
    <p
      style="font-size: 18px; color: #6b7280; max-width: 600px; margin: 0 auto;"
    >
      Secure, offline PGP encryption for the modern web.
      <br />
      <span
        style="font-size: 14px; color: #9ca3af; margin-top: 8px; display: inline-block;"
        >v{appVersion}</span
      >
    </p>
  </div>

  <div
    style="display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 32px; max-width: 1000px; margin: 0 auto;"
  >
    <!-- Key Management -->
    <div
      style="background: white; border: 1px solid #e5e7eb; border-radius: 16px; padding: 32px; text-align: left; transition: all 0.2s; display: flex; flex-direction: column;"
    >
      <div style="font-size: 40px; margin-bottom: 24px;">🔑</div>
      <h3
        style="font-size: 20px; font-weight: 600; color: #1f2937; margin: 0 0 8px 0;"
      >
        Key Management
      </h3>
      <p
        style="color: #6b7280; font-size: 15px; margin: 0 0 24px 0; flex: 1; line-height: 1.5;"
      >
        Generate secure RSA/Ed25519 keys, import existing ones, and manage your
        keychain.
      </p>
      <div>
        <a href="/keys" style="text-decoration: none;">
          <Button variant="primary" fullWidth>Manage Keys</Button>
        </a>
      </div>
    </div>

    <!-- Encryption -->
    <div
      style="background: white; border: 1px solid #e5e7eb; border-radius: 16px; padding: 32px; text-align: left; transition: all 0.2s; display: flex; flex-direction: column;"
    >
      <div style="font-size: 40px; margin-bottom: 24px;">🔒</div>
      <h3
        style="font-size: 20px; font-weight: 600; color: #1f2937; margin: 0 0 8px 0;"
      >
        Encryption
      </h3>
      <p
        style="color: #6b7280; font-size: 15px; margin: 0 0 24px 0; flex: 1; line-height: 1.5;"
      >
        Securely encrypt files for yourself or others using their public keys.
        Only they can open it.
      </p>
      <div>
        <a href="/encrypt" style="text-decoration: none;">
          <Button variant="secondary" fullWidth>Encrypt Files</Button>
        </a>
      </div>
    </div>

    <!-- Decryption -->
    <div
      style="background: white; border: 1px solid #e5e7eb; border-radius: 16px; padding: 32px; text-align: left; transition: all 0.2s; display: flex; flex-direction: column;"
    >
      <div style="font-size: 40px; margin-bottom: 24px;">🔓</div>
      <h3
        style="font-size: 20px; font-weight: 600; color: #1f2937; margin: 0 0 8px 0;"
      >
        Decryption
      </h3>
      <p
        style="color: #6b7280; font-size: 15px; margin: 0 0 24px 0; flex: 1; line-height: 1.5;"
      >
        Decrypt messages and files sent to you using your private key and
        passphrase.
      </p>
      <div>
        <a href="/decrypt" style="text-decoration: none;">
          <Button variant="secondary" fullWidth>Decrypt Files</Button>
        </a>
      </div>
    </div>
  </div>
</div>
