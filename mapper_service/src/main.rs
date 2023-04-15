// Some unused assignments matter when comunicating with C and specially WinApi
// Rust correctlly shows warnings, however, in this context, those warnings are redundant and annoying.
#![allow(unused_assignments)]

mod app;
pub mod models;
pub mod utils;

use app::{App, AppCore};
use mapper_service::shared_constants::log_error;
use models::{LastSentRemapInfo, RemappedShortcut};

// https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
// Basically, we have at max 255 keys. What we do with REMAPPED_KEYS and SYS_KEYS_TABLE is
// fill the arrays with None/false and then add Some/true to places where keys are supposed to be
// this allowes us to find REMAPPED_KEYS, SYS_KEYS and REMAPPED_SHORTCUTS_CONTAIN_KEY in constant time, search on stack instead of heap and most likely use array inside CPU cache (this depends on CPU tho).
pub static mut REMAPPED_KEYS: [Option<u8>; 256] = [None; 256];

pub static mut REMAPPED_SHORTCUTS: Vec<RemappedShortcut> = vec![];
pub static mut REMAPPED_SHORTCUTS_CONTAIN_KEY: [bool; 256] = [false; 256];

include!(concat!(env!("OUT_DIR"), "/GENERATED_SYS_KEYS.rs"));

static mut ENABLE_RECURSIVE_REMAPPING: bool = false;
static mut ENABLE_RECURSIVE_SHORTCUTS: bool = false;

fn main() -> anyhow::Result<()> {
    if let Err(err) = App::setup() {
        log_error(&err);
        return Err(err);
    }

    if let Err(err) = unsafe { App::program_loop() } {
        log_error(&err);
        return Err(err);
    }

    Ok(())
}

// I could use kbd_struct.dwExtraInfo instead of static muts,
// however I currently like this approach more.
static mut LAST_SENT_REMAP_INFO: Option<LastSentRemapInfo> = None;
static mut STOP_RECURSIVE_SHORTCUT: bool = false;
