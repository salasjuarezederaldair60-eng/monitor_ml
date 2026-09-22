use std::fs;

fn main() {
    let cliente = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .build()
        .unwrap();

    let url = "https://www.mercadolibre.com.mx/p/MLM41318734";
    let respuesta = cliente.get(url).send().unwrap();
    let status = respuesta.status();
    let html = respuesta.text().unwrap();

    println!("Status: {}", status);
    println!("Tamano del HTML: {} caracteres", html.len());

    fs::write("debug.html", &html).unwrap();
    println!("HTML guardado en debug.html para revisar");
}
