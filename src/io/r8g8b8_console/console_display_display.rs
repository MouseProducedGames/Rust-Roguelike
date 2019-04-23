/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use crossterm_style::Color;
use specs::{Entity, ReadStorage};

// Standard includes.

// Internal includes.
use super::{ConsoleChar, ConsoleDisplay, Darker};
use crate::background::{OriginType, SpeciesType};
use crate::bodies::Body;
use crate::data_types::Name;
use crate::factions::Faction;
use crate::io::{Display, DisplayOption};
use crate::items::{Inventory, Item};
use crate::rrl_math::{Displacement, Position};
use crate::stats::{CreatureStats, Stat};
use crate::world::{Tilemap, VisibilityMap, VisibilityType};

impl Display for ConsoleDisplay {
    fn blit_body(&mut self, names: ReadStorage<Name>, body: &Body) {
        self.clear();

        self.put_string(1, 1, "Body:", Color::Grey, Color::Black);

        for (i, body_slot) in body.get().values().enumerate() {
            if let Some(name) = names.get(body_slot.item()) {
                let formatted = format!(
                    "{}) {}: {}",
                    if i < 26 {
                        (b'a' + (i as u8)) as char
                    } else {
                        (b'A' + ((i - 26) as u8)) as char
                    },
                    body_slot.name(),
                    name
                );
                self.put_string(1, 3_i32 + i as i32, &formatted, Color::Grey, Color::Black);
            }
        }

        self.present();
    }

    fn blit_inventory(&mut self, names: ReadStorage<Name>, inventory: &Inventory) {
        self.clear();

        self.put_string(1, 1, "Inventory:", Color::Grey, Color::Black);

        for (i, entity) in inventory.get().iter().enumerate() {
            if let Some(name) = names.get(*entity) {
                let formatted = format!(
                    "{}) {}",
                    if i < 26 {
                        (b'a' + (i as u8)) as char
                    } else {
                        (b'A' + ((i - 26) as u8)) as char
                    },
                    name
                );
                self.put_string(1, 3_i32 + i as i32, &formatted, Color::Grey, Color::Black);
            }
        }

        self.present();
    }

    fn blit_items(&mut self, names: ReadStorage<Name>, items: &[Entity]) {
        self.clear();

        self.put_string(1, 1, "Items:", Color::Grey, Color::Black);

        for (i, entity) in items.iter().enumerate() {
            if let Some(name) = names.get(*entity) {
                let formatted = format!(
                    "{}) {}",
                    if i < 26 {
                        (b'a' + (i as u8)) as char
                    } else {
                        (b'A' + ((i - 26) as u8)) as char
                    },
                    name
                );
                self.put_string(1, 3_i32 + i as i32, &formatted, Color::Grey, Color::Black);
            }
        }

        self.present();
    }

    fn choose_display_option(&mut self, options: &'static [DisplayOption]) -> DisplayOption {
        self.choose("Select display option:", options)
    }

    fn choose_origin(&mut self, options: &'static [OriginType]) -> OriginType {
        self.choose("Select your character's origin:", options)
    }

    fn choose_species(&mut self, options: &'static [SpeciesType]) -> SpeciesType {
        self.choose("Select your character's species:", options)
    }

    fn display_stats(&mut self, stats: CreatureStats) {
        self.put_stat(42, 2, "Strength", stats.strength());
        self.put_stat(42, 3, "Agility", stats.agility());
        self.put_stat(42, 4, "Coordination", stats.coordination());
        self.put_stat(42, 5, "Endurance", stats.endurance());
        self.put_stat(42, 6, "Perception", stats.perception());
        self.put_health(42, 8, "Health", stats.endurance().value(), stats.health());
    }

    fn get_char(&self) -> char {
        self.get_char_impl()
    }

    fn present(&mut self) {
        self.present_impl();
    }

    fn write_creature(&mut self, faction: Faction, creature_pos: Position, view_pos: Position) {
        let disp = creature_pos - view_pos;
        if (disp.x < -17) || (disp.x > 17) || (disp.y < -17) || (disp.y > 17) {
            return;
        }
        let (display_pos_x, display_pos_y) = (18 + disp.x, 18 + disp.y);
        let ch = if faction == Faction::new(0) {
            ConsoleChar::new(
                '@',
                Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                },
                Color::Rgb { r: 0, g: 0, b: 0 },
            )
        } else {
            ConsoleChar::new(
                'C',
                Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                },
                Color::Rgb { r: 0, g: 0, b: 0 },
            )
        };
        self.put_console_char(display_pos_x, display_pos_y, ch);
    }

    fn write_item(&mut self, item: Item, item_pos: Position, view_pos: Position) {
        let disp = item_pos - view_pos;
        if (disp.x < -17) || (disp.x > 17) || (disp.y < -17) || (disp.y > 17) {
            return;
        }
        let (display_pos_x, display_pos_y) = (18 + disp.x, 18 + disp.y);
        let ch = self.get_item_graphics(item.icon_id());
        self.put_console_char(display_pos_x, display_pos_y, ch);
    }

    fn write_map(&mut self, view_pos: Position, map: &Tilemap, vis: &VisibilityMap) {
        let (map_graphics, back_buffer) = self.get_draw_info();
        for view_addend_y in -17..18_i32 {
            let display_pos_y = (18 + view_addend_y) as usize;
            for view_addend_x in -17..18_i32 {
                let display_pos_x = (18 + view_addend_x) as usize;
                let map_pos = view_pos + Displacement::new(view_addend_x, view_addend_y);
                let ch;
                match vis.value_pos(map_pos) {
                    VisibilityType::None => ch = map_graphics[0],
                    VisibilityType::Seen => {
                        let tile_type = map.tile_type_pos(map_pos);
                        ch = map_graphics[tile_type as usize].darker();
                    }
                    VisibilityType::Visible => {
                        let tile_type = map.tile_type_pos(map_pos);
                        ch = map_graphics[tile_type as usize];
                    }
                }

                *back_buffer.value_mut(display_pos_y, display_pos_x) = ch;
            }
        }
    }
}