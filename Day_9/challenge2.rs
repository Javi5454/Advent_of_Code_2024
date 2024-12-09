use std::env;
use std::fs;

//Función para parsear el mapa del disco en bloques
fn parse_disk_map(disk_map: &str) -> Vec<Option<u32>> {
    let mut blocks = Vec::new();
    let mut file_id = 0;

    for(i, c) in disk_map.chars().enumerate() {
        let length = c.to_digit(10).unwrap() as usize;

        if i % 2 == 0 {
            blocks.extend(vec![Some(file_id); length as usize]);

            file_id += 1;
        } else{
            //Espacio libre
            blocks.extend(vec![None; length as usize]);
        }
    }

    return blocks;
}

fn move_whole_files(blocks: &mut Vec<Option<u32>>) {
    // Calcula los tamaños de los archivos
    let mut file_sizes = std::collections::HashMap::new();
    for &block in blocks.iter() {
        if let Some(file_id) = block {
            *file_sizes.entry(file_id).or_insert(0) += 1;
        }
    }

    // Ordena los archivos por ID decreciente
    let mut sorted_files: Vec<(u32, usize)> = file_sizes.into_iter().collect();
    sorted_files.sort_by(|a, b| b.0.cmp(&a.0));

    // Intenta mover cada archivo
    for (file_id, size) in sorted_files {
        let mut free_space_start = None;
        let mut free_space_length = 0;

        // Busca un espacio libre lo suficientemente grande antes del archivo actual
        for i in 0..blocks.len() {
            //println!("{i}");
            if blocks[i].is_none() {
                if free_space_start.is_none() {
                    free_space_start = Some(i);
                }
                free_space_length += 1;

                if free_space_length == size {
                    // Verifica si el espacio libre está antes del archivo actual
                    let start = free_space_start.unwrap();

                    // Asegúrate de que el espacio libre está completamente antes del archivo
                    let file_start = blocks.iter().position(|&b| b == Some(file_id)).unwrap();
                    if start + size <= file_start {
                        // Mueve el archivo al nuevo espacio
                        for j in 0..size {
                            blocks[start + j] = Some(file_id);
                        }

                        // Limpia la ubicación original del archivo
                        for k in 0..blocks.len() {
                            if blocks[k] == Some(file_id) && (k < start || k >= start + size) {
                                blocks[k] = None;
                            }
                        }

                        break;
                    }
                }
            } else {
                free_space_start = None;
                free_space_length = 0;
            }
        }
    }
}


//Calcula el checksum
fn calculate_checksum(blocks: &[Option<u32>]) -> u128 {
    blocks.iter()
        .enumerate()
        .filter_map(|(pos, &block)| block.map(|id| pos as u128 * id as u128))
        .sum() //Hay que usar u128 o si no overflow
}


fn main() {
    let args: Vec<String> = env::args().collect(); //Cogemos los argumentos

    if args.len() == 2 {
        let file_path = &args[1];

        let input = fs::read_to_string(file_path).expect("Error leyendo el archivo");

        //Paso 1: Parsear el mapa del disco en bloques
        let mut blocks = parse_disk_map(&input);

        //println!("ORIGINAL: {:?}", blocks);

        //println!("{}", blocks.len());

        //Paso 2: Compactamos los archivos
        move_whole_files(&mut blocks);

        //println!("{:?}", blocks);

        //Paso 3: Calculamos el checksum
        let checksum = calculate_checksum(&blocks);
        println!("Checksum: {}", checksum);
        
    } else {
        println!("La estructura de llamada es ./challenge2 <file_path>")
    }
}