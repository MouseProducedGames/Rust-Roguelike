# Rust Roguelike

Rust Roguelike is a text-based roguelike written in Rust, and licensed under the MIT license.

# Current State

* Wander around a small map with randomly-placed walls and floors.
* Movement using the number pad.

## Roadmap

* <span style="color:green">Completed items are green.</span>
* <span style="color:yellow">items in progress are yellow.</span>
* <span style="color:red">finished items are red.</span>

### Roadmap for v0.1

<span style="color:green">* Model-Controller for creatures (AI and player).</span>
<span style="color:green">  * Random wander AI.</span>
<span style="color:green">* Very simple map gen.</span>
<span style="color:green">* Text Map display.</span>
<span style="color:green">* Input.</span>
<span style="color:green">* Tile system.</span>
<span style="color:green">* Game state management.</span>

### Roadmap for v0.2

<span style="color:red">* Model-View-Controller for creatures (AI and player).</span>
<span style="color:red">* Basic Attribute system (Strength, Agility, Hit Points).</span>
<span style="color:red">* Visibility and seen tile system.</span>

### Roadmap for v0.3

<span style="color:red">* AI support for visibility-based movement/exploration.</span>
<span style="color:green">  * Exploration implemented through random wander.</span>
<span style="color:red">  * Movement over visible and seen tiles imlemented through A*.</span>

### Roadmap for v0.4

<span style="color:red">* Basic faction system.</span>
<span style="color:red">  * Player will be in one faction; all AI in a second facton.</span>

### Roadmap for v0.5

<span style="color:red">* Love-Fear/Hate AI system.</span>
<span style="color:red">  * Cluster with friends (love strong friends more).</span>
<span style="color:red">  * Run from strong enemies.</span>
<span style="color:red">  * Attack weak enemies.</span>
<span style="color:red">* Basic combat system.</span>
<span style="color:red">  * AI will hate weaker creatures of other factions.</span>
<span style="color:red">  * AI will fear stronger creatures of other factions.</span>
<span style="color:red">  * Creatures will not be able to attack creatures of their own faction.</span>
<span style="color:red">  * Player can attack creatures of other factions by attempting to move to their tile.</span>

### Roadmap for v0.6

<span style="color:red">* Full Attribute system (???).</span>
<span style="color:red">* ???</span>

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

<span style="color:red">* Rust is a statically-typed language focused on safety and speed.
<span style="color:red">* Rust provides static memory management, where the lifetimes of most objects can be known at compile tme.
<span style="color:red">* Rust has a robust, albiet developing, community and toolset.
<span style="color:red">* I wanted to learn a new programming language.

## Why a Roguelike?

Rust Roguelike exists as a Rust programming exercise due to the following factors:

<span style="color:red">* The best way for a games programmer to learn how to make games, is to make games.
<span style="color:red">* The best way to learn how to program in a language, is to make programs in that language.
<span style="color:red">  * These may seem self-evident, but are worth noting.
<span style="color:red">* *In my opinion*, text-based Roguelikes are an excellent way for an experienced solo programmer to learn how to program in a language.
<span style="color:red">  * They require no art assets beyond the fonts text present on the computer.
<span style="color:red">  * They contain a complete game loop: A player-character, creature AI, movement system, display, input, event system, game logic, and game mechanics.
<span style="color:red">  * With proper separation of concerns into modules, a graphical display can be added after completion, or during development.
<span style="color:red">  * With a proper event system, an audio system can be plugged into your event system later.
<span style="color:red">  * At the end of it, you have a potentially-salable game.
<span style="color:red">  * The MIT license was chosen so that this project could be used, by anyone, as the basis for a salable game.

Rust Roguelike is (C) MouseProducedGames under the MIT license.