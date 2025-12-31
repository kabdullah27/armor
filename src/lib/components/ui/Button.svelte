<script lang="ts">
  export let variant: "primary" | "secondary" | "danger" = "primary";
  export let disabled: boolean = false;
  export let type: "button" | "submit" = "button";
  export let fullWidth: boolean = false;

  const baseStyles =
    "px-6 py-2.5 rounded-lg font-medium text-sm transition-all cursor-pointer border-none";

  const variantStyles = {
    primary: "bg-black text-white hover:bg-gray-800",
    secondary: "bg-white text-black border border-gray-300 hover:bg-gray-50",
    danger: "bg-white text-red-600 border-2 border-red-600 hover:bg-red-50",
  };

  $: buttonClass = `${baseStyles} ${variantStyles[variant]} ${fullWidth ? "w-full" : ""} ${disabled ? "opacity-50 cursor-not-allowed" : ""}`;
</script>

<button {type} class={buttonClass} {disabled} on:click>
  <slot />
</button>

<style>
  button {
    font-family: inherit;
  }
</style>
