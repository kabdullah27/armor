<script lang="ts">
  export let variant: "primary" | "secondary" | "danger" = "primary";
  export let disabled: boolean = false;
  export let type: "button" | "submit" = "button";
  export let fullWidth: boolean = false;
</script>

<button
  {type}
  class="btn {variant}"
  class:full-width={fullWidth}
  {disabled}
  on:click
>
  <slot />
</button>

<style>
  .btn {
    padding: 10px 24px;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    border: none;
    transition: all 0.2s;
    font-family: inherit;
    white-space: nowrap;
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .btn:hover {
    transform: translateY(-1px);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  }

  .btn:active {
    transform: translateY(0);
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
    transform: none;
    box-shadow: none;
  }

  .btn.primary {
    background: #000;
    color: white;
  }

  .btn.primary:hover:not(:disabled) {
    background: #1f2937;
  }

  .btn.secondary {
    background: #fff;
    color: #000;
    border: 1px solid #e5e7eb;
  }

  .btn.secondary:hover:not(:disabled) {
    background: #f9fafb;
    border-color: #d1d5db;
  }

  .btn.danger {
    background: #fff;
    color: #dc2626;
    border: 1px solid #dc2626;
  }

  .btn.danger:hover:not(:disabled) {
    background: #fef2f2;
    border-color: #b91c1c;
    color: #b91c1c;
  }

  .full-width {
    width: 100%;
  }
</style>
