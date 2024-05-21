// See also Arcropolis' implementation which uses the Smash game data instead https://github.com/Raytwo/ARCropolis/blob/master/src/utils.rs#L90 
// See also this function 0x355aa8c https://discord.com/channels/447823426061860879/516439596322652161/777146109315383306

// Region::None => write!(f, ""),
// Region::Japanese => write!(f, "jp_ja"),
// Region::UsEnglish => write!(f, "us_en"),
// Region::UsFrench => write!(f, "us_fr"),
// Region::UsSpanish => write!(f, "us_es"),
// Region::EuEnglish => write!(f, "eu_en"),
// Region::EuFrench => write!(f, "eu_fr"),
// Region::EuSpanish => write!(f, "eu_es"),
// Region::EuGerman => write!(f, "eu_de"),
// Region::EuDutch => write!(f, "eu_nl"),
// Region::EuItalian => write!(f, "eu_it"),
// Region::EuRussian => write!(f, "eu_ru"),
// Region::Korean => write!(f, "kr_ko"),
// Region::ChinaChinese => write!(f, "zh_cn"),
// Region::TaiwanChinese => write!(f, "zh_tw"),

use skyline::nn::oe::get_desired_language;
use skyline::nn::account;
use crate::logging::*;
use crate::common::offsets::OFFSET_GET_SYSTEM_LOCALE;

pub unsafe fn _set_language_from_system() {
    let desired_language = get_desired_language();
    let locale = desired_language.trim_matches('\0'); // There might be a some extra null bytes at the end
    info!("Setting language to {}", locale);
    rust_i18n::set_locale(locale);
}

pub unsafe fn set_language() {
    nn::account::Initialize();
    mount_save("save\0");
    let language_id = get_language_id_in_savedata();
    unmount_save("save\0");
    if let Ok(save_language_id) = language_id {
        dbg!(&save_language_id);
        let region = get_system_region_from_language_id(save_language_id);
        dbg!(&region);
        info!("Setting language to {}", region);
        rust_i18n::set_locale(format!("{}", region).as_str());
    } else {
        warn!("Could not find language in save data, falling back to us_en");
        rust_i18n::set_locale("us_en");
    }

}



// Stoled from Arcropolis
use skyline::nn;
use smash_arc::Region;
use std::io::{Read, Result, Seek, SeekFrom};

#[repr(u8)]
#[derive(Debug)]
pub enum SaveLanguageId {
    Japanese = 0,
    English,
    French,
    Spanish,
    German,
    Italian,
    Dutch,
    Russian,
    Chinese,
    Taiwanese,
    Korean,
}

impl From<u8> for SaveLanguageId {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Self::Japanese,
            1 => Self::English,
            2 => Self::French,
            3 => Self::Spanish,
            4 => Self::German,
            5 => Self::Italian,
            6 => Self::Dutch,
            7 => Self::Russian,
            8 => Self::Chinese,
            9 => Self::Taiwanese,
            10 => Self::Korean,
            _ => Self::English,
        }
    }
}

pub fn mount_save(mount_path: &str) {
    // TODO: Call nn::fs::CheckMountName
    // This provides a UserHandle and sets the User in a Open state to be used.
    let handle = nn::account::try_open_preselected_user().expect("OpenPreselectedUser should not return false");
    // Obtain the UID for this user
    let uid = nn::account::get_user_id(&handle).expect("GetUserId should return a valid Uid");

    unsafe { nn::fs::MountSaveData(skyline::c_str(&format!("{}\0", mount_path)), &uid) };

    // This closes the UserHandle, making it unusable, and sets the User in a Closed state.
    // Smash will crash if you don't do it.
    nn::account::close_user(handle);
}

pub fn unmount_save(mount_path: &str) {
    // TODO: Call nn::fs::CheckMountName
    unsafe { nn::fs::Unmount(skyline::c_str(&format!("{}\0", mount_path))) };
}

pub fn get_language_id_in_savedata() -> Result<SaveLanguageId> {
    let mut file = std::fs::File::open("save:/save_data/system_data.bin")?;
    file.seek(SeekFrom::Start(0x3c6098)).unwrap();
    let mut language_code = [0u8];
    file.read_exact(&mut language_code).unwrap();
    drop(file);

    Ok(SaveLanguageId::from(language_code[0]))
}

pub fn get_system_region_from_language_id(language: SaveLanguageId) -> Region {
    let system_locale_id = unsafe { *(skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const u8).add(OFFSET_GET_SYSTEM_LOCALE) };
    let system_region_map: &[u32] = &[0, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 3, 3, 4];
    let system_region = system_region_map[system_locale_id as usize];

    match language {
        SaveLanguageId::Japanese => Region::Japanese,
        SaveLanguageId::English => {
            if system_region == 1 {
                // US
                Region::UsEnglish
            } else {
                Region::EuEnglish
            }
        },
        SaveLanguageId::French => {
            if system_region == 1 {
                // US
                Region::UsFrench
            } else {
                Region::EuFrench
            }
        },
        SaveLanguageId::Spanish => {
            if system_region == 1 {
                // US
                Region::UsSpanish
            } else {
                Region::EuSpanish
            }
        },
        SaveLanguageId::German => Region::EuGerman,
        SaveLanguageId::Dutch => Region::EuDutch,
        SaveLanguageId::Italian => Region::EuItalian,
        SaveLanguageId::Russian => Region::EuRussian,
        SaveLanguageId::Chinese => Region::ChinaChinese,
        SaveLanguageId::Taiwanese => Region::TaiwanChinese,
        SaveLanguageId::Korean => Region::Korean,
    }
}