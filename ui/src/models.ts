export interface ServiceConfig {
  active_profile: null | string;
  profiles: null | Profile[];
  start_on_boot: boolean;
  enable_recursive_remapping: boolean;
  enable_recursive_shortcuts: boolean;
}

export interface Profile {
  id: string;
  name: string;
  key_remaps: KeyRemap[];
  shortcut_remaps: ShortcutRemap[];
}

export interface ProfileDetailsInfo extends Profile {
  use_this_profile: boolean;
}

export interface KeyRemap {
  from: number;
  to: number;
}

export interface ShortcutRemap {
  from_shortcut_holding_keys: number[];
  from_shortcut_execution_key: number;
  to_shortcut_holding_keys: number[];
  to_shortcut_execution_key: number;
}

export interface CommandResult<T> {
  is_success: boolean;
  message: string;
  result: null | T;
}

export interface UIConfig {
  window_height: number;
  window_width: number;
  window_position_x: number;
  window_position_y: number;
}
