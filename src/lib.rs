#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]
#![allow(
    unused_macros,
    unused_imports,
    non_upper_case_globals,
    non_snake_case,
    clippy::borrow_interior_mutable_const
)]
use smash::hash40;
use std::collections::HashMap;
use the_csk_collection_api::*;

pub static mut MARKED_COLORS: [bool; 256] = [false; 256];
pub mod shez;
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
    the_csk_collection_api::add_chara_db_entry_info(CharacterDatabaseEntry {
        ui_chara_id: 0x0da2301d49,
        fighter_kind: the_csk_collection_api::Hash40Type::Overwrite(
            0x10941B9DD5, /* Hash40 of fighter_kind_roy */
        ),
        fighter_kind_corps: the_csk_collection_api::Hash40Type::Overwrite(
            0x10941B9DD5, /* Hash40 of fighter_kind_roy */
        ),
        ui_series_id: the_csk_collection_api::Hash40Type::Optional(Some(
            0x14B618467B, /* Hash40 of ui_series_fireemblem */
        )),
        fighter_type: the_csk_collection_api::Hash40Type::Optional(Some(
            0x1353795179, /* Hash40 of fighter_type_normal */
        )),
        alt_chara_id: the_csk_collection_api::Hash40Type::Optional(Some(0x0)),
        shop_item_tag: the_csk_collection_api::Hash40Type::Optional(Some(0x0)),
        name_id: the_csk_collection_api::StringType::Overwrite(
            the_csk_collection_api::CStrCSK::new("shez"),
        ),
        exhibit_year: the_csk_collection_api::ShortType::Optional(Some(2022)),
        exhibit_day_order: the_csk_collection_api::IntType::Optional(Some(32901)),
        extra_flags: the_csk_collection_api::IntType::Optional(Some(0)),
        ext_skill_page_num: the_csk_collection_api::SignedByteType::Optional(Some(0)),
        skill_list_order: the_csk_collection_api::SignedByteType::Optional(Some(88)),
        disp_order: the_csk_collection_api::SignedByteType::Optional(Some(88)),
        save_no: the_csk_collection_api::SignedByteType::Optional(Some(27)),
        chara_count: the_csk_collection_api::SignedByteType::Optional(Some(1)),
        is_img_ext_skill_page0: the_csk_collection_api::BoolType::Optional(Some(false)),
        is_img_ext_skill_page1: the_csk_collection_api::BoolType::Optional(Some(false)),
        is_img_ext_skill_page2: the_csk_collection_api::BoolType::Optional(Some(false)),
        can_select: the_csk_collection_api::BoolType::Optional(Some(true)),
        is_usable_soundtest: the_csk_collection_api::BoolType::Optional(Some(true)),
        is_called_pokemon: the_csk_collection_api::BoolType::Optional(Some(false)),
        is_mii: the_csk_collection_api::BoolType::Optional(Some(false)),
        is_boss: the_csk_collection_api::BoolType::Optional(Some(false)),
        is_hidden_boss: the_csk_collection_api::BoolType::Optional(Some(false)),
        is_dlc: the_csk_collection_api::BoolType::Optional(Some(false)),
        is_patch: the_csk_collection_api::BoolType::Optional(Some(false)),
        is_plural_message: the_csk_collection_api::BoolType::Optional(Some(false)),
        is_plural_narration: the_csk_collection_api::BoolType::Optional(Some(false)),
        is_article: the_csk_collection_api::BoolType::Optional(Some(false)),
        unk_0x112b7bb52a: the_csk_collection_api::BoolType::Optional(Some(false)),
        result_pf0: the_csk_collection_api::BoolType::Optional(Some(true)),
        result_pf1: the_csk_collection_api::BoolType::Optional(Some(true)),
        result_pf2: the_csk_collection_api::BoolType::Optional(Some(true)),
        color_num: the_csk_collection_api::UnsignedByteType::Optional(Some(8)),
        extra_hash_maps: the_csk_collection_api::Hash40Map::Overwrite(HashMap::from([
            (
                0x1337FC912E, /* Hash40 of characall_label_c00 */
                the_csk_collection_api::Hash40Type::Optional(Some(0x0)),
            ),
            (
                0x1340FBA1B8, /* Hash40 of characall_label_c01 */
                the_csk_collection_api::Hash40Type::Optional(Some(0x0)),
            ),
            (
                0x13D9F2F002, /* Hash40 of characall_label_c02 */
                the_csk_collection_api::Hash40Type::Optional(Some(0x0)),
            ),
            (
                0x13AEF5C094, /* Hash40 of characall_label_c03 */
                the_csk_collection_api::Hash40Type::Optional(Some(0x0)),
            ),
            (
                0x1330915537, /* Hash40 of characall_label_c04 */
                the_csk_collection_api::Hash40Type::Optional(Some(0x0)),
            ),
            (
                0x13479665A1, /* Hash40 of characall_label_c05 */
                the_csk_collection_api::Hash40Type::Optional(Some(0x0)),
            ),
            (
                0x13DE9F341B, /* Hash40 of characall_label_c06 */
                the_csk_collection_api::Hash40Type::Optional(Some(0x0)),
            ),
            (
                0x13A998048D, /* Hash40 of characall_label_c07 */
                the_csk_collection_api::Hash40Type::Optional(Some(0x0)),
            ),
            (
                0x1B8B13E500, /* Hash40 of characall_label_article_c00 */
                the_csk_collection_api::Hash40Type::Optional(Some(0x0)),
            ),
            (
                0x1BFC14D596, /* Hash40 of characall_label_article_c01 */
                the_csk_collection_api::Hash40Type::Optional(Some(0x0)),
            ),
            (
                0x1B651D842C, /* Hash40 of characall_label_article_c02 */
                the_csk_collection_api::Hash40Type::Optional(Some(0x0)),
            ),
            (
                0x1B121AB4BA, /* Hash40 of characall_label_article_c03 */
                the_csk_collection_api::Hash40Type::Optional(Some(0x0)),
            ),
            (
                0x1B8C7E2119, /* Hash40 of characall_label_article_c04 */
                the_csk_collection_api::Hash40Type::Optional(Some(0x0)),
            ),
            (
                0x1BFB79118F, /* Hash40 of characall_label_article_c05 */
                the_csk_collection_api::Hash40Type::Optional(Some(0x0)),
            ),
            (
                0x1B62704035, /* Hash40 of characall_label_article_c06 */
                the_csk_collection_api::Hash40Type::Optional(Some(0x0)),
            ),
            (
                0x1B157770A3, /* Hash40 of characall_label_article_c07 */
                the_csk_collection_api::Hash40Type::Optional(Some(0x0)),
            ),
            (
                0x160ab9eb98,
                the_csk_collection_api::Hash40Type::Optional(Some(
                    0xC0284EBC0, /* Hash40 of ui_chara_roy */
                )),
            ),
        ])),
        extra_index_maps: the_csk_collection_api::UnsignedByteMap::Overwrite(HashMap::from([
            (
                0x915C075DE, /* Hash40 of c00_index */
                the_csk_collection_api::UnsignedByteType::Optional(Some(0)),
            ),
            (
                0x9B3B77E6A, /* Hash40 of c01_index */
                the_csk_collection_api::UnsignedByteType::Optional(Some(0)),
            ),
            (
                0x9825F64F7, /* Hash40 of c02_index */
                the_csk_collection_api::UnsignedByteType::Optional(Some(0)),
            ),
            (
                0x924286F43, /* Hash40 of c03_index */
                the_csk_collection_api::UnsignedByteType::Optional(Some(0)),
            ),
            (
                0x9E18F51CD, /* Hash40 of c04_index */
                the_csk_collection_api::UnsignedByteType::Optional(Some(0)),
            ),
            (
                0x947F85A79, /* Hash40 of c05_index */
                the_csk_collection_api::UnsignedByteType::Optional(Some(0)),
            ),
            (
                0x9761040E4, /* Hash40 of c06_index */
                the_csk_collection_api::UnsignedByteType::Optional(Some(0)),
            ),
            (
                0x9D0674B50, /* Hash40 of c07_index */
                the_csk_collection_api::UnsignedByteType::Optional(Some(0)),
            ),
            (
                0x9E48F9289, /* Hash40 of n00_index */
                the_csk_collection_api::UnsignedByteType::Optional(Some(0)),
            ),
            (
                0x942F8993D, /* Hash40 of n01_index */
                the_csk_collection_api::UnsignedByteType::Optional(Some(0)),
            ),
            (
                0x9731083A0, /* Hash40 of n02_index */
                the_csk_collection_api::UnsignedByteType::Optional(Some(0)),
            ),
            (
                0x9D5678814, /* Hash40 of n03_index */
                the_csk_collection_api::UnsignedByteType::Optional(Some(0)),
            ),
            (
                0x910C0B69A, /* Hash40 of n04_index */
                the_csk_collection_api::UnsignedByteType::Optional(Some(0)),
            ),
            (
                0x9B6B7BD2E, /* Hash40 of n05_index */
                the_csk_collection_api::UnsignedByteType::Optional(Some(0)),
            ),
            (
                0x9875FA7B3, /* Hash40 of n06_index */
                the_csk_collection_api::UnsignedByteType::Optional(Some(0)),
            ),
            (
                0x92128AC07, /* Hash40 of n07_index */
                the_csk_collection_api::UnsignedByteType::Optional(Some(0)),
            ),
            (
                0x9F873561A, /* Hash40 of c00_group */
                the_csk_collection_api::UnsignedByteType::Optional(Some(0)),
            ),
            (
                0x95E045DAE, /* Hash40 of c01_group */
                the_csk_collection_api::UnsignedByteType::Optional(Some(0)),
            ),
            (
                0x96FEC4733, /* Hash40 of c02_group */
                the_csk_collection_api::UnsignedByteType::Optional(Some(0)),
            ),
            (
                0x9C99B4C87, /* Hash40 of c03_group */
                the_csk_collection_api::UnsignedByteType::Optional(Some(0)),
            ),
            (
                0x90C3C7209, /* Hash40 of c04_group */
                the_csk_collection_api::UnsignedByteType::Optional(Some(0)),
            ),
            (
                0x9AA4B79BD, /* Hash40 of c05_group */
                the_csk_collection_api::UnsignedByteType::Optional(Some(0)),
            ),
            (
                0x99BA36320, /* Hash40 of c06_group */
                the_csk_collection_api::UnsignedByteType::Optional(Some(0)),
            ),
            (
                0x93DD46894, /* Hash40 of c07_group */
                the_csk_collection_api::UnsignedByteType::Optional(Some(0)),
            ),
            (
                0x11895f00fc,
                the_csk_collection_api::UnsignedByteType::Optional(Some(40)),
            ),
        ])),
        ..Default::default()
    });
    the_csk_collection_api::add_chara_layout_db_entry_info(
        the_csk_collection_api::CharacterLayoutDatabaseEntry {
            ui_layout_id: 0x10c7c05c9d,
            ui_chara_id: the_csk_collection_api::Hash40Type::Overwrite(0x0da2301d49),
            chara_color: the_csk_collection_api::UnsignedByteType::Optional(Some(0)),
            eye_0_flash_count: the_csk_collection_api::UnsignedByteType::Optional(Some(2)),
            eye_1_flash_count: the_csk_collection_api::UnsignedByteType::Optional(Some(2)),
            eye_2_flash_count: the_csk_collection_api::UnsignedByteType::Optional(Some(2)),
            eye_0_flash0_pos_x: the_csk_collection_api::FloatType::Optional(Some(-120.0)),
            eye_0_flash0_pos_y: the_csk_collection_api::FloatType::Optional(Some(210.0)),
            eye_0_flash1_pos_x: the_csk_collection_api::FloatType::Optional(Some(-9.0)),
            eye_0_flash1_pos_y: the_csk_collection_api::FloatType::Optional(Some(227.0)),
            eye_0_flash2_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            eye_0_flash2_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            eye_0_flash3_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            eye_0_flash3_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            eye_0_flash4_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            eye_0_flash4_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            eye_1_flash0_pos_x: the_csk_collection_api::FloatType::Optional(Some(-92.0)),
            eye_1_flash0_pos_y: the_csk_collection_api::FloatType::Optional(Some(188.0)),
            eye_1_flash1_pos_x: the_csk_collection_api::FloatType::Optional(Some(16.0)),
            eye_1_flash1_pos_y: the_csk_collection_api::FloatType::Optional(Some(204.0)),
            eye_1_flash2_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            eye_1_flash2_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            eye_1_flash3_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            eye_1_flash3_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            eye_1_flash4_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            eye_1_flash4_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            eye_2_flash0_pos_x: the_csk_collection_api::FloatType::Optional(Some(-73.0)),
            eye_2_flash0_pos_y: the_csk_collection_api::FloatType::Optional(Some(83.0)),
            eye_2_flash1_pos_x: the_csk_collection_api::FloatType::Optional(Some(7.0)),
            eye_2_flash1_pos_y: the_csk_collection_api::FloatType::Optional(Some(95.0)),
            eye_2_flash2_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            eye_2_flash2_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            eye_2_flash3_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            eye_2_flash3_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            eye_2_flash4_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            eye_2_flash4_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            eye_flash_info_pos_x: the_csk_collection_api::FloatType::Optional(Some(2.0)),
            eye_flash_info_pos_y: the_csk_collection_api::FloatType::Optional(Some(-2.0)),
            chara_1_offset_x: the_csk_collection_api::FloatType::Optional(Some(2.0)),
            chara_1_offset_y: the_csk_collection_api::FloatType::Optional(Some(-69.0)),
            chara_1_scale: the_csk_collection_api::FloatType::Optional(Some(1.1)),
            chara_1_1_offset_x: the_csk_collection_api::FloatType::Optional(Some(3.0)),
            chara_1_1_offset_y: the_csk_collection_api::FloatType::Optional(Some(-64.0)),
            chara_1_1_scale: the_csk_collection_api::FloatType::Optional(Some(1.46)),
            chara_1_2_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            chara_1_2_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            chara_1_2_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)),
            chara_1_3_offset_x: the_csk_collection_api::FloatType::Optional(Some(3.0)),
            chara_1_3_offset_y: the_csk_collection_api::FloatType::Optional(Some(-44.0)),
            chara_1_3_scale: the_csk_collection_api::FloatType::Optional(Some(1.6)),
            chara_1_4_offset_x: the_csk_collection_api::FloatType::Optional(Some(3.0)),
            chara_1_4_offset_y: the_csk_collection_api::FloatType::Optional(Some(-44.0)),
            chara_1_4_scale: the_csk_collection_api::FloatType::Optional(Some(1.6)),
            chara_1_5_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            chara_1_5_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            chara_1_5_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)),
            chara_3_0_offset_x: the_csk_collection_api::FloatType::Optional(Some(397.0)),
            chara_3_0_offset_y: the_csk_collection_api::FloatType::Optional(Some(-80.0)),
            chara_3_0_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)),
            chara_3_1_offset_x: the_csk_collection_api::FloatType::Optional(Some(406.0)),
            chara_3_1_offset_y: the_csk_collection_api::FloatType::Optional(Some(-90.0)),
            chara_3_1_scale: the_csk_collection_api::FloatType::Optional(Some(0.96)),
            chara_3_2_offset_x: the_csk_collection_api::FloatType::Optional(Some(100.0)),
            chara_3_2_offset_y: the_csk_collection_api::FloatType::Optional(Some(-100.0)),
            chara_3_2_scale: the_csk_collection_api::FloatType::Optional(Some(0.8)),
            chara_3_3_offset_x: the_csk_collection_api::FloatType::Optional(Some(397.0)),
            chara_3_3_offset_y: the_csk_collection_api::FloatType::Optional(Some(-120.0)),
            chara_3_3_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)),
            chara_3_4_offset_x: the_csk_collection_api::FloatType::Optional(Some(329.0)),
            chara_3_4_offset_y: the_csk_collection_api::FloatType::Optional(Some(-80.0)),
            chara_3_4_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)),
            chara_3_5_offset_x: the_csk_collection_api::FloatType::Optional(Some(380.0)),
            chara_3_5_offset_y: the_csk_collection_api::FloatType::Optional(Some(-128.0)),
            chara_3_5_scale: the_csk_collection_api::FloatType::Optional(Some(1.02)),
            chara_3_6_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            chara_3_6_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            chara_3_6_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)),
            chara_3_7_offset_x: the_csk_collection_api::FloatType::Optional(Some(397.0)),
            chara_3_7_offset_y: the_csk_collection_api::FloatType::Optional(Some(-80.0)),
            chara_3_7_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)),
            chara_5_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            chara_5_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            chara_5_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)),
            chara_select_icon_list_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            chara_select_icon_list_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            chara_select_icon_list_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)),
            chara_7_0_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            chara_7_0_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            chara_7_0_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)),
            chara_7_1_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            chara_7_1_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            chara_7_1_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)),
            chara_0_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            chara_0_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)),
            chara_0_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)),
            spirits_eye_visible: the_csk_collection_api::BoolType::Optional(Some(true)),
            ..Default::default()
        },
    );
}

mod slipatch;

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
    shez::install();
    slipatch::install();
}
