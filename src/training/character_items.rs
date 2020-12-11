use crate::common::consts::*;
use smash::app;
use smash::cpp::l2c_value::LuaConst;
use smash::lib::lua_const::*;

pub fn roll_variation() {
    // Select an item variation from the menu, if applicable
    ()
}

pub struct CharItem {
    fighter_kind: LuaConst,
    item_kind: LuaConst,
    variation: Option<[LuaConst; 8]>,
}

impl CharItem {
    pub fn is_valid(self) -> bool {
        // Checks if the item's character is present
        true
    }

    pub fn give_item(self, fighter_id: FighterId) {
        // Creates the item and gives it to the player or the CPU

        /* Steps:
         * Call is_valid to check if the item's character is present
         * Call ItemModule::is_have_item to make sure the fighter doesn't already have an item
         * Perform any other item-specific checks? May not have to do this
         * Call roll_variation to get a variation if applicable
         * Call ItemModule::have_item to spawn the item
         */
        ()
    }
}

pub const CHARITEM_ALL: [CharItem; 35] = [
    CharItem {
        fighter_kind: FIGHTER_KIND_DIDDY,
        item_kind: ITEM_KIND_BANANA,
        variation: None,
    },
    CharItem {
        // Robin Tome
        fighter_kind: FIGHTER_KIND_REFLET,
        item_kind: ITEM_KIND_BOOK,
        variation: None, // TODO: Look at the lua const ITEM_BOOK_STATUS_KIND_BEFORE_BORN
    },
    CharItem {
        // Banjo-Kazooie Grenade Egg
        fighter_kind: FIGHTER_KIND_BUDDY,
        item_kind: ITEM_KIND_BUDDYBOMB,
        variation: None,
    },
    CharItem {
        // Bob-omb
        fighter_kind: FIGHTER_KIND_DAISY,
        item_kind: ITEM_KIND_BOMBHEI,
        variation: Some([
            // TODO: Are all these needed?
            ITEM_VARIATION_BOMBHEI_NORMAL,
            ITEM_VARIATION_BOMBHEI_OFFSET,
            ITEM_VARIATION_BOMBHEI_TIMEBOMB,
            ITEM_VARIATION_BOMBHEI_ADVENTURE,
            ITEM_VARIATION_BOMBHEI_SUDDENDEATH,
            ITEM_VARIATION_NONE,
            ITEM_VARIATION_NONE,
            ITEM_VARIATION_NONE,
        ]),
    },
    CharItem {
        // Turnip
        fighter_kind: FIGHTER_KIND_DAISY,
        item_kind: ITEM_KIND_DAISYDAIKON,
        variation: Some([
            ITEM_VARIATION_DAISYDAIKON_1, // TODO: Name all the variations
            ITEM_VARIATION_DAISYDAIKON_2,
            ITEM_VARIATION_DAISYDAIKON_3,
            ITEM_VARIATION_DAISYDAIKON_4,
            ITEM_VARIATION_DAISYDAIKON_5,
            ITEM_VARIATION_DAISYDAIKON_6,
            ITEM_VARIATION_DAISYDAIKON_7,
            ITEM_VARIATION_DAISYDAIKON_8,
        ]),
    },
    CharItem {
        // Mr Saturn
        fighter_kind: FIGHTER_KIND_DAISY,
        item_kind: ITEM_KIND_DOSEISAN,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_DIDDY,
        item_kind: ITEM_KIND_DIDDYPEANUTS,
        variation: None,
    },
    CharItem {
        // Sheik Sideb Bomb
        fighter_kind: FIGHTER_KIND_SHEIK,
        item_kind: ITEM_KIND_NONE, // Need to find this item name. FUSIN keeps popping up but there's no item_kind
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_KROOL,
        item_kind: ITEM_KIND_KROOLCROWN,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_LINK,
        item_kind: ITEM_KIND_LINKARROW,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_LINK,
        item_kind: ITEM_KIND_LINKBOMB,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_KOOPAJR,
        item_kind: ITEM_KIND_MECHAKOOPA,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_ROCKMAN,
        item_kind: ITEM_KIND_METALBLADE,
        variation: None,
    },
    CharItem {
        // Villager Apple
        fighter_kind: FIGHTER_KIND_MURABITO,
        item_kind: ITEM_KIND_MURABITOFRUIT,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_PACMAN,
        item_kind: ITEM_KIND_PACMANAPPLE,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_PACMAN,
        item_kind: ITEM_KIND_PACMANBELL,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_PACMAN,
        item_kind: ITEM_KIND_PACMANBOSS,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_PACMAN,
        item_kind: ITEM_KIND_PACMANCHERRY,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_PACMAN,
        item_kind: ITEM_KIND_PACMANKEY,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_PACMAN,
        item_kind: ITEM_KIND_PACMANMELON,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_PACMAN,
        item_kind: ITEM_KIND_PACMANORANGE,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_PACMAN,
        item_kind: ITEM_KIND_PACMANSTRAWBERRY,
        variation: None,
    },
    CharItem {
        // Bob-omb
        fighter_kind: FIGHTER_KIND_PEACH,
        item_kind: ITEM_KIND_BOMBHEI,
        variation: Some([
            // TODO: Are all these needed?
            ITEM_VARIATION_BOMBHEI_NORMAL,
            ITEM_VARIATION_BOMBHEI_OFFSET,
            ITEM_VARIATION_BOMBHEI_TIMEBOMB,
            ITEM_VARIATION_BOMBHEI_ADVENTURE,
            ITEM_VARIATION_BOMBHEI_SUDDENDEATH,
            ITEM_VARIATION_NONE,
            ITEM_VARIATION_NONE,
            ITEM_VARIATION_NONE,
        ]),
    },
    CharItem {
        // Turnip
        fighter_kind: FIGHTER_KIND_PEACH,
        item_kind: ITEM_KIND_PEACHDAIKON,
        variation: Some([
            ITEM_VARIATION_PEACHDAIKON_1,
            ITEM_VARIATION_PEACHDAIKON_2,
            ITEM_VARIATION_PEACHDAIKON_3,
            ITEM_VARIATION_PEACHDAIKON_4,
            ITEM_VARIATION_PEACHDAIKON_5,
            ITEM_VARIATION_PEACHDAIKON_6,
            ITEM_VARIATION_PEACHDAIKON_7,
            ITEM_VARIATION_PEACHDAIKON_8,
        ]),
    },
    CharItem {
        // Mr Saturn
        fighter_kind: FIGHTER_KIND_PEACH,
        item_kind: ITEM_KIND_DOSEISAN,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_RICHTER,
        item_kind: ITEM_KIND_RICHTERHOLYWATER,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_ROBOT,
        item_kind: ITEM_KIND_ROBOTGYRO,
        variation: Some([
            // TODO: Are all these needed?
            ITEM_VARIATION_ROBOTGYRO_1P,
            ITEM_VARIATION_ROBOTGYRO_2P,
            ITEM_VARIATION_ROBOTGYRO_3P,
            ITEM_VARIATION_ROBOTGYRO_4P,
            ITEM_VARIATION_ROBOTGYRO_5P,
            ITEM_VARIATION_ROBOTGYRO_6P,
            ITEM_VARIATION_ROBOTGYRO_7P,
            ITEM_VARIATION_ROBOTGYRO_8P,
        ]),
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_SIMON,
        item_kind: ITEM_KIND_SIMONHOLYWATER,
        variation: None,
    },
    CharItem {
        // Cardboard Box from Taunt
        fighter_kind: FIGHTER_KIND_SNAKE,
        item_kind: ITEM_KIND_SNAKECBOX,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_SNAKE,
        item_kind: ITEM_KIND_SNAKEGRENADE,
        variation: None,
    },
    CharItem {
        // Robin Levin Sword
        fighter_kind: FIGHTER_KIND_REFLET,
        item_kind: ITEM_KIND_THUNDERSWORD,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_TOONLINK,
        item_kind: ITEM_KIND_TOONLINKBOMB,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_WARIO,
        item_kind: ITEM_KIND_WARIOBIKE,
        // Pretty sure these other ones are just the bike parts
        // ITEM_KIND_WARIOBIKEA,
        // ITEM_KIND_WARIOBIKEB,
        // ITEM_KIND_WARIOBIKEC,
        // ITEM_KIND_WARIOBIKED,
        // ITEM_KIND_WARIOBIKEE,
        variation: None,
    },
    CharItem {
        // Villager wood chip?
        fighter_kind: FIGHTER_KIND_MURABITO,
        item_kind: ITEM_KIND_WOOD,
        variation: None,
    },
    CharItem {
        fighter_kind: FIGHTER_KIND_YOUNGLINK,
        item_kind: ITEM_KIND_YOUNGLINKBOMB,
        variation: None,
    },
];
