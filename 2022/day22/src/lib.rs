#[macro_use]
extern crate num_derive;

use num_traits::FromPrimitive;

use std::fmt::Display;
use std::collections::BTreeSet;
use std::collections::BTreeMap;
use core::fmt;
use shared::Dir;
use colored::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct RowColInfo {
    pub valid_length: usize,
    pub start_index: usize, // First nonvoid
    pub end_index: usize,  // Last nonvoid
    pub last_wall_index: Option<usize>,
    pub first_wall_index: Option<usize>,
}

impl RowColInfo {
    pub fn new(v: &[Object]) -> Self {
        let first_nonvoid = v.iter().position(|o| *o != Object::Void).unwrap();
        let last_nonvoid = v.iter().rposition(|o| *o != Object::Void).unwrap();


        let v = v.iter().filter(|o| **o != Object::Void).collect::<Vec<&Object>>();

        let first_wall = v.iter().position(|o| **o == Object::Wall);
        let last_wall  = v.iter().rposition(|o| **o == Object::Wall);

        let valid_length = 1 + last_nonvoid - first_nonvoid;
        Self {
            valid_length,
            start_index: first_nonvoid,
            end_index: last_nonvoid,
            first_wall_index: first_wall,
            last_wall_index: last_wall,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum Face {
    Top,
    Bottom,
    Front,
    Back,
    Left, Right
}


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Axis {
    PosX,
    PosY,
    PosZ,
    NegX,
    NegY,
    NegZ,
}

#[derive(Debug, Clone)]
pub struct FaceInfo {
    start_col: usize,
    start_row: usize,
    pub dir_col: Axis,
    pub dir_row: Axis,
}

#[derive(Debug, Clone)]
pub struct Cube {
    pub side_length: usize,
    pub walls: BTreeSet<(i64, i64, i64, Face)>, // list of 3d coordinates
    pub face_info: BTreeMap<Face, FaceInfo>,
}


impl Cube {
    // This logic only works for the specific cube nets in the input and example
    // Of the net is organize in some other this logic will panic with assertion errors
    //
    // Example data is organized as
    // .  .  To
    // Ba Le Fr
    // .  .  Bo Ri
    // 
    // My input is organized as
    // .  To Ri
    // .  Fr .
    // Le Bo .
    // Ba .  .
    //
    // Top, front and right are organized the same
    pub fn new(map: &Map, side_length: usize) -> Self {

        let start_col = map.start.0 as usize;
        let mut walls = BTreeSet::new();
        let mut face_info = BTreeMap::new();


        // ################################
        // Go through first face, this will be the top face
        let top_face = map.parse_face(start_col, 0, side_length);
        let z = side_length as i64 + 1;
        for (iy, v) in top_face.iter().enumerate() {
            // At iy == 0, y coordinate should be side_length
            // At the final step (iy == side_length - 1), y coordinate should be 1
            let y = (side_length - iy) as i64;
            for (ix, o) in v.iter().enumerate() {
                if *o == Object::Wall {
                    let x = ix as i64 + 1;
                    walls.insert((x,y,z, Face::Top)); 
                }
            }
        }

        face_info.insert(Face::Top, FaceInfo {
            start_col,
            start_row: 0,
            dir_col: Axis::PosX,
            dir_row: Axis::NegY, // Larger row number corresponds to lower y
        });


        // #####################
        // Front face is below top face
        let front_face = map.parse_face(start_col, side_length, side_length);
        assert_eq!(front_face.len(), side_length);
        assert_eq!(front_face[0].len(), side_length);

        let y = 0;
        for (iz, v) in front_face.iter().enumerate() {
            let z = (side_length - iz) as i64;
            for (ix, o) in v.iter().enumerate() {
                if *o == Object::Wall {
                    let x = ix as i64 + 1;
                    walls.insert((x,y,z, Face::Front));
                }
            }
        }

        face_info.insert(Face::Front, FaceInfo {
            start_col,
            start_row: side_length,
            dir_row: Axis::NegZ,
            dir_col: Axis::PosX,
        });

        // ######################
        // Bottom face is below front face
        let bottom_face = map.parse_face(start_col, 2 * side_length, side_length);
        assert_eq!(bottom_face.len(), side_length);
        assert_eq!(bottom_face[0].len(), side_length);

        let z = 0;
        for (iy, v) in bottom_face.iter().enumerate() {
            let y = iy as i64 + 1; // At iy == 0, y coordinate should be 1
            for (ix, v) in v.iter().enumerate() {
                if *v == Object::Wall {
                    let x = ix as i64 + 1;
                    walls.insert((x,y,z, Face::Bottom));
                }
            }
        }
        face_info.insert(Face::Bottom, FaceInfo {
            start_col,
            start_row: 2 * side_length,
            dir_row: Axis::PosY,
            dir_col: Axis::PosX,
        });

        // #################################
        // Try to get right face, right of front face, this is the case in the input
        let right_face = map.parse_face(start_col + side_length, 0, side_length);

        if !right_face[0].is_empty() {
            // In the input we should now have the right face
            assert_eq!(right_face.len(), side_length);
            assert_eq!(right_face[0].len(), side_length);

            let x = side_length as i64 + 1;
            for (iy, v) in right_face.iter().enumerate() {
                let y = (side_length - iy) as i64;
                for (iz, v) in v.iter().enumerate() {
                    if *v == Object::Wall {
                        let z = (side_length - iz) as i64;
                        walls.insert((x,y,z, Face::Right));
                    }
                }
            }

            face_info.insert(Face::Right, FaceInfo {
                start_col: start_col + side_length,
                start_row: 0,
                dir_row: Axis::NegY,
                dir_col: Axis::NegZ,
            });

            // #################################
            // Left face is left from bottom face
            let left_face = map.parse_face(start_col - side_length, 2 * side_length, side_length);

            assert_eq!(left_face.len(), side_length);
            assert_eq!(left_face[0].len(), side_length);

            dbg!(&left_face);

            let x = 0;
            for (iy, v) in left_face.iter().enumerate() {
                let y = iy as i64 + 1;
                for (iz, v) in v.iter().enumerate() {
                    if *v == Object::Wall {
                        let z = (side_length - iz) as i64;
                        walls.insert((x,y,z, Face::Left));
                    }
                }
            }

            face_info.insert(Face::Left, FaceInfo {
                start_col: start_col - side_length,
                start_row: 2 * side_length,
                dir_row: Axis::PosY,
                dir_col: Axis::NegZ,
            });

            // #################################
            // Back face is below left face
            let back_face = map.parse_face(start_col - side_length, 3 * side_length, side_length);
            assert_eq!(back_face.len(), side_length);
            assert_eq!(back_face[0].len(), side_length);
            let y = (side_length as i64) + 1;
            for (ix, v) in back_face.iter().enumerate() {
                let x = ix as i64 + 1;
                for (iz, v) in v.iter().enumerate() {
                    if *v == Object::Wall {
                        let z = (side_length - iz) as i64;
                        walls.insert((x,y,z, Face::Back));
                    }
                }
            }


            face_info.insert(Face::Back, FaceInfo {
                start_col: start_col - side_length,
                start_row: 3 * side_length,
                dir_row: Axis::PosX,
                dir_col: Axis::NegZ,
            });

        } else {
            // #########################
            // In the example case, right face is right from the bottom face (x = side_length +1)
            let right_face = map.parse_face(start_col + side_length, 2 * side_length, side_length);
            assert_eq!(right_face.len(), side_length);
            assert_eq!(right_face[0].len(), side_length);
            let x = side_length as i64 + 1;
            for (iy, v) in right_face.iter().enumerate() {
                let y = iy as i64 + 1;
                for (iz, v) in v.iter().enumerate() {
                    if *v == Object::Wall {
                        let z = iz as i64 + 1;
                        walls.insert((x,y,z, Face::Right));
                    }
                }
            }

            face_info.insert(Face::Right, FaceInfo {
                start_col: start_col + side_length,
                start_row: 2 * side_length,
                dir_row: Axis::PosY,
                dir_col: Axis::PosZ,
            });


            // #########################
            // Left face is left of front face
            let left_face = map.parse_face(start_col - side_length, side_length, side_length);
            assert_eq!(left_face.len(), side_length);
            assert_eq!(left_face[0].len(), side_length);
            let x = 0;
            for (iz, v) in left_face.iter().enumerate() {
                let z = (side_length - iz) as i64;
                for (iy, v) in v.iter().enumerate() {
                    if *v == Object::Wall {
                        let y = (side_length - iy) as i64;
                        walls.insert((x,y,z, Face::Left));
                    }
                }
            }
            face_info.insert(Face::Left, FaceInfo {
                start_col: start_col - side_length,
                start_row: side_length,
                dir_row: Axis::NegZ,
                dir_col: Axis::NegY,
            });

            // #########################
            // Back face is left of 'left' face
            let back_face = map.parse_face(start_col - 2 * side_length, side_length, side_length);
            assert_eq!(back_face.len(), side_length);
            assert_eq!(back_face[0].len(), side_length);
            let y = side_length as i64 + 1;
            for (iz, v) in back_face.iter().enumerate() {
                let z = (side_length - iz) as i64;
                for (ix, v) in v.iter().enumerate() {
                    if *v == Object::Wall {
                        let x = (side_length - ix) as i64;
                        walls.insert((x,y,z, Face::Back));
                    }
                }
            }
            face_info.insert(Face::Back, FaceInfo {
                start_col: start_col - 2 * side_length,
                start_row: side_length,
                dir_row: Axis::NegZ,
                dir_col: Axis::NegX,
            });
        }
        assert_eq!(face_info.len(), 6);

        Cube { side_length,
            walls,
            face_info,
            }
    }

    fn get_face(&self, state: &State3D) -> Face {
        let max_coord = self.side_length as i64 + 1;
        if state.z == max_coord {
            Face::Top
        } else if state.z == 0 {
            Face::Bottom
        } else if state.y == 0 {
            Face::Front
        } else if state.y == max_coord {
            Face::Back
        } else if state.x == 0 {
            Face::Left
        } else if state.x == max_coord {
            Face::Right
        } else {
            panic!("State {:?} is not on any face", state);
        }
    }


    pub fn get_password(&self, state: &State3D) ->  i64 {
        let state2d = self.convert_state(state);
        get_password(state2d)
    }

    fn convert_state(&self, state: &State3D) -> State {
        // Converts 3D state to 2D state that corresponds to original cube net
        let face = self.get_face(state);
        assert_eq!(face, state.face);
        let face_info = self.face_info.get(&face).unwrap();

        let row = face_info.start_row + match face_info.dir_row {
            Axis::NegX => self.side_length - state.x as usize,
            Axis::PosX => state.x as usize - 1,
            Axis::NegY => self.side_length - state.y as usize,
            Axis::PosY => state.y as usize - 1,
            Axis::NegZ => self.side_length - state.z as usize,
            Axis::PosZ => state.z as usize - 1,
        };

        let col = face_info.start_col + match face_info.dir_col {
            Axis::NegX => self.side_length - state.x as usize,
            Axis::PosX => state.x as usize - 1,
            Axis::NegY => self.side_length - state.y as usize,
            Axis::PosY => state.y as usize - 1,
            Axis::NegZ => self.side_length - state.z as usize,
            Axis::PosZ => state.z as usize - 1,
        };

        // Convert direction
        let dir_v = state.direction.get_dir();

        let dir_2d_x = match face_info.dir_col {
            Axis::NegX => -dir_v.0,
            Axis::PosX =>  dir_v.0,
            Axis::NegY => -dir_v.1,
            Axis::PosY =>  dir_v.1,
            Axis::NegZ => -dir_v.2,
            Axis::PosZ =>  dir_v.2,
        };

        let direction = if dir_2d_x == -1 {
            Dir::W
        } else if dir_2d_x == 1 {
            Dir::E
        } else {
            let dir_2d_y = match face_info.dir_row {
                Axis::NegX => -dir_v.0,
                Axis::PosX =>  dir_v.0,
                Axis::NegY => -dir_v.1,
                Axis::PosY =>  dir_v.1,
                Axis::NegZ => -dir_v.2,
                Axis::PosZ =>  dir_v.2,
            };
            if dir_2d_y == 1 {
                Dir::S
            } else if dir_2d_y == -1 {
                Dir::N
            } else {
                panic!("Could not convert direction");
            }
        };

        State {
            x: col as i64,
            y: row as i64,
            direction
        }
    }
}


pub fn get_password(state: State) -> i64 {
    println!("Row: {}", state.y + 1);
    println!("Column: {}", state.x + 1);
    println!("Direction: {}", state.direction);
    1000 * (state.y + 1) + 4 * (state.x + 1) + match state.direction {
        Dir::E => 0,
        Dir::S => 1,
        Dir::W => 2,
        Dir::N => 3,
    }
}



#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Object {
    Wall,
    Empty,
    Void,
    Loc,
    Visited(Dir),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Order {
    Move(i64),
    TurnLeft,
    TurnRight,
    End,
}

impl Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Order::Move(n) => write!(f, "Move {}", n),
            Order::TurnLeft => write!(f, "Turn Left"),
            Order::TurnRight => write!(f, "Turn Right"),
            Order::End => write!(f, "End"),
        }
    }
}

