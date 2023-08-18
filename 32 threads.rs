use std::thread;

fn main() {
    // Una lista para guardar los hilos creados
    let mut hilos = vec![];

    for i in 1..=5 {
        // Crear un hilo usando una closure
        let hilo = thread::spawn(move || {
            println!("Soy el hilo número {}", i);
        });

        // Agregar el hilo a la lista
        hilos.push(hilo);
    }

    // Esperar a que todos los hilos terminen
    for hilo in hilos {
        hilo.join().unwrap();
    }

    println!("Todos los hilos han terminado.");
}