use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs;

fn find_groups_with_coordinates(map: Vec<Vec<char>>) -> Vec<(char, Vec<(isize, isize)>)> {
    let rows = map.len();
    let cols = if rows > 0 { map[0].len() } else { 0 };
    let mut visited = vec![vec![false; cols]; rows];
    let mut groups = Vec::new();

    // Direcciones para explorar (arriba, abajo, izquierda, derecha)
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for r in 0..rows {
        for c in 0..cols {
            if !visited[r][c] {
                // Tipo de planta en la celda actual
                let plant_type = map[r][c];
                let mut queue = VecDeque::new();
                queue.push_back((r as isize, c as isize));
                visited[r][c] = true;
                let mut group_coords = Vec::new();

                // BFS para explorar el grupo conectado
                while let Some((x, y)) = queue.pop_front() {
                    group_coords.push((x as isize, y as isize));
                    for &(dx, dy) in &directions {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        if nx >= 0 && nx < rows as isize && ny >= 0 && ny < cols as isize {
                            let nx = nx;
                            let ny = ny;
                            if !visited[nx as usize][ny as usize] && map[nx as usize][ny as usize] == plant_type {
                                visited[nx as usize][ny as usize] = true;
                                queue.push_back((nx, ny));
                            }
                        }
                    }
                }

                // Añadir el grupo encontrado al resultado
                groups.push((plant_type, group_coords));
            }
        }
    }

    groups
}

fn sides(group: &Vec<(char, Vec<(isize, isize)>)>) -> Vec<(char, usize)> {
    let mut result: Vec<(char, usize)> = Vec::new();

    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for (plant, form) in group {
        let mut s: HashSet<(isize, isize, isize, isize)> = HashSet::new();

        for &(r, c) in form {
            for &(dr, dc) in &directions {
                if form.contains(&(r + dr, c + dc)) {
                    continue;
                }

                let (mut rr, mut cc) = (r, c);

                while form.contains(&(rr + dc, cc + dr)) && !form.contains(&(rr + dr, cc + dc)) {
                    rr += dc;
                    cc += dr;
                }
                s.insert((rr, cc, dr, dc));
            }
        }
        let total_sides = s.len();
        result.push((*plant, total_sides));
    }

    return result;
}

fn calculate_total_prince(group: &Vec<(char, Vec<(isize, isize)>)>, sides: Vec<(char, usize)>) -> usize {
    let mut result = 0;
    
    //Puesto que los vectores respetan el orden de consulta

    for i in 0..group.len(){
        let (_, total_sides) = sides[i];
        let (_, area) = &group[i];
        result += area.len() * total_sides;
    }

    result
}

fn main() {
    let args: Vec<String> = env::args().collect(); //Cogemos los argumentos

    if args.len() == 2 {
        let file_path = &args[1];

        let input = fs::read_to_string(file_path).expect("Error leyendo el archivo");

        //Construccion del mapa
        let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let groups = find_groups_with_coordinates(map);

        for (plant_type, coords) in &groups {
            println!("Plant Type: {}, Coordinates: {:?}", plant_type, coords);
        }

        let total_sides: Vec<(char, usize)> = sides(&groups);


        println!("{:?}", total_sides);

        let total = calculate_total_prince(&groups, total_sides);

        println!("Resultado final: {}", total);
    } else {
        println!("La estructura de llamada es ./challenge2 <file_path>")
    }
}
