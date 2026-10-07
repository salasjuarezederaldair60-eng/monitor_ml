# Monitor_ml

Monitor de precios en Rust que vigila productos y avisa por Telegram cuando bajan de precio. Es configurable: no depende de una sola tienda, sino de las URLs y selectores CSS que cargues en `products.json`.

## Características

- Revisión periódica de precios (cada 30 minutos)
- Aviso por Telegram con precio anterior, precio actual y enlace
- Varios productos a la vez, definidos en un archivo JSON
- Guarda el último precio visto en el mismo archivo

## Requisitos

- [Rust](https://www.rust-lang.org/tools/install) y Cargo
- Un bot de Telegram (token) y tu chat ID
  - Crea el bot con [@BotFather](https://t.me/BotFather)
  - Obtén tu chat ID con [@userinfobot](https://t.me/userinfobot)

## Instalación

```bash
git clone https://github.com/salasjuarezederaldair60-eng/Monitor_ml.git
cd Monitor_ml
cargo build --release
```

## Configuración

1. Define el token y el chat ID como variables de entorno (no van en ningún archivo):

```powershell
# Windows (PowerShell)
$env:TELEGRAM_TOKEN="tu_token"
$env:TELEGRAM_CHAT_ID="tu_chat_id"
```

```bash
# Linux / macOS
export TELEGRAM_TOKEN="tu_token"
export TELEGRAM_CHAT_ID="tu_chat_id"
```

2. Crea `products.json` en la raíz del proyecto:

```json
[
  {
    "url": "https://ejemplo.com/producto/12345",
    "nombre": "Mi producto",
    "selector": ".precio",
    "precio_anterior": 0.0
  }
]
```

Los cuatro campos son obligatorios:

- `url`: página del producto
- `nombre`: cómo aparecerá en la alerta
- `selector`: selector CSS del elemento que contiene el precio
- `precio_anterior`: empieza en `0.0`; el programa lo actualiza solo

## Uso

```bash
cargo run --release
```

En la primera revisión solo guarda el precio. En las siguientes, si el precio es menor que el anterior, envía la alerta a Telegram.

## Estado

Funcional en su versión básica. Próximas mejoras planeadas: soporte para más fuentes de datos, panel de configuración más simple, historial de precios.