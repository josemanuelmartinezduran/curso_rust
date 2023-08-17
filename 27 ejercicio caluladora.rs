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

struct Multiplicacion{
    factores: Vec<f32>
}

impl Multiplicacion{
    fn new(factores: Vec<f32>) -> Self{
        Self{factores}
    }
}

impl Opera for Multiplicacion {
    fn opera(&self) -> f32{
        let mut mult = 1.0f32;
        for &i in &self.factores{
            mult = mult * i;
        }
        mult
    }
}

struct Division{
    dividendo:f32,
    divisor:f32
}

impl Division{
    fn new(dividendo:f32, divisor:f32)->Self{
        Self{dividendo, divisor}
    }
}

impl Opera for Division{
    fn opera(&self) -> f32{
        if self.divisor == 0.0 {
            0.0f32
        } else {
            self.dividendo/self.divisor
        }
    }
}



fn main(){
    let s = Suma::new([2.0f32, 5.6f32, 1.00, 0.00, 0.00, 0.00]);
    println!("La suma es {}", s.opera());
    let r = Resta::new(16.24, 2.5);
    println!("La resta es {}", r.opera());
    let m = Multiplicacion::new(vec![5.0f32, 4.0f32, 2.0f32]);
    println!("La multiplicacion es {}", m.opera());
    let d = Division::new(4.5f32, 2.00f32);
    println!("La division es {}", d.opera());
}