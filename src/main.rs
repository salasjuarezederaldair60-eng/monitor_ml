use serde::{Deserialize, Serialize};
use std::fs;
use std::thread;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Producto {
    url: String,
    nombre: String,
    selector: String,
    precio_anterior: f64,
}

fn telegram_token() -> String { std::env::var("TELEGRAM_TOKEN").expect("Falta la variable TELEGRAM_TOKEN") }
fn telegram_chat_id() -> String { std::env::var("TELEGRAM_CHAT_ID").expect("Falta la variable TELEGRAM_CHAT_ID") }

fn cargar_productos() -> Vec<Producto> {
    let contenido = fs::read_to_string("products.json")
        .expect("No se pudo leer products.json");
    serde_json::from_str(&contenido).expect("JSON invalido")
}

fn guardar_productos(productos: &Vec<Producto>) {
    let json = serde_json::to_string_pretty(productos).unwrap();
    fs::write("products.json", json).expect("No se pudo guardar products.json");
}

fn limpiar_numero(texto: &str) -> Option<f64> {
    let limpio: String = texto
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '.' || *c == ',')
        .collect();
    let limpio = limpio.replace(',', "");
    limpio.parse::<f64>().ok()
}

fn obtener_precio(url: &str, selector_texto: &str) -> Option<f64> {
    let cliente = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .build()
        .ok()?;

    let respuesta = cliente.get(url).send().ok()?;
    let html = respuesta.text().ok()?;
    let documento = scraper::Html::parse_document(&html);

    // Intenta cada selector separado por comas hasta encontrar uno que funcione
    for selector_individual in selector_texto.split(',') {
        let selector_limpio = selector_individual.trim();
        if let Ok(selector) = scraper::Selector::parse(selector_limpio) {
            if let Some(elemento) = documento.select(&selector).next() {
                let texto: String = elemento.text().collect();
                if let Some(precio) = limpiar_numero(&texto) {
                    return Some(precio);
                }
            }
        }
    }
    None
}

fn enviar_telegram(mensaje: &str) {
    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage",
        telegram_token()
    );
    let cliente = reqwest::blocking::Client::new();
    let _ = cliente
        .post(&url)
        .form(&[("chat_id", telegram_chat_id().as_str()), ("text", mensaje)])
        .send();
}

fn main() {
    println!("Monitor de precios configurable iniciado. Revisando cada 30 minutos...");

    loop {
        let mut productos = cargar_productos();

        for producto in productos.iter_mut() {
            match obtener_precio(&producto.url, &producto.selector) {
                Some(precio_actual) => {
                    println!("{}: {}", producto.nombre, precio_actual);

                    if producto.precio_anterior > 0.0 && precio_actual < producto.precio_anterior {
                        let mensaje = format!(
                            "Bajo de precio!\n{}\nAntes: {}\nAhora: {}\n{}",
                            producto.nombre, producto.precio_anterior, precio_actual, producto.url
                        );
                        enviar_telegram(&mensaje);
                        println!("Alerta enviada a Telegram.");
                    }

                    producto.precio_anterior = precio_actual;
                }
                None => println!("No se pudo obtener el precio de {} (revisar selector CSS)", producto.nombre),
            }
        }

        guardar_productos(&productos);

        println!("Esperando 30 minutos para la proxima revision...");
        thread::sleep(Duration::from_secs(30 * 60));
    }
}
