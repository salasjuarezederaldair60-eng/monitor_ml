# Monitor_ml

Monitor de precios en Rust que vigila productos y avisa por Telegram apenas bajan del umbral que definas. Pensado para ser configurable: no depende de una sola tienda, sino de lo que cargues en el archivo de configuración.

## Características

- Chequeo periódico de precios (intervalo configurable)
- Notificaciones instantáneas vía bot de Telegram
- Configuración simple mediante archivo TOML, sin tocar código
- Soporte para múltiples productos a la vez

## Requisitos

- [Rust](https://www.rust-lang.org/tools/install) y Cargo instalados
- Un bot de Telegram propio (token) y tu chat ID
  - Podés crear un bot hablando con [@BotFather](https://t.me/BotFather) en Telegram
  - Para obtener tu chat ID, podés usar [@userinfobot](https://t.me/userinfobot)

## Instalación

```bash
git clone https://github.com/TU_USUARIO/Monitor_ml.git
cd Monitor_ml
cargo build --release
```

## Configuración

Creá un archivo `config.toml` en la raíz del proyecto con esta estructura:

```toml
[[productos]]
url = "https://ejemplo.com/producto/12345"
precio_objetivo = 199.99
intervalo_segundos = 300

[telegram]
token = "TU_TOKEN_DE_BOT"
chat_id = "TU_CHAT_ID"
```

Podés agregar tantos bloques `[[productos]]` como productos quieras monitorear; cada uno puede tener su propio precio objetivo e intervalo de chequeo.

## Uso

```bash
cargo run --release
```

El programa empieza a chequear los precios según el intervalo configurado y envía un mensaje al chat de Telegram indicado apenas alguno baja del umbral definido.

## Stack técnico

- Rust
- Integración de APIs REST
- Bot de Telegram (Bot API)
- Configuración vía TOML

## Casos de uso

- Afiliados que necesitan detectar bajadas de precio para generar comisiones
- Compradores esperando una oferta puntual
- Negocios que quieren vigilar precios de la competencia

## Estado del proyecto

Funcional en su versión básica. Próximas mejoras planeadas: soporte para más fuentes de datos, panel de configuración más simple, historial de precios.
