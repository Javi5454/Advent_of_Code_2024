use std::collections::{HashSet, VecDeque};

fn find_groups_with_coordinates(map: &[&str]) -> Vec<(char, Vec<(usize, usize)>)> {
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
                let plant_type = map[r].chars().nth(c).unwrap();
                let mut queue = VecDeque::new();
                queue.push_back((r, c));
                visited[r][c] = true;
                let mut group_coords = Vec::new();

                // BFS para explorar el grupo conectado
                while let Some((x, y)) = queue.pop_front() {
                    group_coords.push((x, y));
                    for &(dx, dy) in &directions {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        if nx >= 0 && nx < rows as isize && ny >= 0 && ny < cols as isize {
                            let nx = nx as usize;
                            let ny = ny as usize;
                            if !visited[nx][ny] && map[nx].chars().nth(ny).unwrap() == plant_type {
                                visited[nx][ny] = true;
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

fn main() {
    let map_data = [
        "EEEEE",
        "EXXXX",
        "EEEEE",
        "EXXXX",
        "EEEEE"
    ];

    let groups = find_groups_with_coordinates(&map_data);

    for (plant_type, coords) in groups {
        println!("Plant type '{}', Coordinates: {:?}", plant_type, coords);
    }
}
