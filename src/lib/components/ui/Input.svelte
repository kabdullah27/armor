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
    <input
      {id}
      {type}
      {placeholder}
      {disabled}
      bind:value
      class="uk-input"
      class:uk-disabled={disabled}
    />
  </div>
</div>
