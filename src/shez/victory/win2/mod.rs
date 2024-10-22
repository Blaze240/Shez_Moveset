use {
    skyline_smash::app::utility::get_kind,
    smash::{
        app::{lua_bind::*, sv_animcmd::*, sv_battle_object::*, utility::*},
        hash40,
        lib::{lua_const::*, L2CAgent, L2CValue},
        lua2cpp::*,
        phx::*,
    },
    smash_script::*,
    smashline::{Priority::*, *},
};

unsafe extern "C" fn roy_sound_win2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 0.0);
    if macros::is_excute(agent) {
        if macros::is_excute(agent) {
            let entry_count =
                smash::app::lua_bind::FighterManager::entry_count(singletons::FighterManager());
            let mut master_exist = false;

            for i in 0..entry_count {
                let chara = the_csk_collection_api::get_ui_chara_from_entry_id(i as u32);

                if chara == hash40("ui_chara_master") {
                    master_exist = true;
                    break;
                }
            }
            if master_exist {
                macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_roy_win_master"));
                frame(agent.lua_state_agent, 54.0);
                if macros::is_excute(agent) {
                    macros::PLAY_SE(agent, Hash40::new("se_roy_appeal_l01_win02"));
                }
                frame(agent.lua_state_agent, 62.0);
                if macros::is_excute(agent) {
                    macros::PLAY_SE(agent, Hash40::new("se_roy_appeal_l02_win02"));
                }
                frame(agent.lua_state_agent, 71.0);
                if macros::is_excute(agent) {
                    macros::PLAY_SE(agent, Hash40::new("se_roy_appeal_l02_win02"));
                }
                frame(agent.lua_state_agent, 118.0);
                if macros::is_excute(agent) {
                    macros::PLAY_SE(agent, Hash40::new("se_roy_win2_win02"));
                }
            }
        } else {
            frame(agent.lua_state_agent, 48.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_roy_win03"));
            }
            frame(agent.lua_state_agent, 54.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("se_roy_appeal_l01_win02"));
            }
            frame(agent.lua_state_agent, 62.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("se_roy_appeal_l02_win02"));
            }
            frame(agent.lua_state_agent, 71.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("se_roy_appeal_l02_win02"));
            }
            frame(agent.lua_state_agent, 118.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("se_roy_win2_win02"));
            }
        }
    }
}

pub fn install() {
    Agent::new("roy")
        .sound_acmd("sound_win2_shez", roy_sound_win2, Default)
        .install();
}
