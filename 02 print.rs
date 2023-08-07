fn main() {
    // Uso de {}
    println!("{} days", 31);

    // {} Posicionales
    println!("{0}, es el {1}. {0}, Es mi {0}?", "Juan", "Jefe");

    // {} definidos al vulo
    println!("{subject} {verb} {object}",
             object="La tina",
             subject="Anita",
             verb="Lava");

    //Diferentes tipo de formato
    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
    println!("Base 16 (hexadecimal): {:X}", 69420); // 10F2C

    // Texto justificado
    println!("{number:>5}", number=1);

    // Padding
    println!("{number:0>5}", number=1); // 00001
    // Padding derecho.
    println!("{number:0<5}", number=1); // 10000

    // Named arguments con  `$`.
    println!("{number:0>width$}", number=1, width=5);

    // Ayudame Rust!!.
    println!("My name is {0}, {1} {0}", "Bond");

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    // Deja de compilar
}