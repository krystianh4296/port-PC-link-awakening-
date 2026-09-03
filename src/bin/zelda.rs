use gameboy_port::game::game::Game;
use gameboy_port::input::Input;
use gameboy_port::rendering::renderer::Renderer;

fn main() {
    let mut game = Game::new();
    let mut input = Input::new();
    let mut renderer = Renderer::new();

    while game.is_running() && renderer.is_open() {
        input.update(renderer.window());
        game.update(&input);
        renderer.draw();
    }
}   