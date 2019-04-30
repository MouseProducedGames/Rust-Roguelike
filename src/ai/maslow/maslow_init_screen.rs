/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::World;

// Standard includes.
use std::sync::{Arc, Mutex};

// Internal includes.
use super::{
    faction_reaction, random_wander, MaslowFuncWrapper, MaslowFuncWrapperLookup, MaslowNode,
    MaslowNodeLookup, MaslowTree, MaslowTreeLookup,
};
use crate::screens::{Screen, ScreenPushWrapper, ScreenState};

enum MaslowInitState {
    World,
    MaslowFuncWrappers,
    MaslowNodes,
    MaslowTrees,
    Finished,
}

pub struct MaslowInitScreen {
    maslow_init_state: MaslowInitState,
    state: ScreenState,
}

impl MaslowInitScreen {
    pub fn new() -> Self {
        Self {
            maslow_init_state: MaslowInitState::World,
            state: ScreenState::Started,
        }
    }

    fn init_maslow_func_wrappers(&self, world: &mut World) {
        let maslow_func_wrapper_lookup = world
            .write_resource::<Arc<Mutex<MaslowFuncWrapperLookup>>>()
            .clone();
        let mut maslow_func_wrapper_lookup = maslow_func_wrapper_lookup.lock().unwrap();

        let random_wander_func =
            MaslowFuncWrapper::new("Random Wander", Arc::new(Mutex::new(random_wander)));
        maslow_func_wrapper_lookup.insert(random_wander_func);

        let faction_reaction_func =
            MaslowFuncWrapper::new("Faction Reaction", Arc::new(Mutex::new(faction_reaction)));
        maslow_func_wrapper_lookup.insert(faction_reaction_func);
    }

    fn init_maslow_nodes(&self, world: &mut World) {
        let maslow_node_lookup = world
            .write_resource::<Arc<Mutex<MaslowNodeLookup>>>()
            .clone();
        let mut maslow_node_lookup = maslow_node_lookup.lock().unwrap();

        let maslow_func_wrapper_lookup = world
            .write_resource::<Arc<Mutex<MaslowFuncWrapperLookup>>>()
            .clone();
        let maslow_func_wrapper_lookup = maslow_func_wrapper_lookup.lock().unwrap();

        // Unwrap is used when getting from lookup, as it is an exception if
        // the indicated values do not exist.

        let faction_reaction_node = Arc::new(Mutex::new(MaslowNode::new(
            &"Faction Reaction",
            maslow_func_wrapper_lookup
                .get("Faction Reaction")
                .unwrap()
                .clone(),
            &[],
        )));
        maslow_node_lookup.insert(faction_reaction_node);

        let random_wander_node = Arc::new(Mutex::new(MaslowNode::new(
            &"Random Wander",
            maslow_func_wrapper_lookup
                .get("Random Wander")
                .unwrap()
                .clone(),
            &[],
        )));
        maslow_node_lookup.insert(random_wander_node);
    }

    fn init_maslow_trees(&self, world: &mut World) {
        let maslow_tree_lookup = world
            .write_resource::<Arc<Mutex<MaslowTreeLookup>>>()
            .clone();
        let mut maslow_tree_lookup = maslow_tree_lookup.lock().unwrap();

        let maslow_node_lookup = world
            .write_resource::<Arc<Mutex<MaslowNodeLookup>>>()
            .clone();
        let maslow_node_lookup = maslow_node_lookup.lock().unwrap();

        // Unwrap is used when getting from lookup, as it is an exception if
        // the indicated values do not exist.

        let maslow_tree = MaslowTree::new(
            &"Faction/Wander",
            &[
                maslow_node_lookup.get("Faction Reaction").unwrap().clone(),
                maslow_node_lookup.get("Random Wander").unwrap().clone(),
            ],
        );
        maslow_tree_lookup.insert(maslow_tree);
    }
}

impl Screen for MaslowInitScreen {
    fn init(&mut self) {
        self.state = match self.state {
            ScreenState::Inactive => ScreenState::Inactive,
            ScreenState::Started => ScreenState::Running,
            ScreenState::Running => ScreenState::Running,
            ScreenState::Stopped => ScreenState::Inactive,
        }
    }

    fn close(&mut self) {
        self.state = match self.state {
            ScreenState::Inactive => ScreenState::Inactive,
            ScreenState::Started => ScreenState::Inactive,
            ScreenState::Running => ScreenState::Inactive,
            ScreenState::Stopped => ScreenState::Inactive,
        }
    }

    fn blocks_draw(&self) -> bool {
        true
    }

    fn blocks_update(&self) -> bool {
        true
    }

    fn draw(&mut self, _world: &mut World) {}

    fn update(&mut self, world: &mut World, _screen_push_wrapper: &mut ScreenPushWrapper) {
        self.maslow_init_state = match self.maslow_init_state {
            MaslowInitState::World => {
                world.add_resource(Arc::new(Mutex::new(MaslowFuncWrapperLookup::new())));
                world.add_resource(Arc::new(Mutex::new(MaslowNodeLookup::new())));
                world.add_resource(Arc::new(Mutex::new(MaslowTreeLookup::new())));

                MaslowInitState::MaslowFuncWrappers
            }
            MaslowInitState::MaslowFuncWrappers => {
                world.add_resource(Arc::new(Mutex::new(MaslowFuncWrapperLookup::new())));
                world.add_resource(Arc::new(Mutex::new(MaslowNodeLookup::new())));
                world.add_resource(Arc::new(Mutex::new(MaslowTreeLookup::new())));
                self.init_maslow_func_wrappers(world);

                MaslowInitState::MaslowNodes
            }
            MaslowInitState::MaslowNodes => {
                self.init_maslow_nodes(world);

                MaslowInitState::MaslowTrees
            }
            MaslowInitState::MaslowTrees => {
                self.init_maslow_trees(world);

                self.state = ScreenState::Stopped;

                MaslowInitState::Finished
            }
            MaslowInitState::Finished => {
                // This state is a placeholder that exists due to the
                // necessity of returning something from the StartGame
                // match. As such, we should never actually reach it.
                panic!("We should have exited before getting here!");
            }
        }
    }

    fn state(&self) -> ScreenState {
        self.state
    }
}
