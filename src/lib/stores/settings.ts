import { writable } from "svelte/store";

export type NavbarPosition = "top" | "left";

export interface UISettings {
  navbarPosition: NavbarPosition;
  navbarColor: string;
}

// Default settings
const defaultSettings: UISettings = {
  navbarPosition: "left", // Default to left as requested
  navbarColor: "#ffffff",
};

function createSettingsStore() {
  const { subscribe, set, update } = writable<UISettings>(defaultSettings);

  return {
    subscribe,
    set,
    update,
    // Helper to toggle position
    togglePosition: () => update(s => ({ 
      ...s, 
      navbarPosition: s.navbarPosition === "left" ? "top" : "left" 
    })),
    // Helper to set color
    setColor: (color: string) => update(s => ({ ...s, navbarColor: color })),
  };
}

export const settings = createSettingsStore();
