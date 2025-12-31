<script lang="ts">
  export let id: string = "";
  export let label: string = "";
  export let value: string = "";
  export let type: "text" | "email" | "password" | "date" | "time" = "text";
  export let placeholder: string = "";
  export let required: boolean = false;
  export let disabled: boolean = false;
</script>

<div class="form-group">
  {#if label}
    <label for={id} class="form-label">
      {label}
      {#if required}<span class="required">*</span>{/if}
    </label>
  {/if}

  {#if type === "password"}
    <input
      {id}
      type="password"
      {placeholder}
      {disabled}
      bind:value
      class="form-input"
      class:disabled
    />
  {:else if type === "email"}
    <input
      {id}
      type="email"
      {placeholder}
      {disabled}
      bind:value
      class="form-input"
      class:disabled
    />
  {:else if type === "date"}
    <input
      {id}
      type="date"
      {placeholder}
      {disabled}
      bind:value
      class="form-input"
      class:disabled
    />
  {:else if type === "time"}
    <input
      {id}
      type="time"
      {placeholder}
      {disabled}
      bind:value
      class="form-input"
      class:disabled
    />
  {:else}
    <!-- Default to text -->
    <input
      {id}
      type="text"
      {placeholder}
      {disabled}
      bind:value
      class="form-input"
      class:disabled
    />
  {/if}
</div>

<style>
  .form-group {
    margin-bottom: 1.25rem;
  }

  .form-label {
    display: block;
    font-size: 0.875rem;
    font-weight: 500;
    color: #374151;
    margin-bottom: 0.5rem;
  }

  .required {
    color: #dc2626;
  }

  .form-input {
    width: 100%;
    padding: 0.625rem 0.875rem;
    border: 1px solid #d1d5db;
    border-radius: 0.5rem;
    font-size: 0.875rem;
    color: #111827;
    font-family: inherit;
    transition: border-color 0.15s;
    box-sizing: border-box; /* Ensure padding doesn't affect width */
  }

  .form-input:focus {
    outline: none;
    border-color: #000;
  }

  .form-input.disabled {
    opacity: 0.5;
    cursor: not-allowed;
    background-color: #f9fafb;
  }
</style>
