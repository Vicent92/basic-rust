use std::io;

fn main() {
    // VARIABLES
    // let edad = 32;
    // let nombre = "Vicente";
    // println!("Me llamo {nombre} y tengo {edad} años");

    // RECIBIR DATOS DEL USUARIO
    // println!("¿Cuál es tu nombre?");

    // let mut nombre = String::new();
    // io::stdin().read_line(&mut nombre).unwrap();

    // // Eliminar el salto de línea al final
    // nombre = nombre.trim().to_string();

    // // opteniendo edad
    // println!("¿Cuál es tu edad?");
    // let mut edad = String::new();
    // io::stdin().read_line(&mut edad).unwrap();

    // // Convertir a número
    // let edad: i32 = edad.trim().parse().unwrap();

    // println!("Hola {nombre}, bienvenido al curso de Rust");

    // println!("Podemos ver que tienes {edad} años");

    // CONDICIONALES
    // let edad_base = 18;
    // println!("Si quieres pasar, tines que ser mayor de edad");
    // println!("¿Cuál es tu edad?");

    // let mut edad = String::new();
    // io::stdin().read_line(&mut edad).unwrap();

    // let edad_int: u8 = edad.trim().parse().unwrap();

    // if edad_int >= edad_base {
    //     println!("Puedes pasar");
    // } else {
    //     println!("No puedes pasar");
    // }

    //LOOP
    let num1 = 10;
    let num2 = 20;

    let resultado = num1 + num2;

    loop {
        println!("Cual es el resultado de {num1} + {num2}?");

        let mut respuesta = String::new();
        io::stdin().read_line(&mut respuesta).unwrap();

        let respuesta: i32 = respuesta.trim().parse().unwrap();

        if respuesta == resultado {
            println!("Correcto");
            break;
        } else {
            println!("Incorrecto, vuelve a intentarlo");
        }
    }
}
