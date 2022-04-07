use bracket_lib::prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls(); // clears the window
        ctx.print(1, 1, "Hello, Bracket Terminal!"); // prints text to the game window
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Flappy Dragon").build()?;
    main_loop(context, State{})
}
