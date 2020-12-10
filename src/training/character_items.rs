use smash::lib::lua_const::*;
use smash::app:lua_bind::*;
use crate::common::*;
use crate::common::consts::*;

pub fn init() {
    unsafe {
    }
}

pub fn roll_variation() {
    /// Select an item variation from the menu, if applicable
}

struct CharItem {
    const FIGHTER_KIND: i32,
    const ITEM_KIND: i32,
    VARIATION: Option<[i32; 8]>, //type? // is 8 enough for the variation types?
}

impl CharItem {
    pub fn is_valid(self) -> bool {
        /// Checks if the item's character is present
    }
    pub fn give_item(self, fighter_id: FighterId) {
        /// Creates the item and gives it to the player or the CPU

        /* Steps:
        * Call is_valid to check if the item's character is present
        * Call ItemModule::is_have_item to make sure the fighter doesn't already have an item
        * Perform any other item-specific checks? May not have to do this
        * Call roll_variation to get a variation if applicable
        * Call ItemModule::have_item to spawn the item
        */
    }
}

// TODO: Does this belong in common/consts.rs instead?
pub struct CharItemSet : CharItem {
    BANANA: {
        *FIGHTER_KIND_DIDDY,
        *ITEM_KIND_BANANA,
        None,
    },
    BOOK: { // Robin Tome
        *FIGHTER_KIND_REFLET,
        *ITEM_KIND_BOOK,
        None, // TODO: Look at the lua const ITEM_BOOK_STATUS_KIND_BEFORE_BORN
    },
    BUDDYBOMB: { // Banjo-Kazooie Grenade Egg
        *FIGHTER_KIND_BUDDY,
        *ITEM_KIND_BUDDYBOMB,
        None,
    },
    DAISYBOMBHEI: { // Bob-omb
        *FIGHTER_KIND_DAISY,
        *ITEM_KIND_BOMBHEI,
        Some([ // TODO: Are all these needed?
            *ITEM_VARIATION_BOMBHEI_NORMAL,
            *ITEM_VARIATION_BOMBHEI_OFFSET,
            *ITEM_VARIATION_BOMBHEI_TIMEBOMB,
            *ITEM_VARIATION_BOMBHEI_ADVENTURE,
            *ITEM_VARIATION_BOMBHEI_SUDDENDEATH,
        ]),
    },
    DAISYDAIKON: { // Turnip
        *FIGHTER_KIND_DAISY,
        *ITEM_KIND_DAISYDAIKON,
        Some([
            *ITEM_VARIATION_DAISYDAIKON_1, // TODO: Name all the variations
            *ITEM_VARIATION_DAISYDAIKON_2,
            *ITEM_VARIATION_DAISYDAIKON_3,
            *ITEM_VARIATION_DAISYDAIKON_4,
            *ITEM_VARIATION_DAISYDAIKON_5,
            *ITEM_VARIATION_DAISYDAIKON_6,
            *ITEM_VARIATION_DAISYDAIKON_7,
            *ITEM_VARIATION_DAISYDAIKON_8,
        ]),
    },
    DAISYDOSEISAN: { // Mr Saturn
        *FIGHTER_KIND_DAISY,
        *ITEM_KIND_DOSEISAN,
        None,
    },
    DIDDYPEANUTS: {
        *FIGHTER_KIND_DIDDY,
        *ITEM_KIND_DIDDYPEANUTS,
        None,
    },
    FUSIN: { // Sheik Sideb Bomb
        *FIGHTER_KIND_SHEIK,
        *ITEM_KIND_NONE, // Need to find this item name. FUSIN keeps popping up but there's no item_kind
    },
    KROOLCROWN: {
        *FIGHTER_KIND_KROOL,
        *ITEM_KIND_KROOLCROWN,
        None,
    },
    LINKARROW: {
        *FIGHTER_KIND_LINK,
        *ITEM_KIND_LINKARROW,
        None,
    },
    LINKBOMB: {
        *FIGHTER_KIND_LINK,
        *ITEM_KIND_LINKBOMB,
        None,
    },
    MECHAKOOPA: {
        *FIGHTER_KIND_KOOPAJR,
        *ITEM_KIND_MECHAKOOPA,
        None,
    },
    METALBLADE: {
        *FIGHTER_KIND_ROCKMAN,
        *ITEM_KIND_METALBLADE,
        None,
    },
    MURABITOFRUIT: { // Villager Apple
        *FIGHTER_KIND_MURABITO,
        *ITEM_KIND_MURABITOFRUIT,
        None,
    },
    PACMANAPPLE: {
        *FIGHTER_KIND_PACMAN,
        *ITEM_KIND_PACMANAPPLE,
        None,
    },
    PACMANBELL: {
        *FIGHTER_KIND_PACMAN,
        *ITEM_KIND_PACMANBELL,
        None,
    },
    PACMANBOSS: {
        *FIGHTER_KIND_PACMAN,
        *ITEM_KIND_PACMANBOSS,
        None,
    },
    PACMANCHERRY: {
        *FIGHTER_KIND_PACMAN,
        *ITEM_KIND_PACMANCHERRY,
        None,
    },
    PACMANKEY: {
        *FIGHTER_KIND_PACMAN,
        *ITEM_KIND_PACMANKEY,
        None,
    },
    PACMANMELON: {
        *FIGHTER_KIND_PACMAN,
        *ITEM_KIND_PACMANMELON,
        None,
    },
    PACMANORANGE: {
        *FIGHTER_KIND_PACMAN,
        *ITEM_KIND_PACMANORANGE,
        None,
    },
    PACMANSTRAWBERRY: {
        *FIGHTER_KIND_PACMAN,
        *ITEM_KIND_PACMANSTRAWBERRY,
        None,
    },
    PEACHBOMBHEI: { // Bob-omb
        *FIGHTER_KIND_PEACH,
        *ITEM_KIND_BOMBHEI,
        Some([ // TODO: Are all these needed?
            *ITEM_VARIATION_BOMBHEI_NORMAL,
            *ITEM_VARIATION_BOMBHEI_OFFSET,
            *ITEM_VARIATION_BOMBHEI_TIMEBOMB,
            *ITEM_VARIATION_BOMBHEI_ADVENTURE,
            *ITEM_VARIATION_BOMBHEI_SUDDENDEATH,
        ]),
    },
    PEACHDAIKON: { // Turnip
        *FIGHTER_KIND_PEACH,
        *ITEM_KIND_PEACHDAIKON,
        Some([
            *ITEM_VARIATION_PEACHDAIKON_1,
            *ITEM_VARIATION_PEACHDAIKON_2,
            *ITEM_VARIATION_PEACHDAIKON_3,
            *ITEM_VARIATION_PEACHDAIKON_4,
            *ITEM_VARIATION_PEACHDAIKON_5,
            *ITEM_VARIATION_PEACHDAIKON_6,
            *ITEM_VARIATION_PEACHDAIKON_7,
            *ITEM_VARIATION_PEACHDAIKON_8,
        ]),
    },
    PEACHDOSEISAN: { // Mr Saturn
        *FIGHTER_KIND_PEACH,
        *ITEM_KIND_DOSEISAN,
        None,
    },
    RICHTERHOLYWATER: {
        *FIGHTER_KIND_RICHTER,
        *ITEM_KIND_RICHTERHOLYWATER,
        None,
    },
    ROBOTGYRO: {
        *FIGHTER_KIND_ROBOT,
        *ITEM_KIND_ROBOTGYRO,
        Some([ // TODO: Are all these needed?
            *ITEM_VARIATION_ROBOTGYRO_1P,
            *ITEM_VARIATION_ROBOTGYRO_2P,
            *ITEM_VARIATION_ROBOTGYRO_3P,
            *ITEM_VARIATION_ROBOTGYRO_4P,
            *ITEM_VARIATION_ROBOTGYRO_5P,
            *ITEM_VARIATION_ROBOTGYRO_6P,
            *ITEM_VARIATION_ROBOTGYRO_7P,
            *ITEM_VARIATION_ROBOTGYRO_8P,
        ]),
    },
    SIMONHOLYWATER: {
        *FIGHTER_KIND_SIMON,
        *ITEM_KIND_SIMONHOLYWATER,
        None,
    },
    SNAKECBOX: { // Cardboard Box from Taunt
        *FIGHTER_KIND_SNAKE,
        *ITEM_KIND_SNAKECBOX,
        None,
    },
    SNAKEGRENADE: {
        *FIGHTER_KIND_SNAKE,
        *ITEM_KIND_SNAKEGRENADE,
        None,
    },
    THUNDERSWORD: { // Robin Levin Sword
        *FIGHTER_KIND_REFLET,
        *ITEM_KIND_THUNDERSWORD,
        None,
    },
    TOONLINKBOMB: {
        *FIGHTER_KIND_TOONLINK,
        *ITEM_KIND_TOONLINKBOMB,
        None,
    },
    WARIOBIKE: {
        *FIGHTER_KIND_WARIO,
        *ITEM_KIND_WARIOBIKE,
        // Pretty sure these other ones are just the bike parts
        // ITEM_KIND_WARIOBIKEA,
        // ITEM_KIND_WARIOBIKEB,
        // ITEM_KIND_WARIOBIKEC,
        // ITEM_KIND_WARIOBIKED,
        // ITEM_KIND_WARIOBIKEE,
        None,
    },
    WOOD: { // Villager wood chip?
        *FIGHTER_KIND_MURABITO,
        *ITEM_KIND_WOOD,
        None,
    },
    YOUNGLINKBOMB: {
        *FIGHTER_KIND_YOUNGLINK,
        *ITEM_KIND_YOUNGLINKBOMB,
        None,
    },
}

impl CharItemSet {
    pub fn get_valid_items(self) {
        /// Return the CharItems for which is_valid == true
    }
}