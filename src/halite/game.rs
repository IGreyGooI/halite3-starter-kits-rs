use super::input::tokenize;
use super::log::{Log, Logger};
use super::update::Update;
use std::io::{self, Read, Write, stdin, stdout, Stdin, Stdout};
use std::collections::HashMap;
use super::position::Position;
use std::{fmt, convert};

pub type ShipId = u32;
pub type PlayerId = u32;
pub type StructureId = u32;
pub type MapSize = (u32, u32);
pub type HaliteAmount = u32;
pub type CommandString = String;


pub trait FourDirection {
    const EAST: u8;
    const WEST: u8;
    const SOUTH: u8;
    const NORTH: u8;
}

pub struct Direction {
    d: u8
}

impl FourDirection for Direction {
    const EAST: u8 = 0x00;
    const WEST: u8 = 0x80;
    const SOUTH: u8 = 0xc0;
    const NORTH: u8 = 0x40;
}

// fmt::Display is for default formatter, right?
// yes
impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.d {
            Direction::EAST => f.write_str(&"e"),
            Direction::WEST => f.write_str(&"w"),
            Direction::SOUTH => f.write_str(&"s"),
            Direction::NORTH => f.write_str(&"n"),
            _ => panic!("invalid direction")
        }
    }
}

impl convert::Into<Direction> for u8 {
    fn into(self) -> Direction {
        Direction {
            d: self,
        }
    }
}

impl convert::Into<u8> for Direction {
    fn into(self) -> u8 {
        self.d
    }
}

#[derive(Debug)]
pub struct Khala {
    map_size: MapSize,
    pub game_constants: HashMap<String, String>,
    
    pub resource_map: Vec<Vec<u32>>,
    pub ships: Vec<Ship>,
    pub structures: Vec<Structure>,
    pub player_owned_halite: Vec<HaliteAmount>,
    
    pub num_players: u32,
    pub my_id: PlayerId,
    pub turn_number: u32,
    logger: Logger,
}

macro_rules! read_line_and_tokenize {
    ($buf: ident, $tokens: ident, $logger: ident) => {
        let mut $buf = String::new();
        stdin().read_line(&mut $buf).unwrap();
        let mut $tokens = tokenize(&mut $buf);
        $logger.log(
            format!(
                "\
                [Information] read_line_and_tokenize invoke at: {}::{} \n  \
                Read from stdin: \n    \
                buf: \n      \
                {}    \
                tokens: \n      \
                {:?}\
                \n",file!(), line!(), $buf, $tokens));
    };
    
    ($buf: ident, $tokens: ident) => {
        let mut $buf = String::new();
        stdin().read_line(&mut $buf).unwrap();
        let mut $tokens = tokenize(&mut $buf);
    }
}