impl Object {
    pub fn new(c: char) -> Self {
        match c {
            '.' => Object::Empty,
            '#' => Object::Wall,
            ' ' => Object::Void,
            _ => panic!("Unknown character '{c}'"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    pub grid: Vec<Vec<Object>>,
    pub start: (i64, i64),
    pub rows: Vec<RowColInfo>,
    pub columns: Vec<RowColInfo>,
}


impl Map {
    pub fn print_map(&self) {
        for ln in &self.grid {
            println!("{}", ln.iter().map(|m| match m {
            Object::Wall => "#".normal().to_string(),
            Object::Empty => ".".normal().to_string(),
            Object::Void => " ".to_string(),
            Object::Visited(dir) => dir.get_char().to_string().blue().to_string(),
            Object::Loc => "X".to_string(),
            }).collect::<String>());
        }
    }

    pub fn print_loc(&self, state: &State) {
        let mut grid = self.grid.clone();
        let visited = grid.iter().rposition(|v| v.iter().any(|o| matches!(o, Object::Visited(_))));
        let max_i = if let Some(v) = visited {
            grid.len().min(v + 5)
        } else {
            grid.len() 
        };

        assert!(state.x >= 0);
        assert!(state.y >= 0);
        let x = state.x as usize;
        let y = state.y as usize;
        grid[y][x] = Object::Loc;
        for (j,ln) in grid[..max_i].iter().enumerate() {
            println!("{}", ln.iter().enumerate().map(|(i,m)| match m {
                Object::Wall => "#".blue().to_string(),
                Object::Empty => if (i == state.x as usize) || (j == state.y as usize) {
                    ".".red().to_string()
                } else {
                    ".".normal().to_string()
                    },
                Object::Void => " ".to_string(),
                Object::Visited(dir) => dir.get_char().to_string().magenta().to_string(),
                Object::Loc => state.direction.get_char().to_string().red().to_string(),
            }).collect::<String>());
        }
    }

    fn parse_face(&self, start_col: usize, start_row: usize, side_length: usize) -> Vec<Vec<Object>> {
        let end_x = start_col + side_length;
        let end_y = start_row + side_length;
        self.grid[start_row..end_y].iter().map(|v| {
            if v.len() < end_x {
                vec![]
            } else {
                v[start_col..end_x].to_owned()
            }
        }).collect()
    }
}


#[derive(Debug, Clone)]
pub struct State {
    pub x: i64,
    pub y: i64,
    pub direction: Dir,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, FromPrimitive, Hash)]
pub enum Dir3D {
    N, // positive y
    E, // positive x
    Up, // positive z
    S, // negative y
    W, // negative x
    Down, // negative z
}

impl Dir3D {
    pub fn new(c: char) -> Self {
        match c {
            '^' => Dir3D::N,
            'v' => Dir3D::S,
            '<' => Dir3D::W,
            '>' => Dir3D::E,
            'U' => Dir3D::Up,
            'D' => Dir3D::Down,
            _ => panic!("Unknown character"),
        }
    }

    pub fn opposite(self) -> Self {
        FromPrimitive::from_u8((self as u8 + 3) % 6).unwrap()
    }

    pub const fn get_dir(self) -> (i64,i64,i64) {
        match self {
            Self::N => (0, 1,0), // positive y
            Self::S => (0,-1,0), // negative y
            Self::E => (1, 0, 0), // positive x
            Self::W => (-1, 0, 0), // negative x
            Self::Up => (0,0,1), // positive z
            Self::Down => (0,0, -1), // negative z
        }
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct State3D {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub direction: Dir3D,
    pub face: Face,
}
