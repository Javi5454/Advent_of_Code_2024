use regex::Regex;
use std::env;
use std::fs; //Para expresiones regulares

fn main() {
    let args: Vec<String> = env::args().collect(); //Cogemos los argumentos

    //Pillamos los datos del input
    if args.len() == 2 {
        let file_path = &args[1];

        let input = fs::read_to_string(file_path).expect("Error leyendo el archivo");

        let mut result = 0; //Resultado final

        for line in input.lines() {
            let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

            for captura in re.captures_iter(line) {
                //Iteramos sobre las coincidencias
                let expresion = captura.get(0).unwrap().as_str(); //Esto pilla toda la expresion

                let num1: i32 = captura[1].parse().unwrap();
                let num2: i32 = captura[2].parse().unwrap();

                let mul = num1 * num2;

                println!("{} = {}", expresion, mul);

                result = result + mul;
            }
        }

        println!("Total: {}", result);
    } else {
        println!("La estructura de llamada es ./challenge1 <file_path>");
    }
}