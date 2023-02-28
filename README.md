TODO:

- Read settings from json
- Remap shortcuts
- Add settings UI
  - base UI
    - ability to remap keys
    - ability to remap shortcuts
    - Ability to select whether or not to allow reccursive remapping (for all keys)
    - After remapping, restart mapper_service windows service
  - UI installs latest mapper_service from github
  - UI registers mapper_service as a windows service that starts after startup
  - Select keyboard layout or ability to install new keyboard layout

# Limitations

1. If you have `RECURSIVE_REMAPPING` enabled (not recommended, but somebody may like it), remapping from Normal keys (A, B, C, etc... ) onto
   System keys (ALT, CTRL, etc) is stable 1 level deep (this will not change, unless some good soul implements it and sends PR :D),
   meaning that if you remap `A` to `CTRL`, **it will work**, but once you proceed to remap `B` to `A`, your `CTRL` will
   get spammed with `KEYDOWN` commands instead of `SYS_KEYDOWN`. This can (and probably will) lead to undefined behavior. I don't plan on
   fixing this, because the number of people who will use `RECURSIVE_REMAPING`, then proceed to rebind Normal key onto System key and then
   rebing another normal key onto the previous normal key is (imo) an asymptote :D (1/x, lame joke, sorry, meaning less than 1 person, nearly 0 people, will need this functionality) and time to implement it is just not worth it.
