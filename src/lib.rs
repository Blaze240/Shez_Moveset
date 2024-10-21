#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]
use std::collections::HashMap;

use the_csk_collection_api::CStrCSK;

pub static mut MARKED_COLORS: [bool; 256] = [false; 256];

pub fn check_deps() -> bool {
    let mut passed = true;

    for dep in [
        "rom:/skyline/plugins/libthe_csk_collection.nro",
        "rom:/skyline/plugins/libarcropolis.nro",
        "rom:/skyline/plugins/libnro_hook.nro",
        "rom:/skyline/plugins/libsmashline_plugin.nro",
    ] {
        if !std::path::Path::new(dep).is_file() {
            println!("{} not found! This installation is incomplete. Please download all dependencies listed in the README file.", dep);
            passed = false;
        }
    }

    passed
}

extern "C" fn mods_mounted(_ev: arcropolis_api::Event) {
    // EVERYTHING HERE IS UNIQUE THAT'S MEANT TO BE MODIFIED FOR YOUR SPECIFIC PLUGIN.

    const FIGHTER_NAME: &str = "roy";
    const MARKER_FILE: &str = "shez.marker";
    let mut lowest_color: i32 = -1;
    let mut marked_slots: Vec<i32> = vec![];
    for x in 0..256 {
        if let Ok(_) = std::fs::read(format!(
            "mods:/fighter/{}/model/body/c{:02}/{}",
            FIGHTER_NAME, x, MARKER_FILE
        )) {
            unsafe {
                marked_slots.push(x as _);
                MARKED_COLORS[x as usize] = true;
                if lowest_color == -1 {
                    lowest_color = x as _;
                }
            }
        }
    }

    if lowest_color == -1 {
        // if no marker exist, leave
        return;
    }

    let color_num = {
        unsafe {
            let mut index = lowest_color;
            while index < 256 && MARKED_COLORS[index as usize] {
                index += 1;
            }
            index - lowest_color
        }
    };

    the_csk_collection_api::add_chara_db_entry_info(
        the_csk_collection_api::CharacterDatabaseEntry {
            ui_chara_id: 0x0da2301d49,                  // Hash40 of ui_chara_mario64
            clone_from_ui_chara_id: Some(0x0c0284ebc0), // Hash40 of ui_chara_mario
            name_id: the_csk_collection_api::StringType::Overwrite(CStrCSK::new("shez")),
            color_num: the_csk_collection_api::UnsignedByteType::Overwrite(8),
            extra_index_maps: the_csk_collection_api::UnsignedByteMap::Overwrite(HashMap::from([
                (
                    0x02dc2ad1a4, /* Hash40 of color_start_index */
                    the_csk_collection_api::UnsignedByteType::Overwrite(8),
                ),
            ])),
            extra_hash_maps: the_csk_collection_api::Hash40Map::Overwrite(HashMap::from([(
                0x160ab9eb98, /* Hash40 of original_ui_chara_hash */
                the_csk_collection_api::Hash40Type::Overwrite(0x0c0284ebc0), /* Hash40 of ui_chara_mario */
            )])),
            ..Default::default()
        },
    );
}

mod slipatch;
mod victory;

#[skyline::main(name = "shez_moveset")]
pub fn main() {
    if !check_deps() {
        return; // don't do anything else since they don't have all dependencies
    }

    unsafe {
        extern "C" {
            fn arcrop_register_event_callback(
                ty: arcropolis_api::Event,
                callback: arcropolis_api::EventCallbackFn,
            );
        }
        arcrop_register_event_callback(arcropolis_api::Event::ModFilesystemMounted, mods_mounted);
    }
    victory::install();
    slipatch::install();
}
