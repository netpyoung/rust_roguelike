use actor::Actor;
use rendering::render::RenderingComponentAble;
use rendering::window::Windows;
use rendering::window::WindowComponent;
use game::Game;

pub trait GameState {
    fn new() -> Self where Self: Sized;

    fn update(&mut self, npcs: &mut Vec<Box<Actor>>, character: &mut Actor);
    fn render(&mut self, renderer: &mut RenderingComponentAble, npcs: &Vec<Box<Actor>>, character: &Actor, windows: &mut Windows);

    fn enter(&self, &mut WindowComponent);
    fn exit(&self);
    fn should_update_state(&self) -> bool;
}


pub struct MovementGameState;
impl GameState for MovementGameState {
    fn new() -> MovementGameState {
        MovementGameState
    }

    fn update(&mut self, npcs: &mut Vec<Box<Actor>>, character: &mut Actor) {
        character.update();
        Game::set_character_point(character.position);
        for npc in npcs.iter_mut() {
            npc.update();
        }
    }

    fn render(&mut self, renderer: &mut RenderingComponentAble, npcs: &Vec<Box<Actor>>, character: &Actor, windows: &mut Windows) {
        renderer.before_render_new_frame();
        for w in windows.all_windows() {
            renderer.attach_window(w);
        }
        for npc in npcs.iter() {
            npc.render(renderer);
        }
        character.render(renderer);
        renderer.after_render_new_frame();
    }


    fn enter(&self, win: &mut WindowComponent) {
    }
    fn exit(&self) {
    }

    fn should_update_state(&self) -> bool {
        true
    }

}
