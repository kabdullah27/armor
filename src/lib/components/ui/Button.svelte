<script lang="ts">
  export let variant: "primary" | "secondary" | "danger" | "ghost" = "primary";
  export let disabled: boolean = false;
  export let type: "button" | "submit" = "button";
  export let fullWidth: boolean = false;
  export let size: "xs" | "sm" | "md" | "lg" = "sm";
</script>

<button
  {type}
  class="uk-btn uk-btn-{size} {variant === 'primary' ? 'uk-btn-primary' : variant === 'danger' ? 'uk-btn-destructive' : variant === 'ghost' ? 'uk-btn-ghost' : 'uk-btn-default'} {fullWidth ? 'w-full' : ''}"
  {disabled}
  on:click
>
  <slot />
</button>


