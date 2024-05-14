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
    crate::{
        EDGE_EXIST
    }
};

unsafe extern "C" fn cloud_sound_win1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_cloud_win01"));
    }
    if EDGE_EXIST == true{
        frame(agent.lua_state_agent, 28.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_cloud_win04"));
        }
    } else {
        frame(agent.lua_state_agent, 75.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_cloud_win01"));
        }
    }
}

pub fn install() {
    Agent::new("cloud")
        .sound_acmd("sound_win1", cloud_sound_win1, Default)
        .install();
}
