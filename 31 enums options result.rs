// 1. Enum personalizado: Representando diferentes animales
enum Animal {
    Dog,
    Cat,
    Bird,
}

impl Animal {
    fn speak(&self) {
        match self {
            Animal::Dog => println!("Woof!"),
            Animal::Cat => println!("Meow!"),
            Animal::Bird => println!("Tweet!"),
        }
    }
}

// Función que intenta convertir una cadena en un Animal
fn string_to_animal(s: &str) -> Option<Animal> {
    match s {
        "dog" => Some(Animal::Dog),
        "cat" => Some(Animal::Cat),
        "bird" => Some(Animal::Bird),
        _ => None, // Para cualquier otra entrada, devuelve None
    }
}

// Función que intenta dividir dos números
fn divide(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        Err("Division by zero!")
    } else {
        Ok(a / b)
    }
}

fn main() {
    // Usando el enum personalizado
    let dog = Animal::Dog;
    dog.speak();

    // Usando Option
    if let Some(animal) = string_to_animal("cat") {
        animal.speak();
    } else {
        println!("Animal not recognized!");
    }

    // Usando Result
    match divide(4.0, 2.0) {
        Ok(result) => println!("Result is: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}