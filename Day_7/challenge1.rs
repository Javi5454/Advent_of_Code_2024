use std::env;
use std::fs;
fn check_possible_operation(
    target: isize,
    numbers: &Vec<isize>,
    current_value: isize,
    index: usize,
) -> bool {
    //Si hemos procesado todos los numeros, verificamos
    if index == numbers.len() {
        return current_value == target;
    }

    //Intentamos sumar o multiplicar el siguiente numero
    return check_possible_operation(target, numbers, current_value + numbers[index], index + 1)
        || check_possible_operation(target, numbers, current_value * numbers[index], index + 1);
}

fn main() {
    let args: Vec<String> = env::args().collect(); //Cogemos los argumentos

    if args.len() == 2 {
        let file_path = &args[1];

        let input = fs::read_to_string(file_path).expect("Error leyendo el archivo");

        let mut result = 0;

        for line in input.lines() {
            let expected_result: isize = line.split(':').collect::<Vec<_>>()[0].parse().unwrap();

            let operators: Vec<isize> = line.split(": ").collect::<Vec<_>>()[1]
                .split_whitespace()
                .map(|num| num.parse::<isize>().unwrap())
                .collect();

            if check_possible_operation(expected_result, &operators, operators[0], 1) {
                result += expected_result;
            }
        }

        println!("Total: {}", result);
    } else {
        println!("La estructura de llamada es ./challenge1 <file_path>")
    }
}
