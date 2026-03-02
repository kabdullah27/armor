<script lang="ts">
  export let id: string = "";
  export let label: string = "";
  export let value: string = "";
  export let type: "text" | "email" | "password" | "date" | "time" = "text";
  export let placeholder: string = "";
  export let required: boolean = false;
  export let disabled: boolean = false;
</script>

<div class="uk-margin">
  {#if label}
    <label class="uk-form-label" for={id}>
      {label}
      {#if required}<span class="uk-text-danger">*</span>{/if}
    </label>
  {/if}

  <div class="uk-form-controls">
    {#if type === "password"}
      <input
        {id}
        type="password"
        {placeholder}
        {disabled}
        bind:value
        class="uk-input"
        class:uk-disabled={disabled}
      />
    {:else if type === "email"}
      <input
        {id}
        type="email"
        {placeholder}
        {disabled}
        bind:value
        class="uk-input"
        class:uk-disabled={disabled}
      />
    {:else if type === "date"}
      <input
        {id}
        type="date"
        {placeholder}
        {disabled}
        bind:value
        class="uk-input"
        class:uk-disabled={disabled}
      />
    {:else if type === "time"}
      <input
        {id}
        type="time"
        {placeholder}
        {disabled}
        bind:value
        class="uk-input"
        class:uk-disabled={disabled}
      />
    {:else}
      <input
        {id}
        type="text"
        {placeholder}
        {disabled}
        bind:value
        class="uk-input"
        class:uk-disabled={disabled}
      />
    {/if}
  </div>
</div>
