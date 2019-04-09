/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use crossterm_style::Color;

// Standard includes.

// Internal includes.
use super::{ConsoleChar, ConsoleDisplay, Darker};
use crate::background::{OriginType, SpeciesType};
use crate::factions::Faction;
use crate::io::Display;
use crate::items::{Inventory, Item};
use crate::rrl_math::{Displacement, Position};
use crate::stats::{CreatureStats, Stat};
use crate::world::{Tilemap, VisibilityMap, VisibilityType};

impl Display for ConsoleDisplay {
    fn blit_inventory(&mut self, inventory: &Inventory) {
        self.clear();

        self.put_string(1, 1, "Inventory:", Color::Grey, Color::Black);

        for (i, item) in inventory.get().iter().enumerate() {
            let name = match item {
                Item::Generic(name, _) => name,
            };

            let formatted = format!("{}) {}", (1 + i), name);
            self.put_string(1, 3_i32 + i as i32, &formatted, Color::Grey, Color::Black);
        }

        self.present();
    }

    fn choose_origin(&mut self, options: &[OriginType]) -> OriginType {
        let mut keep_going = true;
        let mut option = OriginType::Farmer;

        while keep_going {
            self.clear();

            self.put_string(
                1,
                1,
                "Select your character's origin:",
                Color::Grey,
                Color::Black,
            );

            for (i, option) in options.iter().enumerate() {
                let formatted = format!("{}) {}", (1 + i), option.to_str());
                self.put_string(1, 3_i32 + i as i32, &formatted, Color::Grey, Color::Black);
            }

            self.present();

            if let Some(index) = self.get_char().to_digit(10) {
                let index = index as usize;
                let index = match index {
                    0 => 10,
                    _ => index - 1,
                };
                if index < options.len() {
                    option = options[index];
                    keep_going = false;
                }
            }
        }

        self.clear();

        option
    }

    fn choose_species(&mut self, options: &[SpeciesType]) -> SpeciesType {
        let mut index: usize = 0;

        self.clear();
        loop {
            self.clear_back_buffer();
            self.put_string(
                1,
                1,
                "Select your character's species:",
                Color::Grey,
                Color::Black,
            );

            for (i, species_type) in options.iter().enumerate() {
                let formatted = format!(
                    "   {:<10} {}",
                    species_type.to_str(),
                    species_type.to_short_description_str()
                );
                self.put_string(1, 3_i32 + i as i32, &formatted, Color::Grey, Color::Black);
            }

            self.put_string(1, 3_i32 + index as i32, "->", Color::Yellow, Color::Black);

            self.put_string(
                4,
                20,
                options[index].to_long_description_str(),
                Color::Grey,
                Color::Black,
            );

            self.present();

            match self.get_char() {
                // Return.
                '\r' => {
                    self.clear();
                    return options[index];
                }
                '2' => {
                    index = (index + 1) % options.len();
                }
                '8' => {
                    index = index.wrapping_sub(1);
                    if index > options.len() {
                        index = options.len() - 1;
                    }
                }
                _ => (),
            }
        }
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
            ConsoleChar::new('@', Color::Grey, Color::Black)
        } else {
            ConsoleChar::new('C', Color::Grey, Color::Black)
        };
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
