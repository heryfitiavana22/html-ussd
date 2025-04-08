# html-ussd

Simulate USSD experiences using plain HTML and run them directly in your terminal.

## Features

- Write USSD screens using simple HTML with few rules
- Navigate menus and handle forms in the terminal
- Launch flows from local files or remote URLs
- Built-in multilingual support (EN, FR, MG 🇲🇬)
- Fully extendable: plug your own parser (e.g. JSON, XML) or renderer

## Getting Started

To get started quickly, install the CLI:

### Installation

- **Linux/macOS/WSL**

```bash
curl -sSL https://raw.githubusercontent.com/heryfitiavana22/html-ussd/main/scripts/install.sh | bash
```

- **Windows(powershell)**

```bash
irm https://raw.githubusercontent.com/heryfitiavana22/html-ussd/main/scripts/install.bat | iex
```

Then run from

- a remote server

```bash
html-ussd --main http://localhost:8888/
```

- a local HTML file

```bash
html-ussd --main index.html
```

## Contributing

Contributions and feedback are welcome.
If you want to implement a custom adapter (e.g., JSON, XML), check out the [TagAdapter](http://localhost:8888/) or [Renderer](http://localhost:8888/) section in the docs.
