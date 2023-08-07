fn main() {
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