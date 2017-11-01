extern crate piston_window;
use piston_window::*;

pub mod snake;
use snake::*;

fn main() {

    let mut window: PistonWindow = 
    WindowSettings::new("Snake!", [WINDOW_SIZE_X as u32, WINDOW_SIZE_Y as u32])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let mut game = Game::new();
    while let Some(e) = window.next() {
        match e {
            Event::Loop(loo) => handle_loop(loo, e, &mut game, &mut window),
            Event::Input(inp) => {
                if let Input::Button(button_args) = inp{
                    if let ButtonState::Press = button_args.state {
                        game.on_input(button_args.button)
                    }
                }
            }
            _ => {}
        }
    }
}

fn handle_loop(loo : Loop, e : Event, game : &mut Game, window : &mut PistonWindow ) {
    match loo {
        Loop::Render(render_args) => game.on_draw(render_args,window,&e),
        Loop::Update(update_args) => game.on_update(update_args),
        _ => {}
    }
}
