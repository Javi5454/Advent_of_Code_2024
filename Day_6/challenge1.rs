use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect(); //Cogemos los argumentos

    if args.len() == 2 {
        let file_path = &args[1];

        let input = fs::read_to_string(file_path).expect("Error leyendo el archivo");

        //Construccion del mapa
        let mut lab_map: Vec<Vec<char>> = input.lines()
            .map(|line| line.chars().collect())
            .collect();

        //Direcciones y movimientos asociados
        let directions = [
            (0,-1), //Arriba
            (1,0), //Derecha
            (0,1), //Abajo
            (-1,0) //Izquierda
        ];

        //Encontramos la posicion inicial y la del guarda
        let mut position = (0,0);
        let mut direction_index = 0; //0 = Arriba, 1 = Derecha ...

        for (y, row) in lab_map.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if "^>v<".contains(cell){
                    position = (x as isize, y as isize); //Posicion del guarda
                    direction_index = match cell { //Posicion del guarda
                        '^' => 0,
                        '>' => 1,
                        'v' => 2,
                        '<' => 3,
                        _ => unreachable!(),
                    };
                    break;
                }
            }
        }

        lab_map[position.1 as usize][position.0 as usize] = '.'; //Limpiamos la posicion inicial

        //Simulacion del movimiento del guardia
        let mut visited_positions: HashSet<(isize,isize)> = HashSet::new();

        visited_positions.insert(position); //Posicion inicial

        println!("Posicion inicial: ({},{})", position.0, position.1);

        loop {
            let (dx, dy) = directions[direction_index]; //Movimiento

            let next_position = (position.0 + dx, position.1 + dy);

            println!("Next position: ({},{})", next_position.0, next_position.1);

            if  next_position.1 < 0 
                || next_position.1 >= lab_map.len() as isize
                || next_position.0 < 0
                || next_position.0 >= lab_map[0].len() as isize
                {
                    //Si salimos del mapa, paramos
                    break;
                } else if lab_map[next_position.1 as usize][next_position.0 as usize] == '#'{ //Giramos derecha
                    direction_index = (direction_index + 1) % 4;
                }
                else{ //Seguimos recto
                    position = next_position;
                    visited_positions.insert(position);
                }
        }

        //Imprimimos el numero de posicion distintas
        println!("Posiciones distintas: {}", visited_positions.len());
    } else {
        println!("La estructura de llamada es ./challenge1 <file_path>")
    }
}
