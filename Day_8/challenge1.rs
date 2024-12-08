use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

//DEBUGGIN: Para mostrar las antenas
fn display_antennas(antennas: &HashMap<char, Vec<(usize, usize)>>) {
    for (frequency, positions) in antennas {
        println!("Frequency: {}", frequency);
        for (x, y) in positions {
            println!("  Position: ({},{})", x, y);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect(); //Cogemos los argumentos

    if args.len() == 2 {
        let file_path = &args[1];

        let input = fs::read_to_string(file_path).expect("Error leyendo el archivo");

        //Construccion del mapa
        let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        //Creamos el mapa de antenas
        let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

        for (y, row) in map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if c.is_alphanumeric() {
                    antennas.entry(*c).or_insert_with(Vec::new).push((x, y));
                }
            }
        }

        //Calculamos los antinodos
        let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

        display_antennas(&antennas);

        //Calculamos los antinodos para cada frecuencia
        for (&freq, positions) in &antennas {
            for i in 0..positions.len() {
                for j in i + 1..positions.len() {
                    let (x1, y1) = positions[i];
                    let (x2, y2) = positions[j];

                    //Comprobamos las distancias
                    let dx = x2 as isize - x1 as isize;
                    let dy = y2 as isize - y1 as isize;

                    //Antinodos por cada lado
                    let ax1 = x1 as isize - dx;
                    let ay1 = y1 as isize - dy;
                    let ax2 = x2 as isize + dx;
                    let ay2 = y2 as isize + dy;

                    //Añadimos solo los antinodos validos
                    if ax1 >= 0
                        && ay1 >= 0
                        && ax1 < map[0].len() as isize
                        && ay1 < map.len() as isize
                    {
                        antinodes.insert((ax1 as usize, ay1 as usize));
                        println!("Antinodo freq {}: ({}, {})", freq, ax1, ay1);
                        map[ay1 as usize][ax1 as usize] = '#';
                    }

                    if ax2 >= 0
                        && ay2 >= 0
                        && ax2 < map[0].len() as isize
                        && ay2 < map.len() as isize
                    {
                        antinodes.insert((ax2 as usize, ay2 as usize));
                        println!("Antinodo freq {}: ({}, {})", freq, ax2, ay2);
                        map[ay2 as usize][ax2 as usize] = '#';
                    }
                }
            }
        }

        for element in map {
            println!("{:?}", element);
        }

        println!("Nodos totales: {}", antinodes.len());
    } else {
        println!("La estructura de llamada es ./challenge1 <file_path>")
    }
}
