# html-ussd

Simulate USSD experiences using plain HTML and run them directly in your terminal.

## Features

- Write USSD screens using simple HTML with few rules
- Navigate menus and handle forms in the terminal
- Launch flows from local files or remote URLs
- Built-in multilingual support (EN, FR, MG 🇲🇬)
- Fully extendable: plug your own parser (e.g. JSON, XML) or renderer

## Demo

![Demo](./docs/demo.gif)

## Getting Started

To get started quickly, install the CLI:

### Installation

- **Linux/macOS/WSL**

```bash
curl -sSL https://raw.githubusercontent.com/heryfitiavana22/html-ussd/main/scripts/install.sh | bash
```

- **Windows** (run as administrator)

```bash
curl -o "%TEMP%\install.bat" https://raw.githubusercontent.com/heryfitiavana22/html-ussd/main/scripts/install.bat && "%TEMP%\install.bat"
```

Then run from

- a remote server (this is live page)

```bash
html-ussd run --main "https://html-ussd-example.onrender.com/main-page"
```

- a local HTML file

```bash
html-ussd run --main index.html
```

## Contributing

Contributions and feedback are welcome.
If you want to implement a custom adapter (e.g., JSON, XML), check out the [TagAdapter](https://heryfitiavana22.github.io/html-ussd/supporting-custom-input-formats.html) or [Renderer](https://heryfitiavana22.github.io/html-ussd/custom-renderer.html) section in the docs.
