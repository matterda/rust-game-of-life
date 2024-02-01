mod structs;
mod patterns;
use structs::{World, spawn_pattern};
use patterns::*;

use std::process::Command;


fn main(){
    Command::new("clear").status().unwrap();
    let mut world = World::new(20, 20);
    spawn_pattern(&mut world, GLIDER, 5, 5);
    spawn_pattern(&mut world, TOAD, 10, 10);

    loop {
        println!("{:}", world);
        wait("0.1");
        refresh();
        world.simulation_step();
    }
}


fn wait(s: &str) {
    let mut child = Command::new("sleep").arg(s).spawn().unwrap();
    let _result = child.wait().unwrap();
}

fn refresh() {
    Command::new("clear").status().unwrap();
}