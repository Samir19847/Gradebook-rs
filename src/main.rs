use std::io::{self, Write};
use std::fmt;


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
    pub fn pedir_leer()->String{
        println!("Por favor, ingrese las nota final de cada unidad\n(En orden-Separadas por espacios): ");
        let mut entrada:String=String::new();
        io::stdout().flush().expect("Error en el forzamiento del búfer.");
        io::stdin().read_line(&mut entrada).expect("Error en la lectura de la línea");
        entrada.trim().to_string()
    }
    pub fn conversionnumeros(entrada:String)->Result<f64, Notas_invalidas>{
        if entrada.is_empty(){
            return Err(Notas_invalidas::EntradaVacia);
        }
        else{
            let lista:Vec<f64>=entrada
            .split_whitespace()
            .filter_map(| x | x.parse::<f64>().ok())
            .collect();
            if let Some(negativo)=lista.iter().find(| x | **x<0.0){
                return Err(Notas_invalidas::NotaNegativa(*negativo));
            }
            else if let Some(mayor)=lista.iter().find(| x | **x>100.0){
                return Err(Notas_invalidas::NumeroMayorA100(*mayor));
            }
            else if lista.len() > 4 {
            return Err(Notas_invalidas::ErrorMasDe4(lista.len()));
            }
            let total:f64=lista.iter().copied().fold(0.0, | suma, x| suma + x);
                Ok(total)
             
            }
        }

    pub fn comparacion(&self, total:f64)->String{
        let diferencia:f64=self.nota_para_ganar-total;
        if total<self.nota_para_ganar{
            format!("El estudiante perdió el curso de: {}.\nAlcanzó {} puntos de 240, le faltaron {} puntos", self.nombre, total, diferencia)
        }
        else {
            format!("El estudiante ganó el curso de: {}.\nAlcanzó {} puntos de 240, felicidades.", self.nombre, total)
        }
    }

}


enum Notas_invalidas{
    EntradaVacia,
    NumeroMayorA100(f64),
    NotaNegativa(f64),
    ErrorMasDe4(usize),
    
}

impl fmt::Display for Notas_invalidas{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            Notas_invalidas::EntradaVacia => write!(f, "No ingresaste ninguna nota."),
            Notas_invalidas::NotaNegativa(n) => write!(f, "La nota {} no puede ser negativa.", n),
            Notas_invalidas::NumeroMayorA100(n) => write!(f, "La nota {} supera el máximo de 100.", n),
            Notas_invalidas::ErrorMasDe4(cantidad) => write!(f, "Ingresaste {} notas, se esperaban solo 4.", cantidad),
        }
    }
}
struct Estudiantes{
    nombre: String,
    clave: i32,
    cursos: Vec<Cursos>,
}
impl Estudiantes{
    pub fn new(nombre:String, clave: i32)->Estudiantes{
        Estudiantes { nombre, clave, cursos: Vec::new() }
    }
}


fn main() {
    fn leer_entrada()->String{
        let mut entrada:String=String::new();
        io::stdout().flush().expect("Error en el forzamiento del búfer.");
        io::stdin().read_line(&mut entrada).expect("Error en la lectura de la línea");
        entrada.trim().to_string()
    }
    let mut estudiantes:Vec<Estudiantes>=Vec::new();
    let mut siguiente_clave: i32 = 1;
    loop {
    println!("========================================");
    println!("===                                  ===");
    println!("===              LIBRO               ===");
    println!("===                DE                ===");
    println!("===          CALIFICACIONES          ===");
    println!("===                                  ===");
    println!("========================================");
    println!();
    println!("========================================");
    println!("===          MENÚ PRINCIPAL          ===");
    println!("========================================");
    println!("===  1. Agregar estudiante           ===");
    println!("===  2. Agregar curso                ===");
    println!("===  3. Agregar/actualizar nota      ===");
    println!("===  4. Ver boleta                   ===");
    println!("===  5. Salir                        ===");
    println!("========================================");
    let mut opcion:i32=loop{
        print!("Por favor, ingrese una opción del menú: ");
        match leer_entrada().parse(){
        Ok(v)=>break v,
        Err(_)=>{
            println!("Error: Tipo de dato incorrecto, por favor, ingrese un número...");
            println!();
        }    
        }
    };
    println!();
    match opcion{
        1=>{
        let mut cantidad:i32=loop{
            print!("Por favor, la cantidad de estudiantes que desea agregar: ");
            match leer_entrada().parse(){
                Ok(v)=>break v,
                Err(_)=>{
                    println!("Error: Tipo de dato incorrecto, por favor, ingrese un número...");    
                    println!();
                }
            }
        };
        println!();
        for z in 1..=cantidad{
            print!("Por favor, ingrese el nombre completo del estudiante {z}: ");
            let mut nombre=leer_entrada();
            let estudiantess=Estudiantes::new(nombre, siguiente_clave);
            estudiantes.push(estudiantess);
            siguiente_clave+=1;

        }
        println!();
        println!("¡Estudiantes registrados correctamente!");
        println!();
    },
        2=>{
            if estudiantes.len()<=0{
                println!("Error: no se ha registrado ningún estudiante aún.\nSe necesita por lo menos tener agregado a un esutdiante para asignar cursos...");
                println!();
            }
            else{
                println!("========================================");
                println!("===       LISTA DE ESTUDIANTES       ===");
                println!("========================================");
                println!();
                for z in &estudiantes{
                println!("Clave: {}. Nombre: {}",z.clave, z.nombre);
                }
                println!();
                let mut opcion_estudiante: i32=loop{
                    print!("Por favor, ingrese la clave del estudiante: ");
                    match leer_entrada().parse(){
                        Ok(v)=>break v,
                        Err(_)=>{
                            println!("Error: Tipo de dato incorrecto, por favor, ingrese un número...");
                            println!();
                        }
                    }
                };
                if let Some(estudiante) = estudiantes.iter_mut().find(|x| x.clave == opcion_estudiante) {
                    let materias: i32 = loop {
                        print!("Por favor, ingrese la cantidad de cursos de ese estudiante: ");
                        match leer_entrada().parse() {
                            Ok(v) => break v,
                            Err(_) => {
                                println!("Error: Tipo de dato incorrecto, por favor, ingrese un número...");
                                println!();
                            }
                        }
                    };
                for x in 1..=materias {
                    print!("Por favor, ingrese el curso {x}: ");
                    let nombre = leer_entrada();
                    let curso = Cursos::new(nombre);
                    estudiante.cursos.push(curso);
                }
                println!();
                println!("¡Cursos agregados correctamente!");
                } else {
                    println!("No se encontró ningún estudiante con esa clave.");
                }
            }
        },
        3=>{
            if estudiantes.len()<=0{
                println!("Error: no se ha registrado ningún estudiante aún.\nSe necesita por lo menos tener agregado a un esutdiante para asignar cursos...");
                println!();
            }
        },
        4=>{
            if estudiantes.len()<=0{
                println!("Error: no se ha registrado ningún estudiante aún.\nSe necesita por lo menos tener agregado a un esutdiante para asignar cursos...");
                println!();
            }
        }
        5=>{
             println!("Cerrando programa..."); 
        },
        _=>{
            println!("Opción inválida...\nPor favor, ingrese una opción del menú: ");
            println!();
        },
    }
}
    
    
}