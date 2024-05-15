use {
    crate::MASTER_EXIST,
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

unsafe extern "C" fn roy_sound_win3(agent: &mut L2CAgentBase) {
    if MASTER_EXIST == true {
        frame(agent.lua_state_agent, 0.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_roy_win_master"));
        }
        frame(agent.lua_state_agent, 33.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_roy_special_n01_win03"));
        }
        frame(agent.lua_state_agent, 34.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_roy_attackair_b01_win03"));
        }
        frame(agent.lua_state_agent, 50.0);
        if macros::is_excute(agent) {
            macros::STOP_SE(agent, Hash40::new("se_roy_special_n01_win03"));
        }
        frame(agent.lua_state_agent, 122.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_roy_smash_s01_win03"));
        }
    } else {
        frame(agent.lua_state_agent, 33.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_roy_special_n01_win03"));
        }
        frame(agent.lua_state_agent, 34.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_roy_attackair_b01_win03"));
        }
        frame(agent.lua_state_agent, 40.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_roy_win02"));
        }
        frame(agent.lua_state_agent, 50.0);
        if macros::is_excute(agent) {
            macros::STOP_SE(agent, Hash40::new("se_roy_special_n01_win03"));
        }
        frame(agent.lua_state_agent, 122.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_roy_smash_s01_win03"));
        }
    }
}

pub fn install() {
    Agent::new("roy")
        .sound_acmd("sound_win3_shez", roy_sound_win3, Default)
        .install();
}
