# KeyXpert

## Keyboard remapper for Windows and Linux.

![KeyXpert icon](./ui/src-tauri/icons/logo.png)

## How to install?

1. Download the latest installation file from [Releases tab](https://github.com/TDiblik/KeyXpert/releases).
2. Follow the installer.
   - If, for whatever reason, the installer does not start with admin privelages automatically, please, run it as Administrator.
   - This program **WILL trigger your anti-virus (sooner or later)**, make sure to turn it off and create a rule to ignore `C:\Program Files\KeyXpert` (more info in notes section, btw don't be scared, check the source code yourself to see that the program is not a malware, or alternativelly, compile it yourself from scratch, if you don't trust my bundles)
3. If you already have a version of KeyXpert installed, you can use the "Check for updates" button to automatically check for updates and install the latest version.

## How to use?

TODO

## Development

### Mapper service

Run `cargo r` from the `mapper_service` directory root.

### UI

Run `npm run tauri dev` from the `ui` directory root (make sure to run `npm i` beforehand).

### Publishing

TODO

### TODOs

- Add Linux support
- Create nice looking README so the project looks more legit
- Write tests that make sense.
- Write platform specific tests.

## Limitations / Notes

1. This program **WILL trigger your anti-virus (sooner or later)** (which makes sence, it literally uses similar techniques as keylogging). I could go and sign the bundle, and then verify it, and spend a lot of time persuading Windows that this is not in fast malware, etc, etc, but that would be a lot of work (I'll probably do it if it gets popular). For now, please turn off your anti-virus while installing and create a rule to ignore `C:\Program Files\KeyXpert`.
2. If you have `RECURSIVE_REMAPPING` enabled (not recommended, but somebody may like it), remapping from Normal keys (A, B, C, etc... ) onto
   System keys (ALT, CTRL, etc) is stable 1 level deep (this will not change, unless some good soul implements it and sends PR :D),
   meaning that if you remap `A` to `CTRL`, **it will work**, but once you proceed to remap `B` to `A`, your `CTRL` will
   get spammed with `KEYDOWN` commands instead of `SYS_KEYDOWN`. This can (and probably will) lead to undefined behavior. I don't plan on
   fixing this, because the number of people who will use `RECURSIVE_REMAPING`, then proceed to rebind Normal key onto System key and then
   rebing another normal key onto the previous normal key is (imo) an asymptote :D (1/x, lame joke, sorry, meaning less than 1 person, nearly 0 people, will need this functionality) and time to implement it is just not worth it.
3. Does not work in lock screen.
4. Unable to remap system shortcuts, however able to add system shortcut functinality to different shortcuts.
5. When remapping windows key, I am technically unable to capture the windows key before windows does, so remapping windows key UI kinda sucks, but works.
