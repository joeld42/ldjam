//use std::slice::Iter;

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

impl MapDirection {
    pub fn iterator() -> impl Iterator<Item = MapDirection> {
        [ MapDirection::North, MapDirection::NorthEast, MapDirection::SouthEast,
        MapDirection::South, MapDirection::SouthWest, MapDirection::NorthWest ].iter().copied()
    }
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

#[derive(Copy, Clone, Default, Debug)]
pub struct GameSnapshot
{
    pub map : GameMap,
    pub score : [ i32; 4],
}

impl GameSnapshot {
    pub fn calc_simple_score( &self, player : i32 ) -> i32
    {
        let mut score = 0;
        for mapsq in &self.map {
            if mapsq.power > 0 && mapsq.player == (player + 1) as u8 {
                score += 1;
            }
        }
        score
    }

    pub fn update_scores( &mut self ) {
        for i in 0..4 {
            self.score[i] = self.calc_simple_score( i as i32 );
        }
    }
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

    //let row_sz = MAP_SZ as i32;
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

     pub fn neighbors( &self, ndx : i32, valid_only : bool ) -> Vec::<i32> {

        let mut result = Vec::new();


        for mapdir in MapDirection::iterator() {
            let nbr_ndx = move_dir( ndx, mapdir);
            if !valid_only || (nbr_ndx != INVALID as i32 && self.spaces[ nbr_ndx as usize].contents == MapSpaceContents::Playable) {
                result.push(nbr_ndx );
            }
        }

        result
     }

     pub fn edge_spaces( &self ) -> Vec<i32>
    {
        let mut edge_spaces = Vec::new();
        for i in 0..self.spaces.len() {
            let map_space = self.spaces[i];
            if map_space.contents == MapSpaceContents::Playable {
                for nbr in self.neighbors( i as i32, false ) {
                    // If this is on the edge of the map, add it to the edge_space set
                    if (nbr == INVALID as i32) || (self.spaces[nbr as usize].contents == MapSpaceContents::NotInMap ) {
                        edge_spaces.push( map_space.ndx );
                        break;
                    }
                }
            }
        }

        edge_spaces
    }

    pub fn edge_spaces_corners( &self ) -> Vec<i32>
    {
        let edge_spaces = self.edge_spaces();
        let mut edge_corners = Vec::new();

        // check valid neighbor count in increasing order
        for max_count in 1..6 {
            for endx in 0..edge_spaces.len() {
                let e = edge_spaces[endx];
                if self.neighbors( e, true ).len() <= max_count {
                    edge_corners.push( e );
                }
            }

            if edge_corners.len() > 0 {
                return edge_corners;
            }
        }

        return edge_corners
    }

    pub fn check_reachability( &self ) -> bool {

        let mut reachable: [bool; MAP_SZ * MAP_SZ] = [false; MAP_SZ * MAP_SZ];

        // flood fill check that map_copy is still reachable from everywhere
        for i in 0..self.spaces.len() {
            // start on any playable space
            if self.spaces[i].contents == MapSpaceContents::Playable {
                reachable[i] = true;
                break;
            }
        }

        let mut changed = true;
        while changed {

            changed = false;
            for i in 0..self.spaces.len() {
                if self.spaces[i].contents == MapSpaceContents::Playable &&
                   !reachable[i] {

                    // see if any neighbors are reachable
                    for nbr in self.neighbors( i as i32, true ) {
                        if reachable[nbr as usize] {
                            reachable[i] = true;
                            changed = true;
                            break;
                        }
                    }
                }
            }
        }

        // check reachability
        for i in 0..self.spaces.len() {
            // start on any playable space
            if !reachable[i] && self.spaces[i].contents == MapSpaceContents::Playable {
                return false;
            }
        }

        // otherwise, everything is reachable
        true

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

pub fn gen_valid_moves( gamecurr : GameSnapshot, for_player : usize ) -> Vec<GameSnapshot>
{
    let mut result = Vec::new();

    // Find all the squares we could move from
    let mut split_squares = Vec::new();
    for mapsq in &gamecurr.map {
        if (mapsq.power > 1) && (mapsq.player == (for_player + 1) as u8) {
            // This is our space, and we can potentially split here
            for mapdir in MapDirection::iterator() {
                let ndx = mapsq.ndx;
                let move_ndx = gamecurr.map.search_dir( ndx, mapdir );
                if move_ndx != ndx && move_ndx != INVALID as i32 {
                    // We can move in this direction
                    split_squares.push( (ndx, move_ndx ));
                }
            }
        }
    }

    for (start_ndx, move_ndx) in split_squares {
        // can move from start_ndx to move_ndx
        let start_ndx = start_ndx as usize;
        let move_ndx = move_ndx as usize;

        for amt in 1..gamecurr.map.spaces[start_ndx].power
        {
            let mut next : GameSnapshot = gamecurr;
            next.map.spaces[start_ndx].power -= amt;
            next.map.spaces[move_ndx].power += amt;
            next.map.spaces[move_ndx].player = (for_player + 1) as u8;

            result.push( next );
        }
    }

    result
}

pub fn evaluate_position(snap:GameSnapshot) -> [i32;4]{
    //let mut result = Vec::new();
    let mut access_map : [ i32 ; 100]=[0; 100];
    let mut eval_score:[i32;4]=[0; 4];
    for hex in &snap.map{
        if hex.power>1{
            let player=1<<(hex.player-1);
            let index=hex.ndx;
            for mapdir in MapDirection::iterator() {
                let target_index=snap.map.search_dir( index, mapdir );
                if index != target_index{
                    access_map[target_index as usize]|=player;
                }
            }
        }
    }
    for hex in &snap.map{
        if hex.power>0{
            let mut weight:i32=10000;
            if hex.power>1{
                let player=1<<(hex.player-1);
                let not_player=!player;
                let index=hex.ndx;
                let movepower:i32=((hex.power-1) as i32)*10000;
                let mut opportunity:i32=0;
                for mapdir in MapDirection::iterator() {
                    let mut curr_hex=hex.ndx;
                    let mut distancefactor:i32=10000;
                    loop{
                        curr_hex = move_dir( curr_hex, mapdir);
                        if(curr_hex as usize == INVALID) || (snap.map.spaces[curr_hex as usize].contents != MapSpaceContents::Playable) || (snap.map.spaces[curr_hex as usize].power != 0){
                            break;
                        }
                        if (access_map[curr_hex as usize] & not_player)==0{
                            distancefactor*=9;
                        }
                        else{
                            distancefactor*=5;
                        }
                        distancefactor/=10;
                        opportunity+=distancefactor;
                    }
                }
                if opportunity>0{
                    weight+=1000000000/((1000000000/movepower)+(1000000000/opportunity));
                }
            }
            eval_score[(hex.player-1) as usize]+=weight;
        }
    }
    eval_score
}
