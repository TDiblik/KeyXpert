// references: https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
export function vk_to_string(vk: number): String {
  // 0-9 and A-Z range
  if (vk >= 0x30 && vk <= 0x5a) {
    return String.fromCharCode(vk);
  }

  // F1 - F24 range
  if (vk >= 0x70 && vk <= 0x87) {
    return `F${vk - 0x6f}`;
  }

  console.log(vk);

  // TODO: WIP, finish whenever I feel like copy pasting from docs...
  //
  // I didn't find better way than writing it out by hand...
  // Also, only some keys are supported, I left out the ones which are undefined or mouse related
  // prettier-ignore
  switch (vk) {
    case 0x08: return "BACK";
    case 0x09: return "TAB";
    case 0x0C: return "CLEAR";
    case 0x10: return "SHIFT";
    case 0x11: return "CONTROL";
    case 0x12: return "ALT";
    case 0x13: return "PAUSE";
    case 0x14: return "CAPS";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
    case 0x10: return "SHIFT";
  }

  return "pain";
}
