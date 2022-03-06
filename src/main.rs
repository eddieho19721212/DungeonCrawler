/*
    Import  the map module into global scope and set up the map:: prefix.
    Within main.rs, items declared in map may be accessed as map::my_function.
 */
#![allow(dead_code)]
#![allow(unused_variables)]
mod map;//ver:0.1.1
mod player;//ver:0.1.3
mod map_builder;//ver:0.2.0

/*
    ver:0.1.1
    Subject:Represent Your Tiles
    Create our own prelude as module:
    1 ) The mod keyword can also be used to declare a new module in our source code.
        Because this is the top level of our crate, we don't need to make it public,
        modules branching from crate are visible throughout our program.
 */
mod prelude {
    /*
        Publicly using the bracket-lib prelude re-export it inside our prelude.
        Anything that uses our prelude also use bracket-lib
     */
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    /*
        We imported map into our main scope. Our module can reference the main scope as crate::.
        Re-export the map as a public available from within our prelude.
     */
    pub use crate::map::*;
    pub use crate::player::*;//ver: 0.1.3
    pub use crate::map_builder::*;//ver:0.2.0
}

//Finally, use our prelude module to make it available to the main scope in main.rs
use prelude::*;

/*
    ver:0.1.1
    Subject:Represent Your Tiles
    Map is data for game so that we need defined it in State and initialize Map with State's
    constructor
 */
struct State {
    map: Map,
    player: Player,//ver: 0.1.3
}

/*
    ver:0.1.1
    Subject:Represent Your Tiles
    State' constructor
 */
impl State{
    fn new() -> Self {
        Self {
            map:Map::new(),
            player: Player::new(Point::new(SCREEN_WIDTH/2, SCREEN_HEIGHT/2)), //ver:0.1.3
        }
    }
}

/*
    ver:0.1.1
    Subject:Represent Your Tiles
    Implement the trait GameState defined in Bracket-lib for our State struct
    The trait require us implement the tick function to access and change our State instance.
    Input Params : 1 ) &mut, self allows the tick function to access and change your State instance.
                   2 ) ctx,  provides a window into the currently running bracket-terminal— accessing
                       information like mouse position and keyboard input, and sending commands to
                       draw to the window.
 */

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm){
        //Clean the screen
        ctx.cls();

        /*
            call a Map's render function to draw itself
         */
        self.map.render(ctx);

        /*  This is just a dummy function for first version:
            The first two params(1,1) is the screen-space coordinates, representing where you want
            the text to appear.
            Bracket-lib defines 0,0 as the top-left of the window. In an 80x50 window,
            79x49 is the bottom right.

        ctx.print(1, 1, "Hello, Bracket Terminal!");
         */

        /*
            ver:0.1.3
            Code for draw Player
         */
        self.player.update(ctx, &self.map);
        self.player.render(ctx);
    }
}

/*
    Bracket-lib provide a Result type named BError. Making oour function return a BError allows
    us to take advantage of the ? operator.
 */
fn main()->BError {
    /*
      ver:0.1.1
      Subject:Represent Your Tiles
       1 ) Bracket-lib using builder pattern with an initial constructor function that return the builder.
       2 ) Successive calls to the builder object add information to the build request.
       3 ) Finally, we finished describing the object we want to create, call a build() function.
           This return the completed object or an error containing our desired system.
       4 ) fps_cap automatically tracks our game speed, and tells our OS that it can rest in between frames.
     */
    let context = BTermBuilder::simple80x50()
                            .with_title("First dungeon crawl")
                            .with_fps_cap(30.0)
                            .build()?;
    /*
        ver:0.1.1
        Subject:Represent Your Tiles
        After created a terminal instance, we need to tell bracket-lib to start executing rhe game loop,
        and link the engine with our State so that bracket-lib knows where the tick function is located.
     */
    main_loop(context, State::new())
}
