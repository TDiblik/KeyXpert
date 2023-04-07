TODO:

- Add settings UI
  - base UI
    - After remapping, restart mapper_service windows service
  - UI installs latest mapper_service from github
  - UI registers mapper_service as a windows service that starts after startup
  - Select keyboard layout, install new keyboard layout and delete existing keyboard layouts
  - Ability to remove sticky keys option from poping-up every time I press shift more than once in a span of 1 minute :/
  - Remove ability to Maximize window
- Finish base project windows-only
- Add Linux support
- Add MacOs support (I have very limited access to one mac which I could use for testing from time to time, but PR from Mac users will be more than welcome...)
- Write tests that make sense.
- Write platform specific tests.

# Dev

## Mapper service

- TODO

## UI

- `npm run tauri dev`

# Limitations

1. If you have `RECURSIVE_REMAPPING` enabled (not recommended, but somebody may like it), remapping from Normal keys (A, B, C, etc... ) onto
   System keys (ALT, CTRL, etc) is stable 1 level deep (this will not change, unless some good soul implements it and sends PR :D),
   meaning that if you remap `A` to `CTRL`, **it will work**, but once you proceed to remap `B` to `A`, your `CTRL` will
   get spammed with `KEYDOWN` commands instead of `SYS_KEYDOWN`. This can (and probably will) lead to undefined behavior. I don't plan on
   fixing this, because the number of people who will use `RECURSIVE_REMAPING`, then proceed to rebind Normal key onto System key and then
   rebing another normal key onto the previous normal key is (imo) an asymptote :D (1/x, lame joke, sorry, meaning less than 1 person, nearly 0 people, will need this functionality) and time to implement it is just not worth it.
2. Does not work in lock screen.
3. Unable to remap system shortcuts, however able to add system shortcut functinality to different shortcuts.
