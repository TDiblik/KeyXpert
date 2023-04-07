export interface ServiceConfig {
  active_profile: null | string;
  profiles: null | Profile[];
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
