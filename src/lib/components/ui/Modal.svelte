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
  <div class="modal-overlay" on:click={handleOverlayClick}>
    <div
      class="modal"
      style="max-width: {maxWidth}"
      on:click={handleModalClick}
    >
      {#if title}
        <div class="modal-header">
          <h2>{title}</h2>
          <button class="close-btn" on:click={handleOverlayClick}>×</button>
        </div>
      {/if}

      <div class="modal-body">
        <slot />
      </div>

      <div class="modal-footer">
        <slot name="footer" />
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal {
    background: white;
    border-radius: 0.75rem;
    width: 90%;
    max-height: 90vh;
    overflow-y: auto;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  }

  .modal-header {
    padding: 1.5rem;
    border-bottom: 1px solid #e5e7eb;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .modal-header h2 {
    font-size: 1.25rem;
    font-weight: 600;
    color: #111827;
    margin: 0;
  }

  .close-btn {
    width: 2rem;
    height: 2rem;
    border-radius: 0.375rem;
    border: none;
    background: #f3f4f6;
    color: #6b7280;
    font-size: 1.5rem;
    line-height: 1;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .close-btn:hover {
    background: #e5e7eb;
  }

  .modal-body {
    padding: 1.5rem;
  }

  .modal-footer {
    padding: 1.25rem 1.5rem;
    border-top: 1px solid #e5e7eb;
    display: flex;
    gap: 0.75rem;
    justify-content: flex-end;
  }
</style>
