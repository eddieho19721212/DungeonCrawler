/*
    ver:0.1.3
    Subject:Create the Player Structure
 */
use crate::prelude::*;

/*
    ver:0.1.3
    The player struct
 */
pub struct Player {
    /*
        ver:0.1.3
        Rather than storing x and y separately, you can use the Point type exported
        from bracket-lib. Point is an x and y value, but with additional functionality provides
        basic vector geometry functionality.
     */
    pub position: Point,
}

impl Player {
    /*
        ver:0.1.3
        Player's constructor
     */
    pub fn new(position: Point) -> Self {
        Self {
            position
        }
    }

    /*
        ver:0.1.3
        Just like the map, Player should have a render function.
        It calculates the screen position of the player and uses set to draw the
        @ symbol—representing the adventurer—at that screen location.
     */
    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(self.position.x, self.position.y, WHITE, BLACK, to_cp437('@'));//Player render as @
    }

    /*
        ver:0.1.3
        1 ) The player needs to be able to walk around the map in response to keyboard commands
            from the user.
        2 ) The function creates a variable named delta to store the intended change in player position.
            A delta of zero indicates that no movement was requested.
        3 ) The second half of the function calculates the player’s new position: the current
            position plus the delta. It then calls can_enter_tile from the map—and updates the
            position if the move is valid.
     */
    pub fn update(&mut self, ctx: &mut BTerm, map: &Map) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Down => Point::new(0, 1),
                _ => Point::zero()
            };

            let new_pos = self.position + delta;
            if map.can_enter_tile(new_pos) {
                self.position = new_pos;
            }
        }

    }
}