impl Khala {
    pub fn read_from_stdin() -> Khala {
        let mut init_logger = Logger::new(env!("CARGO_MANIFEST_DIR"), "khala_init.log");
        
        let game_constants = {
            let buf = &mut String::new();
            let result = stdin().read_line(buf);
            match result {
                Ok(_) => (),
                Err(_) => {
                    panic!("cannot read stdin");
                }
            }
            
            let token_iter = buf
                .split(|c| " {},:\"\r\n".contains(c))
                .filter(|x| !x.is_empty());
            
            let tokens: Vec<&str> = token_iter.collect();
            init_logger.log(format!("Read from stdin: \nbuf: {}, \ntokens: {:#?}", buf, tokens));
            if (tokens.len() % 2) != 0 {
                init_logger.log(
                    "Error: constants: expected even total number of key and value tokens from server.");
            }
            let mut map = HashMap::new();
            init_logger.log(format!("Resolve as a HashMap: {:#?}", map));
            for i in (0..tokens.len()).step_by(2) {
                map.insert(tokens[i].to_string(), tokens[i + 1].to_string());
            }
            map
        };
        
        let (num_players, my_id): (u32, u32) = {
            read_line_and_tokenize!(buf, tokens, init_logger);
            (tokens[0].parse().unwrap(),
             tokens[1].parse().unwrap())
        };
        
        init_logger.log(format!("Resolve as num_players: {}, my_id: {:#?}", num_players, my_id));
        
        let mut logger =
            Logger::new(env!("CARGO_MANIFEST_DIR"), format!("khala_bot_{}.log", my_id));
        
        let structures = {
            let mut structures = Vec::<Structure>::new();
            for player_id in 0..num_players {
                structures.push({
                    read_line_and_tokenize!(buf, tokens, logger);
                    Structure {
                        structure_id: (0x4000 + player_id) as StructureId,
                        owner_id: tokens[0].parse().unwrap(),
                        position: (tokens[1].parse().unwrap(), tokens[2].parse().unwrap()),
                        structure_type: StructureType::Shipyard,
                    }
                });
            }
            structures
        };
        
        let (map_size, resource_map) = {
            let size: MapSize = {
                read_line_and_tokenize!(buf, tokens, logger);
                
                (tokens[0].parse().unwrap(),
                 tokens[1].parse().unwrap())
            };
            
            let cells = {
                let mut cells = Vec::<Vec<u32>>::new();
                for row in 0..(size.0 as usize) {
                    let mut this_row = Vec::<u32>::new();
                    read_line_and_tokenize!(buf, tokens, logger);
                    for col in 0..size.1 as usize {
                        this_row.push(tokens[col].parse().unwrap())
                    }
                    cells.push(this_row)
                }
                cells
            };
            (size, cells)
        };
        
        let ships = {
            Vec::<Ship>::new()
        };
        
        let turn_number = 0 as u32;
        
        let player_owned_halite = {
            let mut vec = Vec::<HaliteAmount>::new();
            for player_id in 0..num_players {
                vec.push(
                    game_constants
                        .get("INITIAL_ENERGY")
                        .unwrap()
                        .parse()
                        .unwrap()
                )
            }
            vec
        };
        
        Khala {
            map_size,
            resource_map,
            game_constants,
            ships,
            structures,
            num_players,
            my_id,
            logger,
            turn_number,
            player_owned_halite,
        }
    }
    
    pub fn ready<S: Into<String>>(&mut self, bot_name: S) {
        self.write_to_stdout(bot_name);
        self.write_to_stdout("\n");
    }
    
    pub fn end_turn(&mut self, commands: &[CommandString]) {
        commands.iter().for_each(|command| {
            self.write_to_stdout(command.clone());
            self.write_to_stdout(" ");
        });
        self.write_to_stdout("\n");
    }
    
    pub fn write_to_stdout<S: Into<String>>(&mut self, string: S) {
        let string: String = string.into();
        match stdout().write_all(string.clone().as_bytes()) {
            Err(io::Error { .. }) => {}
            Ok(()) => {
                self.logger.log(
                    format!(
                        "\
                        [Information] Writing the following String into stdout: \n  \
                        plain text: \n    \
                        \"{}\"\n  \
                        debug: \n    \
                        {:#?}\
                        \n", string, string));
            }
        }
    }
}

