use bevy::prelude::*;
use rand::*;

#[derive(Resource, Clone)]
pub struct ComplexLayout {
    pub rooms: Vec<ComplexRoom>,
}

impl ComplexLayout {
    pub fn new() -> Self {
        Self {
            rooms: vec![],
        }
    }
    pub fn get_populated_chunks(&self) -> Vec<IVec2> {
        self.rooms.iter().flat_map(|room| room.chunks.clone()).collect()
    }
    pub fn chunk_to_room(&self, chunk: IVec2) -> Option<&ComplexRoom> {
        self.rooms.iter().find(|room| room.chunks.contains(&chunk))
    }
    pub fn insert_door_by_chunks(&mut self, chunk_a: IVec2, chunk_b: IVec2) {
        let room_a = self.chunk_to_room(chunk_a).unwrap().clone(); 
        let room_b = self.chunk_to_room(chunk_b).unwrap().clone(); 
        if room_a == room_b {
            return;
        }
        self.rooms.iter_mut().for_each(|room| {
            if room.chunks == room_a.chunks {
                room.add_door(ComplexRoomDoor {
                    from: chunk_a,
                    to: chunk_b,
                });
            }
            if room.chunks == room_b.chunks {
                room.add_door(ComplexRoomDoor {
                    from: chunk_b,
                    to: chunk_a,
                });
            }
        });
    }
    pub fn insert_door(&mut self, room_a: ComplexRoom, room_b: ComplexRoom) {
        // find list of chunks that are directly next to each other in the two rooms and pick two to doorify
        let mut rng = rand::thread_rng();
        let mut possible_doors = vec![];
        for chunk_a in room_a.chunks.iter() {
            for chunk_b in room_b.chunks.iter() {
                if chunk_a.as_vec2().distance(chunk_b.as_vec2()) == 1.0 {
                    possible_doors.push((*chunk_a, *chunk_b));
                }
            }
        }
        if possible_doors.len() == 0 {
            panic!("No possible doors found between rooms");
        }
        let choice = possible_doors[rng.gen_range(0..possible_doors.len())];
        self.insert_door_by_chunks(choice.0, choice.1);
    }
    
}

pub fn get_surrounding_chunks(chunk: IVec2) -> Vec<IVec2> {
    vec![
        IVec2::new(chunk.x, chunk.y + 1),
        IVec2::new(chunk.x, chunk.y - 1),
        IVec2::new(chunk.x + 1, chunk.y),
        IVec2::new(chunk.x - 1, chunk.y),
    ]
}

//*****************************************   Complex Room Def   ***********************************************************//


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ComplexRoom {
    pub chunks: Vec<IVec2>,
    pub doors: Vec<ComplexRoomDoor>,
    pub room_type: ComplexRoomType,
}

impl ComplexRoom {
    pub fn new(chunks: Vec<IVec2>) -> Self {
        Self {
            chunks,
            doors: vec![],
            room_type: ComplexRoomType::Enemy,
        }
    }
    pub fn add_door(&mut self, door: ComplexRoomDoor) {
        if self.doors.contains(&door) {
            return;
        }
        self.doors.push(door);
    }
    pub fn all_neighboring_chunks(&self) -> Vec<IVec2> {
        let mut chunk_list: Vec<IVec2> = vec![];

        for chunk in self.chunks.iter() {
            let north = IVec2::new(chunk.x, chunk.y + 1);
            let south = IVec2::new(chunk.x, chunk.y - 1);
            let east = IVec2::new(chunk.x + 1, chunk.y);
            let west = IVec2::new(chunk.x - 1, chunk.y);

            if !self.chunks.contains(&north) && !chunk_list.contains(&north) {
                chunk_list.push(north);
            }
            if !self.chunks.contains(&south) && !chunk_list.contains(&south) {
                chunk_list.push(south);
            }
            if !self.chunks.contains(&east) && !chunk_list.contains(&east) {
                chunk_list.push(east);
            }
            if !self.chunks.contains(&west) && !chunk_list.contains(&west) {
                chunk_list.push(west);
            }
        }

        return chunk_list;
    }
    pub fn is_adjacent(&self, other: &ComplexRoom) -> bool {
        for chunk in self.chunks.iter() {
            for other_chunk in other.chunks.iter() {
                if chunk.as_vec2().distance(other_chunk.as_vec2()) == 1.0 {
                    return true;
                }
            }
        }
        return false;
    }
    pub fn get_permutation(&self) -> ComplexRoomPermutation {
        let min_x = self.chunks.iter().map(|chunk| chunk.x).min().unwrap();
        let max_x = self.chunks.iter().map(|chunk| chunk.x).max().unwrap();
        let min_y = self.chunks.iter().map(|chunk| chunk.y).min().unwrap();
        let max_y = self.chunks.iter().map(|chunk| chunk.y).max().unwrap();
        let width = max_x - min_x + 1;
        let height = max_y - min_y + 1;
        match (width, height) {
            (1, 1) => ComplexRoomPermutation::OneByOne,
            (1, 2) => ComplexRoomPermutation::OneByTwo,
            (2, 1) => ComplexRoomPermutation::TwoByOne,
            (1, 3) => ComplexRoomPermutation::OneByThree,
            (3, 1) => ComplexRoomPermutation::ThreeByOne,
            (2, 2) => {
                let root = IVec2::new(min_x, min_y); // bottom left

                if !self.chunks.contains(&root) { //missing bottom left
                    ComplexRoomPermutation::LShapeQ3
                } else if !self.chunks.contains(&IVec2::new(min_x + 1, min_y)) { // missing bottom right
                    ComplexRoomPermutation::LShapeQ4
                } else if !self.chunks.contains(&IVec2::new(min_x, min_y + 1)) { // missing top left
                    ComplexRoomPermutation::LShapeQ2
                } else { // missing top right
                    ComplexRoomPermutation::LShapeQ1
                }
            }
            _ => panic!("Invalid room size, see get_permutation()"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComplexRoomDoor {
    pub from: IVec2,
    pub to: IVec2,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ComplexRoomType {
    Spawn,
    Enemy,
    Treasure,
    Shop,
    Boss,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
/// X by Y
pub enum ComplexRoomPermutation {
    OneByOne, // ▖
    OneByTwo, // :
    TwoByOne, // ▖▖
    OneByThree, // ┆
    ThreeByOne, // ▖▖▖
    LShapeQ1, // ▙
    LShapeQ2, // ▟
    LShapeQ3, // ▜
    LShapeQ4, // ▛
}