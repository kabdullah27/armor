<script lang="ts">
  import { createEventDispatcher } from "svelte";

  export let show = false;
  export let title = "";
  export let maxWidth = "800px";

  const dispatch = createEventDispatcher();

  function close() {
    show = false;
    dispatch("close");
  }

  function handleOverlayClick(e: MouseEvent) {
    if (e.target === e.currentTarget) {
      close();
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter" || e.key === " ") {
      handleOverlayClick(e as unknown as MouseEvent);
    }
  }
</script>

{#if show}
  <div
    class="uk-modal uk-open uk-display-block"
    style="display: flex; align-items: center; justify-content: center; background: rgba(0,0,0,0.5);"
    role="button"
    tabindex="0"
    on:click={handleOverlayClick}
    on:keydown={handleKeydown}
  >
    <!-- svelte-ignore a11y-click-events-have-key-events, a11y-no-static-element-interactions -->
    <div 
      class="uk-modal-dialog uk-modal-body bg-white shadow-xl"
      style="position: relative; max-width: {maxWidth}; width: 100%; border-radius: 12px; max-height: 90vh; overflow-y: auto; padding: 30px;"
    >
      {#if title}
        <div class="uk-modal-header">
          <h2 id="modal-title" class="uk-modal-title">{title}</h2>
        </div>
      {/if}
      <button
        type="button"
        on:click={close}
        aria-label="Close"
        style="position: absolute; top: 14px; right: 14px; width: 32px; height: 32px; border: 0; border-radius: 8px; background: none; color: #6b7280; font-size: 22px; line-height: 1; cursor: pointer;"
      >×</button>

      <div class="uk-modal-body">
        <slot />
        {#if $$slots.footer}
          <div class="uk-modal-footer flex justify-end gap-2">
            <slot name="footer" />
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}
