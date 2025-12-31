<script lang="ts">
  import { onMount } from "svelte";

  let appVersion = "1.0.0 MVP";
</script>

<div class="container">
  <div class="header">
    <h1>🔒 ARMOR - Offline PGP</h1>
    <p>Version {appVersion}</p>
  </div>

  <div class="content">
    <h2>Welcome to ARMOR MVP</h2>
    <p>Offline PGP desktop application</p>

    <div class="features">
      <div class="feature-card">
        <h3>🔑 Key Management</h3>
        <p>Generate, import, and manage PGP keys</p>
        <a href="/keys" class="btn">Manage Keys</a>
      </div>

      <div class="feature-card">
        <h3>🔒 Encryption</h3>
        <p>Encrypt files for recipients</p>
        <a href="/encrypt" class="btn">Encrypt Files</a>
      </div>

      <div class="feature-card">
        <h3>🔓 Decryption</h3>
        <p>Decrypt and verify files</p>
        <a href="/decrypt" class="btn">Decrypt Files</a>
      </div>
    </div>
  </div>
</div>

<style>
  .container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 24px;
  }

  .header {
    text-align: center;
    margin-bottom: 32px;
  }

  .header h1 {
    font-size: 32px;
    color: #1f2937;
    margin-bottom: 8px;
  }

  .header p {
    color: #6b7280;
  }

  .nav {
    display: flex;
    gap: 16px;
    margin-bottom: 32px;
    padding: 16px;
    background: white;
    border-radius: 8px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  }

  .nav-link {
    padding: 8px 16px;
    text-decoration: none;
    color: #4b5563;
    border-radius: 6px;
    transition: all 0.2s;
  }

  .nav-link:hover {
    background: #f3f4f6;
    color: #1f2937;
  }

  .content {
    background: white;
    padding: 32px;
    border-radius: 8px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  }

  .content h2 {
    color: #1f2937;
    margin-bottom: 8px;
  }

  .content > p {
    color: #6b7280;
    margin-bottom: 32px;
  }

  .features {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 16px;
  }

  .feature-card {
    padding: 24px;
    border: 2px solid #e5e7eb;
    border-radius: 8px;
    transition: all 0.2s;
  }

  .feature-card:not(.disabled):hover {
    border-color: #3b82f6;
    box-shadow: 0 4px 6px rgba(59, 130, 246, 0.1);
  }

  .feature-card.disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .feature-card h3 {
    font-size: 18px;
    margin-bottom: 8px;
    color: #1f2937;
  }

  .feature-card p {
    color: #6b7280;
    margin-bottom: 16px;
  }

  .btn {
    display: inline-block;
    padding: 8px 16px;
    background: #3b82f6;
    color: white;
    text-decoration: none;
    border-radius: 6px;
    transition: all 0.2s;
  }

  .btn:hover {
    background: #2563eb;
  }
</style>
