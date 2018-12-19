use std::fs;
use std::io::prelude::*;use std::fs::File;

use std::io::{BufRead, BufReader};

const WIDTH: usize = 50;
const HEIGHT: usize = 50;

const OPENGROUND: char = '.';
const TREES: char = '|';
const LUMBERYARD: char = '#';

#[derive(Clone)]
struct map{
    grid : [[char; HEIGHT]; WIDTH],
}

impl map{
    fn print(&self){
        for y in 0..WIDTH{
            for x in 0..HEIGHT{
                print!("{} ", self.grid[x][y]);
            }
            println!();
        }
    }

    fn count_char(&self, c : char) -> u32{
        let mut count = 0;
        for row in self.grid.iter(){
            for item in row.iter(){
                if item.clone() == c{
                    count += 1;
                }
            } 
        }
        count
    }

    fn update(&mut self){
        let mut map_temp = self.clone();
        for x in 0..WIDTH{
            for y in 0..HEIGHT{
                match map_temp.grid[x][y]{
                    TREES => map_temp.update_tree(&self, x,y),
                    OPENGROUND => map_temp.update_openground(&self, x,y),
                    LUMBERYARD => map_temp.update_lumberyard(&self, x,y),
                    _ => {
                        println!("Ill-formatted map, exiting");
                        return;
                    }
                }
            }
        }
        *self = map_temp;
    }

    fn count_adjacent(&self, x: usize, y: usize, c: char) -> u32{
        let mut count = 0;

        let x_start = if x > 0{
            x - 1
        }else{
            0
        };

        let x_end = if x < WIDTH - 1{
            x + 1
        }else{
            WIDTH - 1
        };

        let y_start = if y > 0{
            y - 1
        }else{
            0
        };

        let y_end = if y < HEIGHT - 1{
            y + 1
        }else{
            HEIGHT - 1
        };

        for x1 in x_start..x_end+1{
            for y1 in y_start..y_end+1{
                if self.grid[x1][y1] == c && (x1 != x || y1 != y){
                    count += 1;
                }
            }
        }

        count
    }
        
    fn update_tree(&mut self, m: &map, x : usize, y : usize){
        if m.count_adjacent(x, y, LUMBERYARD) >= 3 {
            self.grid[x][y] = LUMBERYARD;
        }
    }
    fn update_openground(&mut self, m: &map, x : usize, y : usize){
        if m.count_adjacent(x, y, TREES) >= 3{
            self.grid[x][y] = TREES;
        }
    }
    fn update_lumberyard(&mut self, m: &map, x : usize, y : usize){
        if m.count_adjacent(x, y, LUMBERYARD) == 0 || 
            m.count_adjacent(x, y, TREES) == 0{
                self.grid[x][y] = OPENGROUND;
        }
    }

    fn calculate_resourcevalue(&self) -> u32{
        self.count_char(TREES) * self.count_char(LUMBERYARD)
    }
}

fn readFile(m : &mut map){
    let mut contents = String::new();

    let mut f = BufReader::new(File::open("src/input.txt").expect("awdawd"));
    let mut buf = Vec::<u8>::new();
    let mut next_x = 0;
    let mut next_y = 0;
    while f.read_until(b'\n', &mut buf).expect("awdawd") != 0{
        let s = String::from_utf8(buf).expect("awdawd");
        
        
        for c in s.chars(){
            if c != LUMBERYARD{
                if c != TREES{
                    if c != OPENGROUND{
                        continue;
                    }
                }
            }
            m.grid[next_x][next_y] = c;
            next_x += 1;
            
            if(next_x >= WIDTH){
                next_x = 0;
                next_y += 1;
                if next_y >= HEIGHT{
                    next_y = 0;
                }
            }
        }

        buf = s.into_bytes();
        buf.clear();
    }
}

fn main() {
    let mut m1 = map {grid : [['|'; WIDTH];HEIGHT]};
    readFile(&mut m1);

    for i in 0..1000000000{
        m1.update();
        if (i % 10000 == 0){
        println!("Update: {}", i);
        }
    }
    println!("Resource values: {:?}", m1.calculate_resourcevalue());
}