<script lang="ts">
  import { Button } from "$lib/components/ui";

  let appVersion = "1.0.0 MVP";
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
