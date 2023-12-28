fn main() {
    // println!("Hello, Jose!");

    // let texto = String::from("Hello, Jose!");

    // let texto2 = texto;

    // let mut valor = 50;
    // let referencia = &mut valor;

    std::println!("Hello, Jose!");

    const NAME_COMPANY: &str = "MangaTrix";
    let name_company = "MangaTrix";

    println!("nome empresa : {}", NAME_COMPANY);
    println!("nome da variavel : {}", name_company);

    let mut name_pessoa = "Jose";
    println!("nome pessoa : {}", name_pessoa);
    name_pessoa = "treinamento";
    println!("nome curso : {}", name_pessoa);
}
