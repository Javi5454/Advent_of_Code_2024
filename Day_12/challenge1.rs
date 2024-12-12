use std::env;
use std::fs;

fn calculate_area_and_perimeter(map: Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let rows = map.len();
    let cols = if rows > 0 { map[0].len() } else { 0 };
    let mut visited = vec![vec![false; cols]; rows];
    let mut regions = Vec::new();

    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for r in 0..rows {
        for c in 0..cols {
            if !visited[r][c] {
                let plant_type = map[r][c];
                let mut stack = vec![(r, c)];
                let mut area = 0;
                let mut perimeter = 0;

                while let Some((x, y)) = stack.pop() {
                    if visited[x][y] {
                        continue;
                    }
                    visited[x][y] = true;
                    area += 1;

                    for &(dx, dy) in &directions {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        if nx >= 0 && nx < rows as isize && ny >= 0 && ny < cols as isize {
                            if map[nx as usize][ny as usize] == plant_type {
                                if !visited[nx as usize][ny as usize] {
                                    stack.push((nx as usize, ny as usize));
                                }
                            } else {
                                perimeter += 1;
                            }
                        } else {
                            perimeter += 1;
                        }
                    }
                }

                regions.push((area, perimeter));
            }
        }
    }

    regions
}


fn calculate_total_prince(regions: Vec<(usize,usize)>) -> usize {
    return regions.iter().map(|&(area, perimeter) | area*perimeter).sum();
}

fn main() {
    let args: Vec<String> = env::args().collect(); //Cogemos los argumentos

    if args.len() == 2 {
        let file_path = &args[1];

        let input = fs::read_to_string(file_path).expect("Error leyendo el archivo");

        //Construccion del mapa
        let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let regions = calculate_area_and_perimeter(map);

        //println!("{:?}", regions);

        let total_price = calculate_total_prince(regions);

        println!("{}", total_price);
    } else {
        println!("La estructura de llamada es ./challenge1 <file_path>")
    }
}
