<script lang="ts">
  export let variant: "primary" | "secondary" | "danger" | "ghost" = "primary";
  export let disabled: boolean = false;
  export let type: "button" | "submit" = "button";
  export let fullWidth: boolean = false;
</script>

<button
  {type}
  class="uk-button {variant === 'primary' ? 'uk-button-primary' : variant === 'danger' ? 'uk-button-danger' : variant === 'ghost' ? 'uk-button-ghost' : 'uk-button-default'} {fullWidth ? 'w-full' : ''}"
  {disabled}
  on:click
>
  <slot />
</button>

