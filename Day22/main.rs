type Coordinate = u32;
type Point = (Coordinate, Coordinate);

const TARGET_X : Coordinate = 14;
const TARGET_Y : Coordinate = 778;
const DEPTH : u32 = 11541;

use std::collections::HashMap;

enum RegionType{
    Rocky,
    Wet,
    Narrow,
}

fn get_erosion_level(x : Coordinate, y : Coordinate, erosion_map : &mut HashMap<Point, u32>) -> u32{
    match erosion_map.get(&(x,y)){
        Some(&erosion_level) => {erosion_level},
        None => {
            let erosion_level = (get_geo_index(x,y, erosion_map) + DEPTH) % 20183;
            erosion_map.insert((x,y), erosion_level);
            erosion_level
        },
    }
}

fn get_geo_index(x : Coordinate, y : Coordinate, erosion_map : &mut HashMap<Point, u32>) -> u32{
    match (x, y){
        (0,0) => 0,
        (TARGET_X, TARGET_Y) => 0,
        (_x, 0) => x * 16807,
        (0, _y) => y * 48271,
        _ => get_erosion_level(x-1, y, erosion_map) * get_erosion_level(x, y-1, erosion_map), 
    }
}

fn get_region_type(x : Coordinate, y : Coordinate, erosion_map : &mut HashMap<Point, u32>) -> RegionType{
    match get_erosion_level(x,y, erosion_map) % 3{
        0 => RegionType::Rocky,
        1 => RegionType::Wet,
        2 => RegionType::Narrow,
        _ => panic!("Modulo operator error")
    }
}

fn calculate_total_risk_level(target_x : Coordinate, target_y : Coordinate) -> u32{
    let mut erosion_map : HashMap<Point, u32> = HashMap::new();
    let mut risk = 0;
    for y in 0..target_y+1{
        for x in 0..target_x+1{
            risk += match get_region_type(x,y, &mut erosion_map){
                RegionType::Rocky => 0,
                RegionType::Wet => 1,
                RegionType::Narrow => 2,                
            }
        }
    }
    risk
}

fn main() {
    println!("Risk: {}", calculate_total_risk_level(TARGET_X, TARGET_Y));
}