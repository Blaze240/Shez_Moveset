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

unsafe extern "C" fn roy_sound_win1(agent: &mut L2CAgentBase) {
    if MASTER_EXIST == true {
        frame(agent.lua_state_agent, 0.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_roy_win_master"));
        }
    } else {
        frame(agent.lua_state_agent, 20.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_roy_win01"));
        }
        frame(agent.lua_state_agent, 44.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_roy_smash_s01_win01"));
        }
        frame(agent.lua_state_agent, 120.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_roy_smash_l01_win01"));
        }
    }
}

pub fn install() {
    Agent::new("roy")
        .sound_acmd("sound_win1_shez", roy_sound_win1, Default)
        .install();
}
