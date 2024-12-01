use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    //Comprobamos la cantidad de argumentos
    if args.len() != 1 {
        let file_path = &args[1];

        let input = fs::read_to_string(file_path).expect("Error leyendo el archivo!");

        //Vectores de los numeros izquierda a la derecha
        let mut left_numbers: Vec<i32> = Vec::new();
        let mut right_numbers: Vec<i32> = Vec::new();

        for line in input.lines() {
            let numbers: Vec<&str> = line.split("   ").collect(); //Separamos los numeros

            //Pillamos los numeros
            let left_number = numbers[0].parse().expect("No se pudo convertir a i32");
            let right_number = numbers[1].parse().expect("No se pudo convertir a i32");

            left_numbers.push(left_number);
            right_numbers.push(right_number);
        }

        //Ordenamos los vectores
        left_numbers.sort();
        right_numbers.sort();

        //Para guardar la suma
        let mut suma = 0;
        let i = 0;

        for index in i..=left_numbers.len() - 1 { //Lo recorremos por indice
            suma = suma + (left_numbers[index] - right_numbers[index]).abs();
        }

        //Mostramos el resultado
        println!("{suma}");
    } else {
        println!("La estructura de llamada es ./challenge1 <file_path>");
    }
}