impl Update for Khala {
    fn update(&mut self) {
        let logger = &mut self.logger;
        self.turn_number = {
            read_line_and_tokenize!(buf, tokens, logger);
            tokens[0].parse().unwrap()
        };
        logger.log(format!("=============== TURN {} ================", self.turn_number));
        for player_id in 0..self.num_players {
            read_line_and_tokenize!(buf, tokens, logger);
            let current_player_id: PlayerId = tokens[0].parse().unwrap();
            let num_ships: u32 = tokens[1].parse().unwrap();
            let num_dropoffs: u32 = tokens[2].parse().unwrap();
            let halite: HaliteAmount = tokens[3].parse().unwrap();
            
            self.player_owned_halite[current_player_id as usize] = halite;
            
            self.ships = {
                let mut new_ships = Vec::<Ship>::new();
                for _ in 0..num_ships {
                    let ship = {
                        read_line_and_tokenize!(buf, tokens, logger);
                        let ship_id: ShipId = tokens[0].parse().unwrap();
                        let position: Position = {
                            (tokens[1].parse().unwrap(),
                             tokens[2].parse().unwrap())
                        };
                        let cargo = tokens[3].parse().unwrap();
                        Ship {
                            owner_id: current_player_id,
                            ship_id,
                            cargo,
                            position,
                        }
                    };
                    
                    new_ships.push(ship);
                }
                new_ships
            };
            
            self.structures = {
                let mut new_structures: Vec<Structure> = {
                    let (shipyards, dropoffs) = self.structures.iter().partition(|structure| {
                        structure.structure_type == StructureType::Shipyard
                    });
                    shipyards
                };
                
                for _ in 0..num_dropoffs {
                    let structure = {
                        read_line_and_tokenize!(buf, tokens, logger);
                        let structure_id: ShipId = tokens[0].parse().unwrap();
                        let position: Position = {
                            (tokens[1].parse().unwrap(),
                             tokens[2].parse().unwrap())
                        };
                        let structure_type = StructureType::Dropoff;
                        Structure {
                            owner_id: current_player_id,
                            structure_id,
                            position,
                            structure_type,
                        }
                    };
                    
                    new_structures.push(structure);
                }
                new_structures
            };
        }
        logger.log(format!("Done updating players and entities"));
        let update_count = {
            read_line_and_tokenize!(buf, tokens, logger);
            tokens[0].parse().unwrap()
        };
        logger.log(format!("Resource map needs update, update_count: {}", update_count));
        for _ in 0..update_count {
            read_line_and_tokenize!(buf, tokens, logger);
            let (x, y): MapSize = {
                (tokens[0].parse().unwrap(),
                 tokens[1].parse().unwrap())
            };
            let halite: HaliteAmount = tokens[2].parse().unwrap();
            
            self.resource_map[y as usize][x as usize] = halite;
        }
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Ship {
    pub owner_id: PlayerId,
    pub ship_id: ShipId,
    pub cargo: u32,
    pub position: Position,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Structure {
    pub owner_id: PlayerId,
    pub structure_id: StructureId,
    pub position: Position,
    pub structure_type: StructureType,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum StructureType {
    Shipyard,
    Dropoff,
}

use super::position::{RecursiveCellPosition, SizedGrid2D};

pub trait Grid2D<T> {
    fn get_at_position(&self, position: Position) -> T;
}

impl Grid2D<HaliteAmount> for Khala {
    fn get_at_position(&self, position: Position) -> HaliteAmount {
        self.resource_map[position.1 as usize][position.0 as usize]
    }
}

impl SizedGrid2D for Khala {
    fn get_size(&self) -> (u32, u32) {
        self.map_size
    }
}

pub trait Command<S> {
    fn spawn_ship() -> S;
    fn transform_ship_into_dropoff_site(ship_id: ShipId) -> S;
    fn move_ship_by_direction<D: FourDirection + fmt::Display>(ship_id: ShipId, direction: D) -> S;
    fn hold_ship(ship_id: ShipId) -> S;
}

impl Command<CommandString> for Khala {
    fn spawn_ship() -> CommandString {
        format!("g")
    }
    
    fn transform_ship_into_dropoff_site(ship_id: ShipId) -> CommandString {
        format!("c {}", ship_id)
    }
    
    fn move_ship_by_direction<D: FourDirection + fmt::Display>(ship_id: ShipId, direction: D) ->
    CommandString {
        format!("m {} {}", ship_id, direction)
    }
    
    fn hold_ship(ship_id: ShipId) -> CommandString {
        format!("m {} {}", ship_id, &"o")
    }
}
