use std::fs::File;
use std::io::Write;
use std::path::Path;

use winapi::{
    ctypes::c_int,
    um::winuser::{
        VK_APPS, VK_ATTN, VK_BROWSER_BACK, VK_BROWSER_FAVORITES, VK_BROWSER_FORWARD,
        VK_BROWSER_HOME, VK_BROWSER_REFRESH, VK_BROWSER_SEARCH, VK_BROWSER_STOP, VK_CRSEL,
        VK_EREOF, VK_EXSEL, VK_ICO_00, VK_ICO_CLEAR, VK_ICO_HELP, VK_LAUNCH_APP1, VK_LAUNCH_APP2,
        VK_LAUNCH_MAIL, VK_LAUNCH_MEDIA_SELECT, VK_LCONTROL, VK_LMENU, VK_LSHIFT, VK_LWIN,
        VK_MEDIA_NEXT_TRACK, VK_MEDIA_PLAY_PAUSE, VK_MEDIA_PREV_TRACK, VK_MEDIA_STOP, VK_NONAME,
        VK_NUMLOCK, VK_OEM_1, VK_OEM_102, VK_OEM_2, VK_OEM_3, VK_OEM_4, VK_OEM_5, VK_OEM_6,
        VK_OEM_7, VK_OEM_8, VK_OEM_AX, VK_OEM_CLEAR, VK_OEM_COMMA, VK_OEM_FJ_JISHO, VK_OEM_FJ_LOYA,
        VK_OEM_FJ_MASSHOU, VK_OEM_FJ_ROYA, VK_OEM_FJ_TOUROKU, VK_OEM_MINUS, VK_OEM_PERIOD,
        VK_OEM_PLUS, VK_PA1, VK_PACKET, VK_PLAY, VK_PROCESSKEY, VK_RCONTROL, VK_RMENU, VK_RSHIFT,
        VK_RWIN, VK_SCROLL, VK_SLEEP, VK_VOLUME_DOWN, VK_VOLUME_MUTE, VK_VOLUME_UP, VK_ZOOM,
    },
};

// https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
// Sys keys as specified above. More info inside main.rs
const SYS_KEYS: [c_int; 63] = [
    VK_LSHIFT,
    VK_RSHIFT,
    VK_LCONTROL,
    VK_RCONTROL,
    VK_LMENU,
    VK_RMENU,
    VK_LWIN,
    VK_RWIN,
    VK_APPS,
    VK_SLEEP,
    VK_NUMLOCK,
    VK_SCROLL,
    VK_OEM_FJ_JISHO,
    VK_OEM_FJ_MASSHOU,
    VK_OEM_FJ_TOUROKU,
    VK_OEM_FJ_LOYA,
    VK_OEM_FJ_ROYA,
    VK_BROWSER_BACK,
    VK_BROWSER_FORWARD,
    VK_BROWSER_REFRESH,
    VK_BROWSER_STOP,
    VK_BROWSER_SEARCH,
    VK_BROWSER_FAVORITES,
    VK_BROWSER_HOME,
    VK_VOLUME_MUTE,
    VK_VOLUME_DOWN,
    VK_VOLUME_UP,
    VK_MEDIA_NEXT_TRACK,
    VK_MEDIA_PREV_TRACK,
    VK_MEDIA_STOP,
    VK_MEDIA_PLAY_PAUSE,
    VK_LAUNCH_MAIL,
    VK_LAUNCH_MEDIA_SELECT,
    VK_LAUNCH_APP1,
    VK_LAUNCH_APP2,
    VK_OEM_1,
    VK_OEM_PLUS,
    VK_OEM_COMMA,
    VK_OEM_MINUS,
    VK_OEM_PERIOD,
    VK_OEM_2,
    VK_OEM_3,
    VK_OEM_4,
    VK_OEM_5,
    VK_OEM_6,
    VK_OEM_7,
    VK_OEM_8,
    VK_OEM_AX,
    VK_OEM_102,
    VK_ICO_HELP,
    VK_ICO_00,
    VK_PROCESSKEY,
    VK_ICO_CLEAR,
    VK_PACKET,
    VK_ATTN,
    VK_CRSEL,
    VK_EXSEL,
    VK_EREOF,
    VK_PLAY,
    VK_ZOOM,
    VK_NONAME,
    VK_PA1,
    VK_OEM_CLEAR,
];

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("GENERATED_SYS_KEYS.rs");

    // Generate SYS_KEYS_TABLE
    let mut sys_keys_table: [bool; 256] = [false; 256];
    for &syskey in SYS_KEYS.iter() {
        sys_keys_table[syskey as usize] = true;
    }

    // Generate SYS_KEYS_TABLE code
    let mut code = "pub static SYS_KEYS_TABLE: [bool; 256] = [\n".to_owned();
    code.push_str(&sys_keys_table.map(|s| format!("{},\n", s)).concat());
    code.push_str("];\n");
    code.push_str(
        &SYS_KEYS
            .map(|s| {
                format!(
                    "
                    #[test]
                    fn is_{}_sys_key() {{
                        unsafe {{
                            assert!(crate::utils::is_sys_key({}));
                        }}
                    }}",
                    s, s
                )
            })
            .concat(),
    );

    // Write the generated code to a file
    let mut generated_sys_keys_file = File::create(dest_path).unwrap();
    generated_sys_keys_file.write_all(code.as_bytes()).unwrap();

    // only build the resource for release builds
    // as calling rc.exe might be slow
    if std::env::var("PROFILE").unwrap() == "release"
        && std::env::var("SHOULD_ATTACH_MANIFEST").unwrap_or("".to_owned()) == "true"
    {
        let mut res = winres::WindowsResource::new();
        res.set_manifest(
            r#"
        <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
        <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
            <security>
                <requestedPrivileges>
                    <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
                </requestedPrivileges>
            </security>
        </trustInfo>
        </assembly>
        "#,
        );
        if let Err(error) = res.compile() {
            eprint!("{}", error);
            std::process::exit(1);
        }
    }
}
