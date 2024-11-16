use smash::app::{self};

use crate::common::consts::*;
use crate::common::*;
use training_mod_sync::*;

static SHIELD_STICK_DIRECTION: RwLock<Direction> = RwLock::new(Direction::OUT);

pub fn roll_direction() {
    assign(
        &SHIELD_STICK_DIRECTION,
        read(&MENU).shield_tilt.get_random(),
    );
}

pub fn mod_get_stick_x(module_accessor: &mut app::BattleObjectModuleAccessor) -> Option<f32> {
    get_angle(module_accessor).map(|a| a.cos() as f32)
}

pub fn mod_get_stick_y(module_accessor: &mut app::BattleObjectModuleAccessor) -> Option<f32> {
    get_angle(module_accessor).map(|a| a.sin() as f32)
}

fn get_angle(module_accessor: &mut app::BattleObjectModuleAccessor) -> Option<f64> {
    if !is_operation_cpu(module_accessor) {
        return None;
    }
    let stick_direction = read(&SHIELD_STICK_DIRECTION);
    stick_direction.into_angle()
}
