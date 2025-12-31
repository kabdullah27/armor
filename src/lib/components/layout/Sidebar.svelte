<script lang="ts">
  import { page } from "$app/stores";

  export let customColor = "#ffffff"; // Default white bg
</script>

<aside class="sidebar" style="background-color: {customColor}">
  <div class="logo">
    <img src="/icons/128x128.png" alt="Armor Logo" width="40" height="40" />
    <span class="brand">ARMOR</span>
  </div>

  <nav class="nav">
    <a href="/" class="nav-item" class:active={$page.url.pathname === "/"}>
      <span class="icon">🏠</span>
      <span class="label">Dashboard</span>
    </a>
    <a
      href="/keys"
      class="nav-item"
      class:active={$page.url.pathname.startsWith("/keys")}
    >
      <span class="icon">🔑</span>
      <span class="label">Keys</span>
    </a>
    <a
      href="/encrypt"
      class="nav-item"
      class:active={$page.url.pathname.startsWith("/encrypt")}
    >
      <span class="icon">🔒</span>
      <span class="label">Encrypt</span>
    </a>
    <a
      href="/decrypt"
      class="nav-item"
      class:active={$page.url.pathname.startsWith("/decrypt")}
    >
      <span class="icon">🔓</span>
      <span class="label">Decrypt</span>
    </a>
    <div class="spacer"></div>
    <a
      href="/settings"
      class="nav-item"
      class:active={$page.url.pathname.startsWith("/settings")}
    >
      <span class="icon">⚙️</span>
      <span class="label">Settings</span>
    </a>
  </nav>
</aside>

<style>
  .sidebar {
    width: 240px;
    height: 100vh;
    display: flex;
    flex-direction: column;
    border-right: 1px solid #e5e7eb;
    background: white;
    position: fixed;
    left: 0;
    top: 0;
    z-index: 50;
  }

  .logo {
    padding: 24px;
    display: flex;
    align-items: center;
    gap: 12px;
    font-weight: 700;
    font-size: 20px;
    color: #1f2937;
    border-bottom: 1px solid #f3f4f6;
  }

  .nav {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding: 16px;
    gap: 8px;
    overflow-y: auto;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 16px;
    border-radius: 8px;
    color: #4b5563;
    text-decoration: none;
    font-weight: 500;
    transition: all 0.2s;
  }

  .nav-item:hover {
    background: #f3f4f6;
    color: #111827;
  }

  .nav-item.active {
    background: #eff6ff;
    color: #2563eb;
  }

  .icon {
    font-size: 18px;
  }

  .spacer {
    flex: 1;
  }
</style>
