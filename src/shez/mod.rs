use {
    smash::{
        app::{lua_bind::*, sv_animcmd::*, *},
        hash40,
        lib::{lua_const::*, L2CValue},
        lua2cpp::*,
        phx::*,
    },
    smash_script::*,
    smashline::*,
};

mod victory;

pub fn is_shez(boma: *mut BattleObjectModuleAccessor) -> bool {
    unsafe {
        let color: i32 = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        crate::MARKED_COLORS[color as usize]
    }
}
pub fn install() {
    victory::install();
}
