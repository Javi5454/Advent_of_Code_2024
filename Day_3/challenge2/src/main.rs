use regex::Regex;
use itertools::Itertools;
use std::env;
use std::fs; //Para expresiones regulares

fn main() {
    let args: Vec<String> = env::args().collect(); //Cogemos los argumentos

    //Pillamos los datos del input
    if args.len() == 2 {
        let file_path = &args[1];

        let input = fs::read_to_string(file_path).expect("Error leyendo el archivo");

        let mut result = 0; //Resultado final

        let re_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap(); //Para buscar muls
        let re_control = Regex::new(r"(do\(\)|don't\(\))").unwrap(); //Para buscar cadenas de control

        let mut enabled = true; //Variable de control solo si los mul estan habilitados

        for line in input.lines() {
            //Creamos un iterador combinado
            let combined_iterator = re_control
                .find_iter(line)
                .chain(re_mul.find_iter(line))
                .collect::<Vec<_>>()
                .into_iter()
                .sorted_by_key(|m| m.start()); //Ordenamos por orden de aparacion

            for cap in combined_iterator {
                let text = cap.as_str(); //Para obtener el texto

                if text == "do()" {
                    //Hemos encontrado un do
                    enabled = true; //Habilitamos las operaciones
                } else if text == "don't()" {
                    enabled = false; //Deshabilitamos
                } else if enabled {
                    //Caso si encontramos un mul
                    if let Some(captures) = re_mul.captures(text) {
                        let num1: i32 = captures[1].parse().unwrap();
                        let num2: i32 = captures[2].parse().unwrap();

                        result += num1 * num2;
                    }
                }
            }
        }

        println!("Total: {}", result);
    } else {
        println!("La estructura de llamada es ./challenge1 <file_path>");
    }
}
