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
    pub fn constructor(nombre:String)->Cursos{
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

    
}



struct Estudiantes{
    nombre: String,
    cursos:Vec<Cursos>,
}


fn main() {

    
    println!("Hello, world!");
}
