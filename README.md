# Monitor_ml

Price monitor written in Rust that tracks products and sends a Telegram alert when their price drops. It is configurable: it does not depend on a single store, only on the URLs and CSS selectors you define in `products.json`.

## Features

- Periodic price checks (every 30 minutes)
- Telegram alert with the previous price, current price, and link
- Multiple products at once, defined in a JSON file
- Saves the last seen price in the same file

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) and Cargo
- A Telegram bot (token) and your chat ID
  - Create the bot with [@BotFather](https://t.me/BotFather)
  - Get your chat ID with [@userinfobot](https://t.me/userinfobot)

## Installation

```bash
git clone https://github.com/salasjuarezederaldair60-eng/monitor_ml.git
cd monitor_ml
cargo build --release
```

## Configuration

1. Set the token and chat ID as environment variables (they are not stored in any file):

```powershell
# Windows (PowerShell)
$env:TELEGRAM_TOKEN="your_token"
$env:TELEGRAM_CHAT_ID="your_chat_id"
```

```bash
# Linux / macOS
export TELEGRAM_TOKEN="your_token"
export TELEGRAM_CHAT_ID="your_chat_id"
```

2. Create `products.json` in the project root:

```json
[
  {
    "url": "https://example.com/product/1",
    "nombre": "My product",
    "selector": ".price",
    "precio_anterior": 0.0
  }
]
```

All four fields are required:

- `url`: product page
- `nombre`: product name shown in the alert
- `selector`: CSS selector of the element that contains the price
- `precio_anterior`: starts at `0.0`; the program updates it automatically

## Usage

```bash
cargo run --release
```

On the first check, it only saves the price. On later checks, if the price is lower than the previous one, it sends an alert to Telegram.

## Status

Working in its basic version. Planned improvements: support for more data sources, a simpler configuration panel, and price history.
