<script lang="ts">
  export let variant: "primary" | "secondary" | "danger" = "primary";
  export let disabled: boolean = false;
  export let type: "button" | "submit" = "button";
  export let fullWidth: boolean = false;

  $: btnClass = (() => {
    switch (variant) {
      case "primary":
        return "uk-button uk-button-primary";
      case "secondary":
        return "uk-button uk-button-default";
      case "danger":
        return "uk-button uk-button-danger";
      default:
        return "uk-button uk-button-default";
    }
  })();
</script>

<button
  {type}
  class="{btnClass} {fullWidth ? 'w-full' : ''}"
  {disabled}
  on:click
>
  <slot />
</button>
