#![feature(proc_macro_hygiene)]
#![feature(const_mut_refs)]
#![feature(exclusive_range_pattern)]
#![feature(once_cell)]
#![allow(
    clippy::borrow_interior_mutable_const,
    clippy::not_unsafe_ptr_arg_deref,
    clippy::missing_safety_doc,
    clippy::wrong_self_convention,
    clippy::option_map_unit_fn,
    clippy::float_cmp
)]

pub mod common;
mod hazard_manager;
mod hitbox_visualizer;
mod training;

#[cfg(test)]
mod test;

use crate::common::consts::get_menu_from_url;
use crate::common::*;
use crate::events::{Event, EVENT_QUEUE};

use skyline::libc::{c_char, mkdir};
use skyline::nro::{self, NroInfo};
use std::fs;

use crate::training::frame_counter;
use owo_colors::OwoColorize;
use training_mod_consts::OnOff;
use training_mod_tui::Color;
use crate::menu::{quick_menu_loop, web_session_loop};

fn nro_main(nro: &NroInfo<'_>) {
    if nro.module.isLoaded {
        return;
    }

    if nro.name == "common" {
        skyline::install_hooks!(
            training::shield::handle_sub_guard_cont,
            training::directional_influence::handle_correct_damage_vector_common,
            training::tech::handle_change_status,
        );
    }
}

macro_rules! c_str {
    ($l:tt) => {
        [$l.as_bytes(), "\u{0}".as_bytes()].concat().as_ptr()
    };
}

#[skyline::main(name = "training_modpack")]
pub fn main() {
    macro_rules! log {
        ($($arg:tt)*) => {
            println!("{}{}", "[Training Modpack] ".green(), format!($($arg)*));
        };
    }

    log!("Initialized.");
    unsafe {
        EVENT_QUEUE.push(Event::smash_open());
    }

    hitbox_visualizer::hitbox_visualization();
    hazard_manager::hazard_manager();
    training::training_mods();
    nro::add_hook(nro_main).unwrap();

    unsafe {
        mkdir(c_str!("sd:/TrainingModpack/"), 777);
    }

    let ovl_path = "sd:/switch/.overlays/ovlTrainingModpack.ovl";
    if fs::metadata(ovl_path).is_ok() {
        log!("Removing ovlTrainingModpack.ovl...");
        fs::remove_file(ovl_path).unwrap();
    }

    log!("Performing version check...");
    release::version_check();

    let menu_conf_path = "sd:/TrainingModpack/training_modpack_menu.conf";
    log!("Checking for previous menu in training_modpack_menu.conf...");
    if fs::metadata(menu_conf_path).is_ok() {
        let menu_conf = fs::read(menu_conf_path).unwrap();
        if menu_conf.starts_with(b"http://localhost") {
            log!("Previous menu found, loading from training_modpack_menu.conf");
            unsafe {
                MENU = get_menu_from_url(MENU, std::str::from_utf8(&menu_conf).unwrap(), false);
                DEFAULTS_MENU = get_menu_from_url(
                    DEFAULTS_MENU,
                    std::str::from_utf8(&menu_conf).unwrap(),
                    true,
                );
            }
        } else {
            log!("Previous menu found but is invalid.");
        }
    } else {
        log!("No previous menu file found.");
    }

    if is_emulator() {
        unsafe {
            DEFAULTS_MENU.quick_menu = OnOff::On;
            MENU.quick_menu = OnOff::On;
        }
    }

    std::thread::spawn(|| loop {
        std::thread::sleep(std::time::Duration::from_secs(10));
        unsafe {
            while let Some(event) = EVENT_QUEUE.pop() {
                let host = "https://my-project-1511972643240-default-rtdb.firebaseio.com";
                let path = format!(
                    "/event/{}/device/{}/{}.json",
                    event.event_name, event.device_id, event.event_time
                );

                let url = format!("{}{}", host, path);
                minreq::post(url).with_json(&event).unwrap().send().ok();
            }
        }
    });

    std::thread::spawn(|| {
        unsafe {
            quick_menu_loop()
        }
    });

    std::thread::spawn(|| {
        unsafe {
            web_session_loop()
        }
    });
}
