use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect(); //Cogemos los argumentos

    if args.len() == 2{
        let file_path = &args[1];

        let input = fs::read_to_string(file_path).expect("Error leyendo el archivo");

        let grid: Vec<&str> = input.lines().collect();

        let word = "XMAS";
        let occurrences = count_word_occurrences(&grid, word);
        println!("La palabra '{}' aparece {} veces.", word, occurrences);
    }
    else{
        println!("La estructura de llamada es ./challenge1 <file_path>");
    }
}

fn count_word_occurrences(grid: &[&str], word: &str) -> usize {
    let directions = [
        (0, 1),   // derecha
        (1, 0),   // abajo
        (1, 1),   // diagonal abajo-derecha
        (1, -1),  // diagonal abajo-izquierda
        (0, -1),  // izquierda
        (-1, 0),  // arriba
        (-1, -1), // diagonal arriba-izquierda
        (-1, 1),  // diagonal arriba-derecha
    ];

    let mut count = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    for i in 0..rows {
        for j in 0..cols {
            for &(dx, dy) in &directions {
                if check_word(grid, word, i as isize, j as isize, dx, dy) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn check_word(grid: &[&str], word: &str, mut x: isize, mut y: isize, dx: isize, dy: isize) -> bool {
    for ch in word.chars() {
        if x < 0 || y < 0 || x >= grid.len() as isize || y >= grid[0].len() as isize {
            return false;
        }
        
        if grid[x as usize].chars().nth(y as usize).unwrap() != ch {
            return false;
        }

        x += dx;
        y += dy;
    }
    
    true
}
