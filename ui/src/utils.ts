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
    case 0x08: return "BACK";
    case 0x09: return "TAB";
    case 0x0C: return "CLEAR";
    case 0x0D: return "ENTER"; // VK_RETURN
    case 0x10: return "SHIFT";
    case 0x11: return "CONTROL";
    case 0x12: return "ALT";
    case 0x13: return "PAUSE";
    case 0x14: return "CAPS";
    case 0x1B: return "ESCAPE";
    case 0x20: return "SPACE";
    case 0x21: return "PAGE_UP";
    case 0x22: return "PAGE_DOWN";
    case 0x23: return "END";
    case 0x24: return "HOME";
    case 0x25: return "LEFT_ARROW";
    case 0x26: return "UP_ARROW";
    case 0x27: return "RIGHT_ARROW";
    case 0x28: return "DOWN_ARROW";
    case 0x2D: return "INSERT";
    case 0x2E: return "DELETE";
    // Exclude range 0x30 to 0x5A
    case 0x5B: return "LEFT 🪟"; //LEFT_SUPER
    case 0x5C: return "RIGHT 🪟"; //RIGHT_SUPER
    case 0x60: return "NUM_0";
    case 0x61: return "NUM_1";
    case 0x62: return "NUM_2";
    case 0x63: return "NUM_3";
    case 0x64: return "NUM_4";
    case 0x65: return "NUM_5";
    case 0x66: return "NUM_6";
    case 0x67: return "NUM_7";
    case 0x68: return "NUM_8";
    case 0x69: return "NUM_9";
    case 0x6A: return "NUM_MULTIPLY";
    case 0x6B: return "NUM_ADD";
    case 0x6C: return "NUM_SEPARATOR";
    case 0x6D: return "NUM_SUBTRACT";
    case 0x6E: return "NUM_DECIMAL";
    case 0x6F: return "NUM_DIVIDE";
    // Exclude range 0x70 to 0x87
    case 0x90: return "NUM_LOCK";
    case 0x91: return "SCROLL_LOCK";
    case 0xA0: return "LEFT_SHIFT";
    case 0xA1: return "RIGHT_SHIFT";
    case 0xA2: return "LEFT_CONTROL";
    case 0xA3: return "RIGHT_CONTROL";
    case 0xA4: return "LEFT_ALT";
    case 0xA5: return "RIGHT_ALT";
    case 0xA6: return "BROWSER_BACK";
    case 0xA7: return "BROWSER_FORWARD";
    case 0xA8: return "BROWSER_REFRESH";
    case 0xA9: return "BROWSER_STOP";
    case 0xAA: return "BROWSER_SEARCH";
    case 0xAB: return "BROWSER_FAVORITES";
    case 0xAC: return "BROWSER_HOME";
    case 0xAD: return "VOLUME_MUTE";
    case 0xAE: return "VOLUME_DOWN";
    case 0xAF: return "VOLUME_UP";
    case 0xB0: return "MEDIA_NEXT_TRACK";
    case 0xB1: return "MEDIA_PREV_TRACK";
    case 0xB2: return "MEDIA_STOP";
    case 0xB3: return "MEDIA_PLAY_PAUSE";
    case 0xB4: return "LAUNCH_MAIL";
    case 0xB5: return "LAUNCH_MEDIA_SELECT";
    case 0xB6: return "LAUNCH_APP1";
    case 0xB7: return "LAUNCH_APP2";
    case 0xBA: return "SEMICOLON";
    case 0xBB: return "EQUALS";
    case 0xBC: return "COMMA";
    case 0xBD: return "MINUS";
    case 0xBE: return "PERIOD";
    case 0xBF: return "SLASH";
    case 0xC0: return "BACK_QUOTE";
    case 0xDB: return "LEFT_BRACKET";
    case 0xDC: return "BACKSLASH";
    case 0xDD: return "RIGHT_BRACKET";
    case 0xDE: return "QUOTE";
    case 0xE5: return "IME_PROCESS";
    case 0xF6: return "ATTN";
    case 0xF7: return "CRSEL";
    case 0xF8: return "EXSEL";
    case 0xF9: return "EREOF";
    case 0xFA: return "PLAY";
    case 0xFB: return "ZOOM";
    case 0xFE: return "PA1";
    case 0xFF: return "OEM_CLEAR";
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
