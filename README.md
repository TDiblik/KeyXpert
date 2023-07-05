## <p style="text-align: center;">DOGFOODING atm to find all bugs</p>

<br/>
<br/>

TODO:

- Add Linux support
- Create some kind of logo
- Create nice looking README so the project looks more legit
- Create website from which users download the software (non-programmers don't use github)
- Write tests that make sense.
- Write platform specific tests.

# Dev

## Mapper service

- TODO

## UI

- `npm run tauri dev`

# Limitations / Notes

1. This program **WILL trigger your anti-virus** (which makes sence, it literally uses similar techniques as keylogging). I could go and sign the bundle, and then verify it, and spend a lot of time persuading Windows that this is not in fast malware, etc, etc, but that would be a lot of work (I'll probably do it if it gets popular). For now, please turn off your anti-virus while installing and create a rule to ignore `C:\Program Files\KeyXpert`.
2. If you have `RECURSIVE_REMAPPING` enabled (not recommended, but somebody may like it), remapping from Normal keys (A, B, C, etc... ) onto
   System keys (ALT, CTRL, etc) is stable 1 level deep (this will not change, unless some good soul implements it and sends PR :D),
   meaning that if you remap `A` to `CTRL`, **it will work**, but once you proceed to remap `B` to `A`, your `CTRL` will
   get spammed with `KEYDOWN` commands instead of `SYS_KEYDOWN`. This can (and probably will) lead to undefined behavior. I don't plan on
   fixing this, because the number of people who will use `RECURSIVE_REMAPING`, then proceed to rebind Normal key onto System key and then
   rebing another normal key onto the previous normal key is (imo) an asymptote :D (1/x, lame joke, sorry, meaning less than 1 person, nearly 0 people, will need this functionality) and time to implement it is just not worth it.
3. Does not work in lock screen.
4. Unable to remap system shortcuts, however able to add system shortcut functinality to different shortcuts.
5. When remapping windows key, I am technically unable to capture the windows key before windows does, so remapping windows key UI kinda sucks, but works.
