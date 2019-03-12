extern crate rand;
use rand::thread_rng;
use rand::rngs::ThreadRng;
use super::io::Window;

pub struct GameState<'a>
{
    alive: bool,
    window: &'a mut Window
}

impl<'a> GameState<'a>
{
    pub fn new(window: &'a mut Window) -> Self
    {
        Self {
            alive: true,
            window: window,
        }
    }

    pub fn alive(&self) -> bool
    {
        self.alive
    }
    
    pub fn kill(&mut self)
    {
        self.alive = false;
    }

    pub fn rng(&mut self) -> ThreadRng
    {
        thread_rng()
    }

    pub fn window(&self) -> & Window
    {
        self.window
    }

    pub fn window_mut(&mut self) -> &mut Window
    {
        return self.window
    }
}
