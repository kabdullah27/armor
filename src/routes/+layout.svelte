<script lang="ts">
  import "../app.css";
  import { settings } from "$lib/stores/settings";
  import Sidebar from "$lib/components/layout/Sidebar.svelte";
  import Topbar from "$lib/components/layout/Topbar.svelte";
</script>

<div class="app-shell" class:has-sidebar={$settings.navbarPosition === "left"}>
  {#if $settings.navbarPosition === "left"}
    <Sidebar customColor={$settings.navbarColor} />
  {:else}
    <Topbar customColor={$settings.navbarColor} />
  {/if}

  <main class="main-content">
    <slot />
  </main>
</div>

<style>
  .app-shell {
    min-height: 100vh;
    background: #f9fafb;
  }

  /* When sidebar is present, add left padding */
  .app-shell.has-sidebar {
    padding-left: 240px;
  }

  .main-content {
    max-width: 1200px;
    margin: 0 auto;
    padding: 24px;
  }
</style>
