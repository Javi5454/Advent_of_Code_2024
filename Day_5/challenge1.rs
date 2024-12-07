use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

// Para construir el grafo
fn build_graph(rules: &Vec<(i32, i32)>) -> HashMap<i32, HashSet<i32>> {
    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new(); //Nuestro grafo vacio

    for &(x, y) in rules {
        graph.entry(x).or_insert_with(HashSet::new).insert(y);

        graph.entry(y).or_insert_with(HashSet::new); //Aseguramos que exista como nodo
    }

    return graph;
}

//Verificamos si un update respeta las reglas del grafo
fn is_valid_update(graph: &HashMap<i32, HashSet<i32>>, update: &Vec<i32>) -> bool {
    let mut position = HashMap::new();

    //Mapeamos cada página a su posicion en la actualizacion
    for (i, &page) in update.iter().enumerate() {
        position.insert(page, i);
    }

    //Verificamos que todas las reglas se cumplan
    for (&x, ys) in graph.iter() {
        if let Some(&pos_x) = position.get(&x) {
            for &y in ys {
                if let Some(&pos_y) = position.get(&y) {
                    if pos_x > pos_y {
                        return false; //Regla violada
                    }
                }
            }
        }
    }

    return true;
}

//DEBUGGINF: Para mostrar el grafo
fn print_graph(graph: &HashMap<i32, HashSet<i32>>) {
    for (node, edges) in graph {
        print!("Nodo: {}", node);

        if edges.is_empty() {
            println!(" Sin conexiones");
        } else {
            let edges_list: Vec<String> = edges.iter().map(|e| e.to_string()).collect();
            println!(" Conecta a -> [{}]", edges_list.join(", "));
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect(); //Cogemos los argumentos

    if args.len() == 2 {
        let file_path = &args[1];

        let input = fs::read_to_string(file_path).expect("Error leyendo el archivo");

        let parted_input: Vec<&str> = input.split("\n\n").collect();

        let mut rules: Vec<(i32, i32)> = Vec::new();

        for line in parted_input[0].lines() {
            let first = line.split("|").collect::<Vec<_>>()[0].parse().unwrap();
            let second = line.split("|").collect::<Vec<_>>()[1].parse().unwrap();

            rules.push((first, second));
        }

        let updates = parted_input[1]; //Updates de paginas

        let graph = build_graph(&rules); //Contruimos el grafo

        let mut result = 0; //Resultado final

        print_graph(&graph);

        for (i, line) in updates.lines().enumerate() {
            let current_update: Vec<i32> = line
                .split(',')
                .filter_map(|s| s.trim().parse::<i32>().ok())
                .collect();

            if is_valid_update(&graph, &current_update) {
                println!("La actualizacion {} está en el orden correcto", i + 1);

                result = result + current_update[current_update.len() / 2];
            } else {
                println!("La actualizacion {} NO está en el orden correcto", i + 1);
            }
        }

        println!("Total: {}", result);
    } else {
        println!("La estructura de llamada es ./challenge1 <file_path>")
    }
}
