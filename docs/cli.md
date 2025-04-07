# CLI Usage

The `html-ussd` CLI lets you lets you simulate USSD flows defined in HTML files.

You can use it with **local files** or load screens from a **remote server**.

## Installation

```bash
cargo install html-ussd
```

## Basic Usage

```bash
html-ussd --main <MAIN>
```

- `<MAIN>` : required — the entry point — can be a local file (eg: `index.html`) or a remote URL (`http://localhost:8888`)

## Options

| Option                            | Description                                             |
| :-------------------------------- | :------------------------------------------------------ |
| `-m`, `--main`                    | Main entry file or URL.                                 |
| `-l`, `--lang`                    | Language to use (`en`, `fr`, `mg`).                     |
| [`--no-cache`](./how-cache-works) | Disable cache. Forces reloading of HTML files or pages. |
| `-h`, `--help`                    | Show help information.                                  |
| `-v`, `--version`                 | Show current version.                                   |

## Examples

### From local file

```bash
html-ussd --main index.html
```

### From remote serverL

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
