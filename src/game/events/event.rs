use crate::game::GameState;

pub trait Event {
    fn execute(&self, game_state: GameState) -> GameState;
}