#![feature(associated_consts)]

extern crate lazy_static;
extern crate rand;
extern crate num;
#[macro_use]
extern crate bitflags;


#[allow(unused_mut)]
#[allow(unused_assignments)]
#[allow(unused_imports)]
#[macro_use]
pub mod halite;

use crate::halite::update::Update;
use crate::halite::game::{Khala, Grid2D, Command, CommandString, Direction, Ship, StructureType,
                          Structure};

use rand::Rng;
use rand::SeedableRng;
use rand::XorShiftRng;

use std::env;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use std::io::{Read, Write, stdin};
use std::fs::File;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rng_seed: u64 = if args.len() > 1 {
        args[1].parse().unwrap()
    } else {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    };
    let seed_bytes: Vec<u8> = (0..16).map(|x| ((rng_seed >> (x % 8)) & 0xFF) as u8).collect();
    let mut rng: XorShiftRng = SeedableRng::from_seed([
        seed_bytes[0], seed_bytes[1], seed_bytes[2], seed_bytes[3],
        seed_bytes[4], seed_bytes[5], seed_bytes[6], seed_bytes[7],
        seed_bytes[8], seed_bytes[9], seed_bytes[10], seed_bytes[11],
        seed_bytes[12], seed_bytes[13], seed_bytes[14], seed_bytes[15]
    ]);
    
    let mut khala = halite::game::Khala::read_from_stdin();
    khala.ready(format!("bot_{}", khala.my_id));
    'main: loop {
        khala.update();
        
        let (my_ships, _): (Vec<Ship>, Vec<Ship>) =
            khala.ships.iter().partition(|ship| { ship.owner_id == khala.my_id });
        
        let my_shipyard = {
            khala.structures.iter().find(|structure| {
                structure.structure_type == StructureType::Shipyard &&
                    structure.owner_id == khala.my_id
            }).unwrap()
        };
        
        let mut command_queue: Vec<CommandString> = Vec::new();
        
        my_ships.iter().for_each(
            |ship| {
                let command = if
                    khala.get_at_position(ship.position) < khala.game_constants
                                                                .get("MAX_CELL_PRODUCTION")
                                                                .unwrap()
                                                                .parse()
                                                                .unwrap() {
                    Khala::move_ship_by_direction::<Direction>(
                        ship.ship_id,
                        (0x40 * rng.gen_range(0, 4)).into())
                } else {
                    Khala::hold_ship(ship.ship_id)
                };
                command_queue.push(command);
            }
        );
        
        if khala.turn_number <= 200 &&
            khala.player_owned_halite[khala.my_id as usize] >=
                khala.game_constants.get("NEW_ENTITY_ENERGY_COST")
                     .unwrap()
                     .parse()
                     .unwrap() &&
            my_ships.iter().find(|ship| { ship.position == my_shipyard.position }) == None {
            command_queue.push(Khala::spawn_ship());
        }
        khala.end_turn(&command_queue);
    }
}


fn print_input() {
    let mut buf = String::new();
    
    let result = stdin().read_to_string(&mut buf).unwrap();
    
    let mut file = File::create(
        Path::new("C:\\Users\\zsw_s\\CLionProjects\\archon").join("input.in"))
        .expect(&format!("Couldn't open file for logging!"));
    
    writeln!(file, "{}", buf).unwrap();
}
