/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
extern crate crossterm;
extern crate crossterm_cursor;
extern crate crossterm_input;
extern crate crossterm_screen;
extern crate crossterm_style;
use crossterm_style::Color;

// Internal includes.
use super::ConsoleChar;
use crate::creature::background::SpeciesType;
use crate::creature::CreatureStats;
use crate::faction::Faction;
use crate::io::Display;
use crate::io::console::ConsoleDisplay;
use crate::rrl_math::Position;
use crate::stats::Stat;
use crate::world::{Tilemap, VisibilityMap};

impl Display for ConsoleDisplay {
    fn choose_species(&mut self, options: &Vec<SpeciesType>) -> SpeciesType {

        let mut keep_going = true;
        let mut option = SpeciesType::Human;

        while keep_going {
            self.clear();

            self.put_string(1, 1, "Select your character's species:", Color::Grey, Color::Black );

            for (i, species_type) in options.iter().enumerate() {
                let formatted = format!("{}) {}", (1 + i), species_type.to_string());
                self.put_string(1, 3_i32 + i as i32, &formatted, Color::Grey, Color::Black);
            }

            self.present();

            if let Some(index) = self.get_char().to_digit(10) {
                let index = index as usize;
                let index = match index { 0 => 10, _ => index - 1 };
                if index < options.len() {
                    option = options[index];
                    keep_going = false;
                }
            }
        }

        self.clear();

        option
    }

    fn display_stats(&mut self, stats: CreatureStats) {
        self.put_stat(42, 2, "Strength", stats.strength());
        self.put_stat(42, 3, "Agility", stats.agility());
        self.put_stat(42, 4, "Coordination", stats.coordination());
        self.put_stat(42, 5, "Endurance", stats.endurance());
        self.put_health(42, 6, "Health", stats.endurance().value(), stats.health());
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
        let ch;
        if faction == Faction::new(0) {
            ch = ConsoleChar::new('@', Color::Grey, Color::Black);
        } else {
            ch = ConsoleChar::new('C', Color::Grey, Color::Black);
        }
        self.put_console_char(display_pos_x, display_pos_y, ch);
    }

    fn write_map(&mut self, view_pos: Position, map: &Tilemap, vis: &VisibilityMap) {
        self.write_map_impl(view_pos, map, vis);
    }
}
