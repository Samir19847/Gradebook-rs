struct Unidad{
    numero:i32,
    nota:f64,
}

struct Cursos{
    nombre:String,
    unidades:[Unidad; 4],
    nota_para_ganar:f64,
}

struct Estudiantes{
    nombre: String,
    cursos:Vec<Cursos>,
}


fn main() {
    println!("Hello, world!");
}
