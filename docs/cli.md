# CLI Usage

The `html-ussd` CLI lets you **launch**, **simulate**, and **navigate** through USSD flows defined in HTML files.

You can run simulations either **locally** (with HTML files) or **remotely** (with a URL).

## Installation

```bash
cargo install html-ussd
```

## Basic Usage

```bash
html-ussd --main <MAIN>
```

- `<MAIN>` is mandatory.
- It can be a local file (e.g., `index.html`) or a remote URL (e.g., `http://localhost:8888`).

## Options

| Option            | Description                                             | Example                              |
| :---------------- | :------------------------------------------------------ | :----------------------------------- |
| `-m`, `--main`    | Main entry file or URL.                                 | `html-ussd -m index.html`            |
| `-l`, `--lang`    | Language to use (en, fr, mg).                           | `html-ussd -m index.html -l fr`      |
| `--no-cache`      | Disable cache. Forces reloading of HTML files or pages. | `html-ussd -m index.html --no-cache` |
| `-h`, `--help`    | Show help information.                                  | `html-ussd --help`                   |
| `-v`, `--version` | Show current version.                                   | `html-ussd --version`                |

## Examples

### Run from a local HTML file

```bash
html-ussd --main index.html
```

### Run from a remote URL

```bash
html-ussd --main http://localhost:8888
```

### Launch in French

```bash
html-ussd --main ./examples/index.html --lang fr
```

### Disable caching

```bash
html-ussd --main http://localhost:8888 --no-cache
```

## Special Key Shortcuts
- Press `0` → Go back to the previous screen.
- Press `00` → Return to the home screen

## Tips
- **File not found** → Ensure the file path is correct and the file exists.
- **Unsupported tags** → Make sure your HTML follows the [HTML Language Rules](./html-rules).
- **Connection error** → If using a URL, ensure the server is running and reachable.


