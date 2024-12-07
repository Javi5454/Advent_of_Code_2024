use std::collections::{HashMap, HashSet, VecDeque};
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

//Corrige una actualizacion no valida aplicando un ordenamiento topologico
fn correct_update(graph: &HashMap<i32, HashSet<i32>>, update: &Vec<i32>) -> Vec<i32> {
    //Creamos un subgrafo solo con los nodos presentes en la actualizacion
    let mut subgraph: HashMap<i32, HashSet<i32>> = HashMap::new();

    for &node in update {
        if let Some(edges) = graph.get(&node){
            subgraph.insert(node, edges.iter().filter(|&&y| update.contains(&y)).cloned().collect());
        } else{
            subgraph.insert(node, HashSet::new());
        }
    }

    //Realizamos el ordenamiento topologico 
    return topological_sort(&subgraph);
}

//Realiza un ordenamiento topologico en un grafo dirigido acíclico
fn topological_sort(graph: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    let mut in_degree: HashMap<i32, usize> = HashMap::new();
    let mut zero_in_degree = VecDeque::new();
    let mut sorted = Vec::new();

    //Calculamos el grado de entrada de cada nodo
    for (&node, edges) in graph.iter() {
        in_degree.entry(node).or_insert(0);

        for &neighbor in edges {
            *in_degree.entry(neighbor).or_insert(0) += 1;
        }
    }

    //Buscamos los nodos con grado de entrada cero
    for (&node, &degree) in in_degree.iter() {
        if degree == 0 {
            zero_in_degree.push_back(node);
        }
    }

    //Pocesamos los nodos con entrada cero
    while let Some(node) = zero_in_degree.pop_front() {
        sorted.push(node);

        if let Some(neighbors) = graph.get(&node){
            for &neighbor in neighbors {
                if let Some(degree) = in_degree.get_mut(&neighbor) {
                    *degree -= 1;
                    if *degree == 0{
                        zero_in_degree.push_back(neighbor);
                    }
                }
            }
        }
    }

    return sorted;
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
            } else {
                println!("La actualizacion {} NO está en el orden correcto", i + 1);

                let corrected = correct_update(&graph, &current_update);

                result = result + corrected[corrected.len() / 2];

                println!("Ordenada correctamente: {:?}", corrected);
            }
        }

        println!("Total: {}", result);
    } else {
        println!("La estructura de llamada es ./challenge1 <file_path>")
    }
}
