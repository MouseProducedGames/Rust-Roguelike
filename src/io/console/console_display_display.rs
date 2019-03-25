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
use crossterm_style::{Color, Colored};

// Internal includes.
use super::super::super::multidim::Multidim;
use super::{ConsoleChar, Darker};
use crate::creature::background::SpeciesType;
use crate::creature::CreatureStats;
use crate::faction::Faction;
use crate::io::Display;
use crate::io::console::ConsoleDisplay;
use crate::rrl_math::{Displacement, Position};
use crate::stats::{Stat, StatModifier};
use crate::world::{Tilemap, VisibilityMap, VisibilityType};

impl Display for ConsoleDisplay {
    fn choose_species(&mut self, options: &Vec<SpeciesType>) -> SpeciesType {
        println!("{}{}", Colored::Fg(Color::Grey), Colored::Bg(Color::Black));

        let mut keep_going = true;
        let mut option = SpeciesType::Human;

        while keep_going {
            self.clear();

            for (i, species_type) in options.iter().enumerate() {
                println!("{}) {}", (1 + i), species_type.to_string());
            }

            println!();
            println!(" Select your character's species: ");

            if let Some(index) = self.get_char().to_digit(10) {
                let index = index as usize;
                let index = match index { 0 => 10, _ => index };
                if index < options.len() {
                    option = options[index - 1];
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
        
        // println!("{}{}", Colored::Fg(Color::Grey), Colored::Bg(Color::Black));

        /* self.move_cursor(42, 2);
        println!(
            "Strength.....: {:>2} : {:+>2}",
            stats.strength().value(),
            stats.strength().modifier()
        );
        self.move_cursor(42, 3);
        println!(
            "Agility......: {:>2} : {:+>2}",
            stats.agility().value(),
            stats.agility().modifier()
        );
        self.move_cursor(42, 4);
        println!(
            "Coordination.: {:>2} : {:+>2}",
            stats.coordination().value(),
            stats.strength().modifier()
        );
        self.move_cursor(42, 5);
        println!(
            "Endurance....: {:>2} : {:+>2}",
            stats.endurance().value(),
            stats.endurance().modifier()
        );
        self.move_cursor(42, 7);
        println!(
            "Health.......: {:>2}/{:>2}",
            stats.health().value(),
            stats.endurance().value()
        ); */
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
        /* *self.buffers[self.back_buffer_index]
            .value_mut(display_pos_y as usize, display_pos_x as usize) = ch; */
        // self.put_char( 18 + disp.x, 18 + disp.y, 'C' );
    }

    fn write_map(&mut self, view_pos: Position, map: &Tilemap, vis: &VisibilityMap) {
        self.write_map_impl(view_pos, map, vis);
    }
}
