use std::collections::HashMap;
use std::env;
use std::fs;

//Funcion para evolucionar las piedras
fn evolve_stones(stones: Vec<u64>, blinks: usize) -> HashMap<u64, usize> {
    let mut stone_counts: HashMap<u64, usize> = HashMap::new();

    //Inicializamos el conteno
    for &stone in &stones {
        *stone_counts.entry(stone).or_insert(0) += 1;
    }

    //Hacemos la evolucion
    for _ in 0..blinks {
        let mut next_counts: HashMap<u64, usize> = HashMap::new();

        for (&stone, &count) in &stone_counts {
            if stone == 0 {
                //Regla 1: Si es 0, se convierte en 1
                *next_counts.entry(1).or_insert(0) += count;

            } else if stone.to_string().len() % 2 == 0 {
                //Regla 2: Si numero par de digitos, dividir
                let digits = stone.to_string();
                let mid = digits.len() / 2;

                // Dividir en dos partes
                let left = digits[..mid].parse::<u64>().unwrap();
                let right = digits[mid..].parse::<u64>().unwrap();


                *next_counts.entry(left).or_insert(0) += count;
                *next_counts.entry(right).or_insert(0) += count;
            } else {
                //Regla 3; Multiplicar por 1024
                *next_counts.entry(stone * 2024).or_insert(0) += count;
            }
        }

        stone_counts = next_counts;
    }

    return stone_counts;
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

        let evolved_stones: HashMap<u64, usize> = evolve_stones(stones, 75); //Solo cambia esto

        for (stone, count) in &evolved_stones {
            println!("Número: {}, Cantidad: {}", stone, count);
        }

        let total: usize = evolved_stones.values().sum();
        println!("Piedras totales: {}", total);
    } else {
        println!("La estructura de llamada es ./challenge1 <file_path>")
    }
}
