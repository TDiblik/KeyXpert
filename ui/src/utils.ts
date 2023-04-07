import type { CommandResult } from "./models";
import { modal_info, type ModalProps } from "./components/Modal/ModalStore";

// References for hard coded values:
// https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
// https://www.freecodecamp.org/news/javascript-keycode-list-keypress-event-key-codes/

// prettier-ignore
export function vk_to_string(vk: number): String {
  // 0-9 and A-Z range
  if (vk >= 0x30 && vk <= 0x5a) {
    return String.fromCharCode(vk);
  }

  // F1 - F24 range
  if (vk >= 0x70 && vk <= 0x87) {
    return `F${vk - 0x6f}`;
  }

  // TODO: Check against https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes 
  // ChatGPT wrote this, because I cannot be asked copy-pasting it from microsoft docs.
  // I've gone through some cases I caught during development (unsuprisingly a lot), but
  // it may contain many more bugs since it's AI generated, but it's OK for now...
  //
  // TODO2: It would be cool to add images instead of text for some keys (minus, percentage, *, /, enter, etc...)
  switch (vk) {
    case 0x08: return "Backspace";
    case 0x09: return "Tab";
    case 0x0C: return "Clear";
    case 0x0D: return "Enter"; // VK_RETURN
    case 0x10: return "Shift";
    case 0x11: return "Ctrl";
    case 0x12: return "Alt";
    case 0x13: return "Pause";
    case 0x14: return "Caps";
    case 0x14: return "Caps";
      case 0x15: return "IME Kana";
      case 0x16: return "IME On";
      case 0x17: return "IME Junja";
      case 0x18: return "IME Final";
      case 0x19: return "IME Hanja";
      case 0x1A: return "IME Off";
    case 0x1B: return "Esc";
      case 0x1C: return "IME Convert";
      case 0x1D: return "IME Nonconvert";
      case 0x1E: return "IME Accept";
      case 0x1F: return "IME mode change request";
    case 0x20: return "Space";
    case 0x21: return "PgUp";
    case 0x22: return "PgDn";
    case 0x23: return "End";
    case 0x24: return "Home";
    case 0x25: return "Left Arrow";
    case 0x26: return "Up Arrow";
    case 0x27: return "Right Arrow";
    case 0x28: return "Down Arrow";
    case 0x2D: return "Insert";
    case 0x2E: return "Del";
      case 0x2F: return "Help";
    // Exclude range 0x30 to 0x5A
    case 0x5B: return "Left Win"; //LEFT_SUPER
    case 0x5C: return "Right Win"; //RIGHT_SUPER
     case 0x5D: return "Apps";
    case 0x5D: return "Sleep";
    case 0x60: return "Numpad 0";
    case 0x61: return "Numpad 1";
    case 0x62: return "Numpad 2";
    case 0x63: return "Numpad 3";
    case 0x64: return "Numpad 4";
    case 0x65: return "Numpad 5";
    case 0x66: return "Numpad 6";
    case 0x67: return "Numpad 7";
    case 0x68: return "Numpad 8";
    case 0x69: return "Numpad 9";
    case 0x6A: return "Numpad *";
    case 0x6B: return "Numpad +";
    case 0x6C: return "Numpad Separator";
    case 0x6D: return "Numpad -";
    case 0x6E: return "Numpad Decimal";
    case 0x6F: return "Numpad /";
    // Exclude range 0x70 to 0x87
    case 0x90: return "Num Lock";
    case 0x91: return "Scroll Lock";
    case 0xA0: return "Left Shift";
    case 0xA1: return "Right Shift";
    case 0xA2: return "Left Ctrl";
    case 0xA3: return "Right Ctrl";
    case 0xA4: return "Left Alt";
    case 0xA5: return "Right Alt";
    case 0xA6: return "Browser Back";
    case 0xA7: return "Browser Forward";
    case 0xA8: return "Browser Refresh";
    case 0xA9: return "Browser Stop";
    case 0xAA: return "Browser Search";
    case 0xAB: return "Browser Favourites";
    case 0xAC: return "Browser Home";
    case 0xAD: return "Volume Mute";
    case 0xAE: return "Volume Down";
    case 0xAF: return "Volume Up";
    case 0xB0: return "Next Track";
    case 0xB1: return "Prev Track";
    case 0xB2: return "Stop Media";
    case 0xB3: return "Pause/Play Media";
    case 0xB4: return "Start Mail";
    case 0xB5: return "Select Media";
    case 0xB6: return "Start App 1";
    case 0xB7: return "Start App 2";
    case 0xBA: return "Semicolon";
    case 0xBB: return "Plus";
    case 0xBC: return "Comma";
    case 0xBD: return "Minus";
    case 0xBE: return "Period";
    case 0xBF: return "Slash/?";
    case 0xC0: return "~";
    case 0xDB: return "Left Bracket";
    case 0xDC: return "Backslash";
    case 0xDD: return "Right Bracket";
    case 0xDE: return "Quote";
      case 0xDF: return "VK_OEM_8";
      case 0xE2: return "VK_OEM_102";
    case 0xE5: return "IME Process";
      case 0xE7: return "VK_PACKET";
    case 0xF6: return "Attn";
    case 0xF7: return "CrSel";
    case 0xF8: return "ExSel";
    case 0xF9: return "Erase EOF";
    case 0xFA: return "Play";
    case 0xFB: return "Zoom";
    case 0xFE: return "PA1";
    case 0xFF: return "Clear";
  }

  return "UNKNOWN";
}

// prettier-ignore
export function cover_special_vk_cases(original_vk: number, code: string): number {
  switch (code) {
    case "MetaLeft": return 0x5B;
    case "MetaRight": return 0x5C;
    
    case "ControlLeft": return 0xA2;
    case "ControlRight": return 0xA3;

    case "AltLeft": return 0xA4;
    case "AltRight": return 0xA5;

    case "ShiftLeft": return 0xA0;
    case "ShiftRight": return 0xA1;
  }

  return original_vk;
}

// new_holding_keys is copied by reference,
// it would be "purer" to copy the array and return new one, but since this function will run a lot,
// I've decided to push into it for performance reasons.
export function if_keycode_pressed(
  all_pressed_keys: any[],
  keyname: string,
  left: number,
  right: number,
  new_holding_keys: number[]
) {
  if (all_pressed_keys[keyname + "Right"]) {
    new_holding_keys.push(right);
  } else if (all_pressed_keys[keyname + "Left"]) {
    new_holding_keys.push(left);
  }
}

export function prevent_event_bubbling(e: KeyboardEvent): boolean {
  e.cancelBubble = true;
  e.preventDefault();
  return false;
}

export function add_padding_to_keycode_array(new_holding_keys: number[]) {
  while (new_holding_keys.length < 4) {
    new_holding_keys.push(0);
  }
}

export function handle_tauri_result<T>(
  result: CommandResult<T>,
  success_callback: (result: T) => void = (_r: T) => {}
): boolean {
  if (!result.is_success) {
    modal_info.set({
      title: "Error occured :(",
      type: "error",
      description: result.message,
    } as ModalProps);
    return false;
  }

  success_callback(result.result);
  return true;
}
