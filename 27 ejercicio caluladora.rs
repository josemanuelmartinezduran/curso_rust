trait Opera{
    fn opera(&self) ->f32;
}

struct Suma{
    sumandos: [f32; 6]
}

impl Suma {
    fn new(sumandos: [f32; 6])->Self{
        Self{sumandos}
    }
}

impl Opera for Suma {
    fn opera(&self) -> f32 {
        let mut res:f32 = 0.0f32;
        for i in self.sumandos {
            res += i;
        }
        res
    }
}

struct Resta{
  minuendo: f32,
  sustraendo: f32
}

impl Resta{
    fn new(minuendo: f32, sustraendo: f32) -> Self {
        Self{minuendo, sustraendo}
    }
}

impl Opera for Resta {
    fn opera(&self) -> f32{
        self.minuendo - self.sustraendo
    }   
}

fn main(){
    let s = Suma::new([2.0f32, 5.6f32, 1.00, 0.00, 0.00, 0.00]);
    println!("La suma es {}", s.opera());
    let r = Resta::new(16.24, 2.5);
    println!("La resta es {}", r.opera());
}