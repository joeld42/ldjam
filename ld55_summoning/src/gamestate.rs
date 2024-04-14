
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MapSpaceContents {
    NotInMap,  // Not part of the board at all
    Blocked,   // A square but blocked by decoration
    Playable,  // A square that can be played on
}

#[derive(Copy, Clone, Debug)]
pub enum MapDirection {
    North,
    NorthEast,
    NorthWest,
    South,
    SouthWest,
    SouthEast
}

impl Default for MapSpaceContents {
    fn default() -> Self {
        MapSpaceContents::NotInMap
    }
}


#[derive(Copy, Clone, Default, Debug)]
pub struct MapSpace {
    pub contents : MapSpaceContents,
    pub player: u8,
    pub power: u8,
    pub ndx: i32,
}

#[derive(Copy, Clone, Debug)]
pub struct GameMap {
    pub spaces : [ MapSpace ; 100],
}

pub const MAP_SZ : usize = 10;
pub const INVALID : usize = 9999;

// Note the index might be out of range
pub fn map_index( row : i32, col : i32 ) -> i32
{
    if (row < 0) || (col < 0) || ( row >= MAP_SZ as i32) || (col >= MAP_SZ as i32) {
        INVALID as i32
    } else {
        (row * (MAP_SZ as i32)) + col
    }    
}

pub fn move_dir( ndx : i32, dir : MapDirection ) -> i32
{
    let row : i32 = ndx / (MAP_SZ as i32);
    let col : i32 = ndx % (MAP_SZ as i32);

    let row_sz = MAP_SZ as i32;
    if col % 2 == 1 {
        // Odd Col
        match dir {
            MapDirection::North => map_index( row + 1, col ),
            MapDirection::NorthEast => map_index( row, col + 1),
            MapDirection::NorthWest => map_index( row, col - 1 ),
            MapDirection::South => map_index( row - 1, col ),
            MapDirection::SouthWest => map_index( row - 1, col - 1 ),
            MapDirection::SouthEast => map_index( row - 1, col + 1 ),
        }
        
    } else {
        // Even Col
        match dir {
            MapDirection::North => map_index( row + 1, col ),
            MapDirection::NorthEast => map_index( row + 1, col + 1),
            MapDirection::NorthWest => map_index( row + 1, col - 1 ),
            MapDirection::South => map_index( row - 1, col ),
            MapDirection::SouthWest => map_index( row, col - 1 ),
            MapDirection::SouthEast => map_index( row, col + 1 ),
        }
    }
}

impl GameMap {
     pub fn search_dir( &self, ndx : i32, dir : MapDirection ) -> i32 {
        
        let mut curr = ndx;        
        loop {
            let last = curr;
            curr = move_dir( curr, dir);
            if  (curr == INVALID as i32) || 
                ( self.spaces[curr as usize].contents != MapSpaceContents::Playable ) ||
                ( self.spaces[curr as usize].power != 0 ) {
                    // if this space is filled or blocked
                    return last;
                 }
        }        
     }
}

impl Default for GameMap {
    fn default() -> GameMap {
        GameMap {
            spaces : [ MapSpace::default(); MAP_SZ*MAP_SZ ],
        }        
    }
}


impl<'a> IntoIterator for &'a GameMap {
    type Item = &'a MapSpace;
    type IntoIter = std::slice::Iter<'a, MapSpace>;

    fn into_iter(self) -> Self::IntoIter {
        self.spaces.iter()
    }
}

impl<'a> IntoIterator for &'a mut GameMap {
    type Item = &'a mut MapSpace;
    type IntoIter = std::slice::IterMut<'a, MapSpace>;

    fn into_iter(self) -> Self::IntoIter {
        self.spaces.iter_mut()
    }
}