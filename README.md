# Rust Roguelike

Rust Roguelike is a text-based roguelike written in Rust, and licensed under the MIT license.

# Current State

* Wander around a small map with randomly-placed walls and floors.
* Movement using the number pad.

## Roadmap

* Completed items do not have any prefix attached.
* + Items in progress are preceded by a plus sign.
* ! Uncompleted items are preceded by an exclamation mark.

### Roadmap for v0.1

* Model-Controller for creatures (AI and player).</span>
  * Random wander AI.</span>
* Very simple map gen.</span>
* Text Map display.</span>
* Input.</span>
* Tile system.</span>
* Game state management.</span>

### Roadmap for v0.2

* ! Model-View-Controller for creatures (AI and player).</span>
* ! Basic Attribute system (Strength, Agility, Hit Points).</span>
* ! Visibility and seen tile system.</span>

### Roadmap for v0.3

* ! AI support for visibility-based movement/exploration.</span>
  * Exploration implemented through random wander.</span>
  * ! Movement over visible and seen tiles imlemented through A*.</span>

### Roadmap for v0.4

* ! Basic faction system.</span>
  * ! Player will be in one faction; all AI in a second facton.</span>

### Roadmap for v0.5

* ! Love-Fear/Hate AI system.</span>
  * ! Cluster with friends (love strong friends more).</span>
  * ! Run from strong enemies.</span>
  * ! Attack weak enemies.</span>
* ! Basic combat system.</span>
  * ! AI will hate weaker creatures of other factions.</span>
  * ! AI will fear stronger creatures of other factions.</span>
  * ! Creatures will not be able to attack creatures of their own faction.</span>
  * ! Player can attack creatures of other factions by attempting to move to their tile.</span>

### Roadmap for v0.6

* ! Full Attribute system (???).</span>
* ! ???</span>

### Timelines for these items are long or uncertain

These items have no completion status as they are uncertain or not yet on the roadmap.

* Hard-coded ability system.
* Items (armour, melee weaponds, shields).
* Species (Dwarf, Elf, Halfling, Human, Orc).
* Spell system (designed using the ability system).
* Spells.
* Magic items.

## Why Rust Roguelike?

I'm not good at names.

## Why Rust?

* ! Rust is a statically-typed language focused on safety and speed.
* ! Rust provides static memory management, where the lifetimes of most objects can be known at compile tme.
* ! Rust has a robust, albiet developing, community and toolset.
* ! I wanted to learn a new programming language.

## Why a Roguelike?

Rust Roguelike exists as a Rust programming exercise due to the following factors:

* ! The best way for a games programmer to learn how to make games, is to make games.
* ! The best way to learn how to program in a language, is to make programs in that language.
  * ! These may seem self-evident, but are worth noting.
* ! *In my opinion*, text-based Roguelikes are an excellent way for an experienced solo programmer to learn how to program in a language.
  * ! They require no art assets beyond the fonts text present on the computer.
  * ! They contain a complete game loop: A player-character, creature AI, movement system, display, input, event system, game logic, and game mechanics.
  * ! With proper separation of concerns into modules, a graphical display can be added after completion, or during development.
  * ! With a proper event system, an audio system can be plugged into your event system later.
  * ! At the end of it, you have a potentially-salable game.
  * ! The MIT license was chosen so that this project could be used, by anyone, as the basis for a salable game.

Rust Roguelike is (C) MouseProducedGames under the MIT license.