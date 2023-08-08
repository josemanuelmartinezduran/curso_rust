fn main() {
    /* Tipos escalares:

Enteros:

i8, i16, i32, i64, i128: Enteros con signo de 8, 16, 32, 64 y 128 bits respectivamente.
u8, u16, u32, u64, u128: Enteros sin signo de 8, 16, 32, 64 y 128 bits respectivamente.
isize, usize: Enteros con y sin signo respectivamente, cuyo tamaño depende del tipo de arquitectura de la máquina (usualmente 32 o 64 bits).
Punto flotante:

f32: Punto flotante de precisión simple.
f64: Punto flotante de precisión doble.
Carácter:

char: Representa un carácter Unicode. Se especifica usando comillas simples, como 'a'.
Booleano:

bool: Solo tiene dos valores posibles, true o false. */
    // Variables con tipo
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Regular annotation
    let an_integer   = 5i32; // Suffix annotation

    // Tipo por default
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    // Variable mutable
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // Tipo inferido
    let mut inferred_type = 12; // Type i64 is inferred from another line.
    inferred_type = 4294967296i64;

    // Se puede cmabiar el tipo?.
    mutable = true;

    // shadowing.
    let mutable = true;
}