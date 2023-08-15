struct Rect{
    base:f32,
    altura:f32,
}

impl Rect{
    fn new(base:f32, altura:f32) -> Self{
        if base == 0.0 && altura == 0.0 {
            Self{base: 4.5, altura: 6.2}    
        } else {
            Self{base, altura}
        }
        
    }
    
    fn area(rect:&Rect) -> f32{
        rect.base * rect.altura
    }
    
    fn perimetro(rect:&Rect) -> f32{
        2.00 * (rect.base + rect.altura)
    }
    
    fn imprime(rect: &Rect) -> (){
        println!("La base es {} y la altura es {} el area es {} y el perimetro es {}", 
        rect.base, 
        rect.altura, 
        Rect::area(&rect), 
        Rect::perimetro(&rect));
    }
}

fn main() {
    Rect::imprime(&Rect::new(0.0, 0.0));
    Rect::imprime(&Rect::new(10.0, 5.6));
}

