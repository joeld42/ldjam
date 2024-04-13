
#[derive(Copy, Clone, Debug)]
pub enum MapSpaceContents {
    NotInMap,  // Not part of the board at all
    Blocked,   // A square but blocked by decoration
    Playable,  // A square that can be played on
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
    spaces : [ MapSpace ; 100],
}

pub const MAP_SZ : usize = 10;

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