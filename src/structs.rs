use std::fmt::Display;


#[derive(Debug)]
pub struct World {
    height: usize,
    width: usize,
    matrix: Vec<Vec<bool>>
}

impl World {
    pub fn new(height: usize, width: usize) -> World {
        World{height, width, matrix: gen_matrix(height, width)}
    }
    pub fn get_cell(&self, x: usize, y: usize) -> &bool {
        let (x, y) = self.check_x_y(x, y);
        &self.matrix[y][x]
    }
    pub fn update_cell(&mut self, x: usize, y: usize, value: bool) {
        let (x, y) = self.check_x_y(x, y);
        self.matrix[y][x] = value
    }
    fn check_x_y(&self, x: usize, y: usize) -> (usize, usize) {
        let mut new_x = x;
        let mut new_y = y;
        if x >= self.width {
            new_x = x % (self.width);
        } if y >= self.height {
            new_y = y % (self.height);
        }
        (new_x, new_y)
    }
    pub fn get_cell_neighbors(&self, x: usize, y: usize) -> Vec<&bool> {
        let mut neighbors = vec![];
        for loc_x in -1..=1 {
            for loc_y in -1..=1 {
                if (loc_x != 0) | (loc_y != 0) {
                    let mut neigh_x = (x as i32 + loc_x) as usize;
                    let mut neigh_y = (y as i32 + loc_y) as usize;
                    (neigh_x, neigh_y) = self.check_x_y(neigh_x, neigh_y);
                    neighbors.push(self.get_cell(neigh_x, neigh_y));
                }
            }
        }
        neighbors
    }
    pub fn simulation_step(&mut self) {
        let mut new_world = World::new(self.height, self.width);
        for y in 0..=self.width {
            for x in 0..=self.height {
                let curr_state = self.get_cell(x, y);
                let neighbors = self.get_cell_neighbors(x, y);
                let new_state = get_cell_state(curr_state, neighbors);
                new_world.update_cell(x, y, new_state);
            }
        }
        self.matrix = new_world.matrix;
    }
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s: String = String::new();
        for line in self.matrix.iter() {
            for is_alive in line.iter() {
                if is_alive == &true {
                    s.push_str("██")
                } else {
                    s.push_str("░░")
                }
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}


fn gen_matrix(height: usize, width: usize) -> Vec<Vec<bool>> {
    let c = false;
    vec![vec![c; width]; height]
}


fn get_cell_state(curr_state: &bool, neighbors: Vec<&bool>) -> bool{
    let n_live_neighbors: usize = neighbors.iter().filter(|i| ***i == true).count();
    match (*curr_state, n_live_neighbors) {
        (true, 0..=1) => false, // underpopulation
        (true, 2..=3) => true, // survive to next generation
        (true, _) => false, // overpopulation
        (false, 3) => true, // reproduction
        _ => false,
    }
}


pub fn spawn_pattern(world: &mut World, pattern: &str, x: usize, y: usize) {
    for (yb, line) in pattern.split('\n').enumerate() {
        for (xb, c) in line.chars().step_by(2).enumerate() {
            if c == '█' {
                world.update_cell(xb + x, yb + y, true);
            }
        }
    }
}
