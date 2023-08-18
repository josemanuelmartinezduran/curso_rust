fn main() {
    // Variable que queremos capturar en la closure
    let saludo = "Hola".to_string();

    // Closure sin 'move'
    // Esto toma una referencia a 'saludo'
    let sin_move = || {
        println!("Sin move: {}", saludo);
    };
    
    sin_move(); // Esto funciona porque 'saludo' sigue siendo válido aquí

    // Ahora intentemos usar 'saludo' nuevamente después de la closure 'sin_move'
    println!("Saludo después de sin_move: {}", saludo);

    // Closure con 'move'
    // Esto toma el propietario de 'saludo', moviéndolo a la closure
    let con_move = move || {
        println!("Con move: {}", saludo);
    };

    // Esta llamada funcionaría, pero ya no podemos usar 'saludo' después
    // de esta closure porque ha sido "movido".
    con_move();

    // La siguiente línea causaría un error al descomentarla, porque 'saludo' 
    // ya no es válido después de ser movido a la closure 'con_move'
    // println!("Saludo después de con_move: {}", saludo);
}