/*
    ver:0.1.1
    Subject:Represent Your Tiles
    The map module import main module prelude
    The crate:: reference to main module
 */
use crate::prelude::*;

/*
    ver:0.1.1
    Subject:Represent Your Tiles
    usize is a special case, it uses the preferred bit size for your CPU.
    If er have a 64-bit computer, usize will be 64 bits.
    Rust commonly uses usize to index collections & arrays
    as keyword is type convertor.
 */
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

/*
    ver:0.1.1
    Subject:Represent Your Tiles
    Definition of Tiles:
    Most 2D game include map, typically an array of tiles.
    Here, the map structure represents the dungeon layout.
    Each tile has a type describing how that tile is rendered and what happens if we try to enter it.
    We will represent our map as a vector.Each entry in the vector represents on tile,
    so for 80x50 map we will have 4000 tiles.
    Tiles represent portions of the map, and the same tile graphic is re-used to
    represent tiles of a particular type. Entity such as the player or any monster are
    overlaid on top.

    Type of Tile:
    Rust's derive list:
    1 ) Clone -- Add a clone() function to the type.
                 Calling mytile.clone() makes a deep copy of the variable without affecting the original.
                 If we clone a dtruct, everything the struct contains will also be cloned.
                 This is useful when we want to safely work with a clone of some data with no risk
                 of altering the original or when we need to work around the borrow checker.
   2 ) Copy --   Changes the default action when assigning a TileType from one variable to another.
                 Instead of moving the value, it takes a copy.Smaller types are often faster
                 when we copy them around.
   3 ) PartialEq -- Adds code that allows us to compare TileType values with the == operator.
 */
#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall, //#
    Floor,
}

/*
    ver:0.1.1
    Subject:Represent Your Tiles
    Create a map struct
 */
pub struct Map {
    pub tiles: Vec<TileType>,
}


impl Map {
    /*
        ver:0.1.1
        Subject:Represent Your Tiles
        Map struct's constructor - creating a map consisting entirely of floors
    */
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    /*
        ver:0.1.1
        Subject:Represent Your Tiles
        Render The Map:
        1 ) The Map needs to be able to draw itself to the screen
        2 ) Iterating y first is faster with row-first striding due to memory cache usage.
        3 ) Using match to determine tile type and call Bracket-lib drawing function' set to
            render each map tile.
        4 ) Floor appear as a . in yellow
        5 ) Wall as # in green
    */
    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y);
                match self.tiles[idx] {
                    TileType::Floor => {
                        ctx.set(x, y, YELLOW, BLACK, to_cp437('.')); //Floor is '.'
                    },
                    TileType::Wall => {
                        ctx.set(x, y, GREEN, BLACK, to_cp437('#'));//Wall is #
                    },
                }
            }
        }
    }
    /*
        ver: 0.1.2
        Subject: Adding the Adventurer
                 We need to determine if an x/y coordinate pair is within the bounds of the map.
                 If you don’t perform bounds-checking, the player can move off the edge of the map,
                 and either wrap around or crash the program.

        The function checks that the location specified in point is greater than zero
        on both the x and y axes, and that it is less than the screen height and width.
     */
    pub fn in_bound(&self, point: Point) -> bool {
        (point.x >= 0 && point.x < SCREEN_WIDTH) && (point.y >= 0 && point.y < SCREEN_HEIGHT)
    }

    /*
        ver: 0.1.2
        Subject: Adding the Adventurer
        1 ) Ensure Player walk on floors but not through walls!
        2 ) Uses in_bounds to check that the destination is valid, and also checks that
            the destination tile is a floor. If both are true, the adventurer may enter the tile.
     */
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bound(point) && self.tiles[map_idx(point.x, point.y)]==TileType::Floor
    }

    /*
         ver: 0.1.2
     */
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bound(point) {
            None
        }else {
            Some(map_idx(point.x, point.y))
        }
    }
}




/*
    ver:0.1.1
    Subject:Represent Your Tiles
    Index of Map: Vector are 1D dimension, so we need a way to transform map locations(x, y)
                  into vector indices.
                  This transformation is known as striding.
                  We use row-first encoding.
    let index = (y * WIDTH) + x;
    let x = index % WIDTH;
    let y = index / WIDTH;
    % is modulus dividing integers always rounds down.
 */
pub fn map_idx(x: i32, y: i32) -> usize {
   /*
        Note that: x & y are 32=bit integers, but vector are indexed by a variable of type usize.\
                   Adding as usize to a variable reference converts the result to a usize.
    */
    ((y * SCREEN_WIDTH) + x) as usize
}


