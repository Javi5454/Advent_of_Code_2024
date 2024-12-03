use std::env;
use std::fs;

//Esta vez vamos a hacer una funcion que me diga si es segura o no para ahorrar código
fn is_safe(numbers: &[i32]) -> bool {
    //Variables de control de monotonia
    let mut is_decreasing = true;
    let mut is_increasing = true;

    for window in numbers.windows(2) {
        let diff = window[1] - window[0];

        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        if diff > 0 {
            is_decreasing = false;
        } else if diff < 0 {
            is_increasing = false;
        }
        //Si no totalmente monotona, error
        if !is_decreasing && !is_increasing {
            return false;
        }
    }

    return true;
}

fn main() {
    let args: Vec<String> = env::args().collect(); //Cogemos los argumentos

    //Pillamos los datos del input
    if args.len() == 2 {
        let file_path = &args[1];

        let input = fs::read_to_string(file_path).expect("Error leyendo el archivo");

        let mut total_safe = 0; //Resultado final

        for line in input.lines() {
            //Convertimos el vector a numeros
            let numbers: Vec<i32> = line
                .split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect();

            if is_safe(&numbers) {
                total_safe += 1;
            }
            //Si no es seguro, intentamos quitar un elemento
            else {
                for i in 0..numbers.len() {
                    let mut clone_numbers = numbers.clone();
                    clone_numbers.remove(i);

                    if is_safe(&clone_numbers) {
                        total_safe += 1;
                        break;
                    }
                }
            }
        }

        println!("{total_safe}");
    } else {
        println!("La estructura de llamada es ./challenge2 <file_path>");
    }
}
