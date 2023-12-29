fn maior_valor(a: i32, b: i32) -> i32 {
    {
        if a > b {
            a
        } else {
            b
        }
    }
}

fn saudacao(naome: &str, saudacao_personalziada: Option<&str>) {
    match saudacao_personalziada {
        Some(s) => println!("{} {}", s, naome),
        None => println!("Ola {}", naome),
    }
}

fn main() {
    let maior = maior_valor(10, 20);
    println!("Maior valor : {}", maior);

    let valor = 32;
    println!("Valor : {}", valor);

    saudacao("Jose", None);

    saudacao("José Augusto", Some("Ola"));
}
