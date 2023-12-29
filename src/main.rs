fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

fn main() {
    // println!("Hello, Jose!");

    // let texto = String::from("Hello, Jose!");

    // let texto2 = texto;

    // let mut valor = 50;
    // let referencia = &mut valor;

    std::println!("Hello, Jose!");

    const NAME_COMPANY: &str = "MangaTrix";
    let name_company = "Manga-Trix";

    println!("nome empresa : {}", NAME_COMPANY);
    println!("nome da variavel : {} , {} ", name_company, NAME_COMPANY);

    let mut name_pessoa = "Jose";
    println!(
        "nome pessoa : {} , tipo {}",
        name_pessoa,
        type_of(name_pessoa)
    );
    name_pessoa = "treinamento";
    println!("nome curso : {}", name_pessoa);

    let inteiro = 10;

    let int_to_float = inteiro as f32;

    println!(
        "Valor da variavel inteiro {} e flutuante {} , tipo inteiro {} , tipo flutuante {}",
        inteiro,
        int_to_float,
        type_of(inteiro),
        type_of(int_to_float)
    );

    let float = 2.5;

    let float_to_int = float as i32;

    println!(
        "Valor da variavel float {} e inteiro {}, tipo float {} , tipo inteiro {}",
        float,
        float_to_int,
        type_of(float),
        type_of(float_to_int)
    );

    let int_to_string = inteiro.to_string();

    println!(
        "valor int_to_String {} , tipe {}",
        int_to_string,
        type_of(&int_to_string)
    );

    let string = "42";

    let string_to_int = string.parse::<i64>().unwrap();

    println!(
        "valor string_to_int {} , tipo {}",
        string_to_int,
        type_of(string_to_int)
    );
}
