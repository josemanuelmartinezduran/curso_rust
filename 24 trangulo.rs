struct Triangulo{
    co: f32,
    ca: f32,
    h: f32
}

impl Triangulo{
    fn new(co:f32, ca:f32, h:f32 ) -> Self{
        Self{co, ca, h}
    }
    
    fn pitagoras(t:&mut Triangulo) -> (){
        if t.co == 0.0{
            let res = (t.h * t.h) - (t.ca * t.ca);
            t.co = res.sqrt();
        } else if t.ca == 0.0 {
            let res = (t.h * t.h) - (t.ca * t.ca) as f32;
            t.ca = res.sqrt() as f32;
        } else if t.h == 0.0 {
            let res = (t.h * t.h) - (t.ca * t.ca) as f32;
            t.h = res.sqrt() as f32;
        }
    }
}

fn main(){
    let mut t = Triangulo::new(2.0, 0.0, 4.0);
    Triangulo::pitagoras(&mut t);
    println!("El triangulo está resuelto? CO: {} CA:{} H:{}", t.co, t.ca, t.h);
}