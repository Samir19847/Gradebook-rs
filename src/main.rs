use std::io::{self, Write};

struct Unidad{
    letra:String,
    nota:f64,
}

struct Cursos{
    nombre:String,
    unidades:[Unidad; 4],
    nota_para_ganar:f64,
}

impl Cursos{
    pub fn new(nombre:String)->Cursos{
        Cursos {
            nombre,
            unidades: [
                Unidad { letra: String::from("I"), nota: 0.0 },
                Unidad { letra: String::from("II"), nota: 0.0 },
                Unidad { letra: String::from("III"), nota: 0.0 },
                Unidad { letra: String::from("IV"), nota: 0.0 },
            ],
            nota_para_ganar: 240.0,
        }
    }
    pub fn actualizar_notas(&mut self, numero_unidad:String, nota:f64){
        if let Some(unidad) = self.unidades.iter_mut().find(|u| *u.letra == numero_unidad) 
        {
        unidad.nota = nota;
        }
    }
    pub fn sumar_notas(&self, entrada:String){
        println!("Por favor, ingrese las notas finales de cada unidades\n(En orden-Separadas por espacios): ");
        let lista:Vec<f64>=entrada
            .split_whitespace()
            .filter_map(| x | x.parse::<f64>().ok())
            .collect();
        let total:f64=lista.iter().copied().fold(0.0, | suma, x| suma + x);
        let diferencia:f64=self.nota_para_ganar-total;

        if total<self.nota_para_ganar{
            println!("El estudiante perdió el curso de: {}.\nAlcanzó {} puntos de 240, le faltaron {} puntos", self.nombre, total, diferencia);
        }
        else{
             println!("El estudiante ganó el curso de: {}.\nAlcanzó {} puntos de 240, felicidades.", self.nombre, total);
        }
    
    }

    
}



struct Estudiantes{
    nombre: String,
    cursos:Vec<Cursos>,
}


fn main() {
    fn leer_entrada()->String{
        let mut entrada:String=String::new();
        io::stdout().flush().expect("Error en el forzamiento del búfer.");
        io::stdin().read_line(&mut entrada).expect("Error en la lectura de la línea");
        entrada.trim().to_string()
    }
    let entrada=leer_entrada();
    println!("Hello, world!");
}
