use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
    direction: Direction,
}

impl Position {
    //Constructor
    fn new(x: isize, y: isize, direction: Direction) -> Self {
        Position { x, y, direction }
    }
    //Movimiento de girar a la derecha
    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    //Funcion para avanzar
    fn move_forward(&mut self) {
        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    // Veririca si la posicion está dentro de los limites del mapa
    fn is_within_bounds(&self, width: isize, height: isize) -> bool {
        self.x >= 0 && self.x < width && self.y >= 0 && self.y < height
    }
}

//Simulacion del guarda y vemos si es bucle
fn simulate_guard(grid: &mut Vec<Vec<char>>, start_pos: Position) -> bool {
    let mut visited = HashSet::new();
    let mut current_pos = start_pos;

    grid[current_pos.y as usize][current_pos.x as usize] = '^';

    loop {
        let mut next_pos = current_pos;
        next_pos.move_forward();

        //Verificar si la posicion siguiente esta fuera del mapa
        if !next_pos.is_within_bounds(grid[0].len() as isize, grid.len() as isize) {
            break;
        }

        //Verificamos si hay un obstaculo
        if grid[next_pos.y as usize][next_pos.x as usize] == '#' {
            //Si lo hay, giramos
            current_pos.turn_right();

            if grid[current_pos.y as usize][current_pos.x as usize] != '^' {
                grid[current_pos.y as usize][current_pos.x as usize] = '+';
            }
        } else {
            //Si no, avanzamos
            current_pos.move_forward();
            if grid[current_pos.y as usize][current_pos.x as usize] != '^' {
                grid[current_pos.y as usize][current_pos.x as usize] = '|';
            }
        }

        if !visited.insert(current_pos){
            return true;
        }
    }
    return false;
}

//Funcion para buscar bucles
fn find_loop_positions(grid: Vec<Vec<char>>, start_pos: Position) -> usize {
    let mut posibble_positions = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '.' && (x as isize != start_pos.x || y as isize != start_pos.y) {
                //Creamos un nuevo grid con el nuevo obstaculo
                let mut new_grid = grid.clone();
                new_grid[y][x] = '#';

                //Simulamos el movimiento
                let visited_positions = simulate_guard(&mut new_grid, start_pos);

                println!("({},{})", x, y);

                //Comprobamos si hemos entrado en un bucle
                if visited_positions{
                    posibble_positions += 1;
                }
            }
        }
    }

    return posibble_positions;
}

fn main() {
    let args: Vec<String> = env::args().collect(); //Cogemos los argumentos

    if args.len() == 2 {
        let file_path = &args[1];

        let input = fs::read_to_string(file_path).expect("Error leyendo el archivo");

        //Construccion del mapa
        let lab_map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        //Encontramos la posicion inicial y la del guarda
        let mut start_pos: Position = Position {
            x: 0,
            y: 0,
            direction: Direction::Up,
        };

        for (y, row) in lab_map.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if "^>v<".contains(cell) {
                    start_pos.x = x as isize; //Posicion del guarda
                    start_pos.y = y as isize;
                    start_pos.direction = match cell {
                        //Posicion del guarda
                        '^' => Direction::Up,
                        '>' => Direction::Right,
                        'v' => Direction::Down,
                        '<' => Direction::Left,
                        _ => unreachable!(),
                    };
                    break;
                }
            }
        }

        let result = find_loop_positions(lab_map, start_pos);

        println!(
            "Número de posiciones posibles para generar un bucle: {}",
            result
        );
    } else {
        println!("La estructura de llamada es ./challenge2 <file_path>")
    }
}
