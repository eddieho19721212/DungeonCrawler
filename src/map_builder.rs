/*
    ver:0.2.0
    Subject:Building a Dungeon
 */
use crate::prelude::*;

/*
    ver:0.2.0
    maximum number of rooms
 */
const NUM_ROOMS: usize = 20;

/*
    ver:0.2.0
    Define MapBuilder struct : The struct contain its own Map for which it will work on its copy
                               and then pass the result to the game.
                               The rooms vector is a list of the rooms that will be added to
                               the map.
                               Each room represented with the Rect struct from Bracket-lib.
    Rect type(bracket_geometry::rect) which is a helper for calculations involving rectangles.
    Point type(bracket_geometry::point)

 */
pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

/*
    ver:0.2.1
    The previous example started with a dungeon entirely comprised of floors. A
    room-carving algorithm works the other way around, starting with a solid
    block of rock and carving rooms and corridors from the stone.
 */
impl MapBuilder {
    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }
}