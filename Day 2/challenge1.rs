use std::env;
use std::fs;

fn main(){
    let args: Vec<String> = env::args().collect(); //Cogemos los argumentos

    //Pillamos los datos del input
    if args.len() == 2{
        let file_path = &args[1];

        let input = fs::read_to_string(file_path).expect("Error leyendo el archivo");

        let mut total_safe = 0; //Resultado final

        for line in input.lines(){
            let str_numbers: Vec<&str> = line.split(" ").collect();

            //Convertimos el vector a numeros
            let numbers : Vec<i32> = str_numbers.iter().map(|&x| x.parse().unwrap()).collect();
            
            let mut safe = true; //Para ver si es segura o no la linea

            //Para ver si es creciente o decreciente
            let mut is_increasing = true;
            let mut is_decreasing = true;

            for window in numbers.windows(2) {
                let diff = (window[1] - window[0]).abs();

                if diff < 1 || diff > 3 {
                    safe = false;
                    break;
                }

                if window[0] < window[1]{
                    is_decreasing = false;
                } else if window[0] > window[1] {
                    is_increasing = false;
                }

                //Si no es totalmente decreciente o creciente, salimos
                if !is_decreasing && !is_increasing {
                    safe = false;
                    break;
                }
            }

            if safe {
                total_safe = total_safe + 1;
            }
        }

        println!("{total_safe}");
    } else{
        println!("La estructura de llamada es ./challenge1 <file_path>");
    }
}