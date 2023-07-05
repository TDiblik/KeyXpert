import {writable} from "svelte/store";

export type AdvancedSettingsProps = null | object;

export const advanced_settings_props = writable<AdvancedSettingsProps | null>(null);
