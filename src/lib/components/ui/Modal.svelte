<script lang="ts">
  export let show: boolean = false;
  export let title: string = "";
  export let maxWidth: string = "600px";

  function handleOverlayClick() {
    show = false;
  }

  function handleModalClick(event: MouseEvent) {
    event.stopPropagation();
  }
</script>

{#if show}
  <div
    class="uk-modal uk-open"
    style="display: flex; background: rgba(0,0,0,0.5);"
    on:click={handleOverlayClick}
  >
    <div
      class="uk-modal-dialog uk-margin-auto-vertical"
      style="max-width: {maxWidth}; width: 100%;"
      on:click={handleModalClick}
    >
      <button
        class="uk-modal-close-default"
        type="button"
        on:click={handleOverlayClick}
        uk-close
      ></button>

      {#if title}
        <div class="uk-modal-header">
          <h2 class="uk-modal-title">{title}</h2>
        </div>
      {/if}

      <div class="uk-modal-body">
        <slot />
      </div>

      <div class="uk-modal-footer uk-text-right">
        <slot name="footer" />
      </div>
    </div>
  </div>
{/if}
