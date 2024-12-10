use std::collections::VecDeque;
use std::env;
use std::fs;

fn calculate_trailhead_scores(map: Vec<Vec<u8>>) -> usize {
    let rows = map.len();
    let cols = map[0].len();
    let mut total_score = 0;

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

    //Hacemos una busqueda en anchura
    let bfs = |start_x: usize, start_y: usize| -> usize {
        let mut queue = VecDeque::new();
        let mut visited = vec![vec![false; cols]; rows];

        queue.push_back((start_x, start_y));
        visited[start_x][start_y] = true;

        let mut score = 0;

        while let Some((x, y)) = queue.pop_front() {
            for &(dx, dy) in &directions {
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if is_valid(nx, ny) {
                    let nx = nx as usize;
                    let ny = ny as usize;

                    if !visited[nx][ny] && map[nx][ny] == map[x][y] + 1 {
                        visited[nx][ny] = true;
                        queue.push_back((nx, ny));

                        //Si alcanzamos altura 9
                        if map[nx][ny] == 9 {
                            score += 1;
                        }
                    }
                }
            }
        }

        return score;
    };

    //Identificamos todos los altura 0
    for x in 0..rows {
        for y in 0..cols {
            if map[x][y] == 0 {
                total_score += bfs(x, y);
            }
        }
    }

    return total_score;
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
        println!("La estructura de llamada es ./challenge1 <file_path>")
    }
}
