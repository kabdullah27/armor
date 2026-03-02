<script lang="ts">
  import { createEventDispatcher } from "svelte";

  export let show = false;
  export let title = "";
  export let maxWidth = "800px";

  const dispatch = createEventDispatcher();

  function close() {
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
      class="uk-modal-dialog uk-modal-body"
      style="max-width: {maxWidth}; width: 100%; border-radius: 12px; max-height: 90vh; overflow-y: auto;"
    >
      {#if title}
        <div class="uk-modal-header">
          <h2 id="modal-title" class="uk-modal-title">{title}</h2>
        </div>
      {/if}
      <button
        class="uk-modal-close-default"
        type="button"
        uk-close
        on:click={close}
        aria-label="Close"
      ></button>

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
