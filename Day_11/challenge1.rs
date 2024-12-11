use std::env;
use std::fs;

//Funcion para evolucionar las piedras
fn evolve_stones(stones: Vec<u64>, blinks: usize) -> Vec<u64> {
    let mut current_stones = stones;

    for i in 0..blinks {
        let mut next_stones = Vec::new();

        for &stone in &current_stones {
            if stone == 0 {
                //Regla 1: Si es 0 pasa a 1
                next_stones.push(1);

            } else if stone.to_string().len() % 2 == 0 {
                //Regla 2: Si tiene un numero par de digitos, partimos
                let digits = stone.to_string();
                let mid = digits.len() / 2;

                let left = digits[..mid].parse::<u64>().unwrap();
                let right = digits[mid..].parse::<u64>().unwrap();

                next_stones.push(left);
                next_stones.push(right);
            } else {
                //Regla 3: Multiplicamos por 2024
                next_stones.push(stone * 2024);
            }
        }

        current_stones = next_stones;
    }

    return current_stones;
}

fn main() {
    let args: Vec<String> = env::args().collect(); //Cogemos los argumentos

    if args.len() == 2 {
        let file_path = &args[1];

        let input = fs::read_to_string(file_path).expect("Error leyendo el archivo");

        //Construccion del mapa
        let stones: Vec<u64> = input
            .split_whitespace()
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();

        let evolved_stones = evolve_stones(stones, 7);

        println!("{:?}", evolved_stones);

        println!("Piedras totales: {}", evolved_stones.len());
    } else {
        println!("La estructura de llamada es ./challenge1 <file_path>")
    }
}
