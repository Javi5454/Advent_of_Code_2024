use std::collections::HashMap;
use std::env;
use std::fs;

fn calculate_trailhead_scores(map: Vec<Vec<u8>>) -> usize {
    let rows = map.len();
    let cols = map[0].len();

    //Direcciones para moverse
    let directions = [
        (0, 1),  //Abajo
        (1, 0),  //Derecha
        (0, -1), //Arriba
        (-1, 0), //Izquierda
    ];

    //Verificamos si una posicion es valida
    let is_valid =
        |x: isize, y: isize| -> bool { x >= 0 && x < rows as isize && y >= 0 && y < cols as isize };

    //Memoizacion para almacenar los resultados de caminos desde cada celda
    let mut memo: HashMap<(usize, usize), usize> = HashMap::new();

    //Funcion recursiva para contar caminos desde una posicion
    fn count_paths(
        x: usize,
        y: usize,
        map: &Vec<Vec<u8>>,
        directions: &[(isize, isize)],
        is_valid: &dyn Fn(isize, isize) -> bool,
        memo: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        //Si ya hemos calculado los caminos desde esta celda, devolvemos el valor
        if let Some(&cached) = memo.get(&(x, y)) {
            return cached;
        }

        //Si estamos en una celda con altura 9, hay un camino valido que termina aqui
        if map[x][y] == 9 {
            return 1;
        }

        let mut total_paths = 0;

        //Exploramos los vecinos validos
        for &(dx,dy) in directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if is_valid(nx,ny){
                let nx = nx as usize;
                let ny = ny as usize;

                //Solo podemos movernos a una celda cuya altura sea +1
                if map[nx][ny] == map[x][y] + 1 {
                    total_paths += count_paths(nx,ny, map, directions, is_valid, memo);
                }
            }
        }

        //Almacenamos el resultado de la memoizacion
        memo.insert((x,y), total_paths);

        return total_paths;
    }

    let mut total_rating = 0;

    //Identificamos todos los altura 0
    for x in 0..rows {
        for y in 0..cols {
            if map[x][y] == 0 {
                total_rating += count_paths(x, y, &map, &directions, &is_valid, &mut memo);
            }
        }
    }

    return total_rating;
}

fn main() {
    let args: Vec<String> = env::args().collect(); //Cogemos los argumentos

    if args.len() == 2 {
        let file_path = &args[1];

        let input = fs::read_to_string(file_path).expect("Error leyendo el archivo");

        //Construccion del mapa
        let map: Vec<Vec<u8>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();

        let total_score = calculate_trailhead_scores(map);

        println!("Puntuacion total: {}", total_score);
    } else {
        println!("La estructura de llamada es ./challenge2 <file_path>")
    }
}
