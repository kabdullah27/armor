<script lang="ts">
  import { page } from "$app/stores";

  export let customColor = "#ffffff";
</script>

<header class="topbar" style="background-color: {customColor}">
  <div class="logo">
    <img src="/icons/128x128.png" alt="Armor Logo" width="32" height="32" />
    <span class="brand">ARMOR</span>
  </div>

  <nav class="nav">
    <a href="/" class="nav-item" class:active={$page.url.pathname === "/"}
      >Dashboard</a
    >
    <a
      href="/keys"
      class="nav-item"
      class:active={$page.url.pathname.startsWith("/keys")}>Keys</a
    >
    <a
      href="/encrypt"
      class="nav-item"
      class:active={$page.url.pathname.startsWith("/encrypt")}>Encrypt</a
    >
    <a
      href="/decrypt"
      class="nav-item"
      class:active={$page.url.pathname.startsWith("/decrypt")}>Decrypt</a
    >
    <a
      href="/settings"
      class="nav-item"
      class:active={$page.url.pathname.startsWith("/settings")}>Settings</a
    >
  </nav>
</header>

<style>
  .topbar {
    display: flex;
    align-items: center;
    padding: 0 24px;
    height: 64px;
    background: white;
    border-bottom: 1px solid #e5e7eb;
    justify-content: space-between;
    position: relative; /* Needed for absolute centering */
  }

  .logo {
    display: flex;
    align-items: center;
    gap: 12px;
    font-weight: 700;
    font-size: 18px;
    color: #1f2937;
    z-index: 10; /* Ensure logo stays on top/clickable */
  }

  .nav {
    display: flex;
    gap: 24px;
    position: absolute;
    left: 50%;
    transform: translateX(-50%);
  }

  .nav-item {
    color: #4b5563;
    text-decoration: none;
    font-weight: 500;
    padding: 8px 0;
    border-bottom: 2px solid transparent;
    transition: all 0.2s;
  }

  .nav-item:hover {
    color: #111827;
  }

  .nav-item.active {
    color: #2563eb;
    border-bottom-color: #2563eb;
  }
</style>
