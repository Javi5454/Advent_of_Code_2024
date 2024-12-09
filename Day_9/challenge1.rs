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

//Compacta los archivos moviendo bloques
fn compact_files(blocks: &mut Vec<Option<u32>>) {
    let mut free_index = 0; // Índice del primer espacio libre

    // Busca el primer espacio libre desde la izquierda
    while free_index < blocks.len() && blocks[free_index].is_some() {
        free_index += 1;
    }

    // Itera desde el final hacia el principio para mover bloques
    for i in (0..blocks.len()).rev() {
        if let Some(file_id) = blocks[i] {
            if i > free_index {
                // Mueve el bloque al primer espacio libre
                blocks[free_index] = Some(file_id);
                blocks[i] = None;

                // Actualiza `free_index` al siguiente espacio libre
                free_index += 1;
                while free_index < blocks.len() && blocks[free_index].is_some() {
                    free_index += 1;
                }
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

        //Paso 2: Compactamos los archivos
        compact_files(&mut blocks);

        //Paso 3: Calculamos el checksum
        let checksum = calculate_checksum(&blocks);
        println!("Checksum: {}", checksum);
        
    } else {
        println!("La estructura de llamada es ./challenge1 <file_path>")
    }
}
