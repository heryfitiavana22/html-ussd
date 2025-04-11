# CLI Usage

The `html-ussd` CLI lets you lets you simulate USSD flows defined in HTML files.

You can use it with **local files** or load screens from a **remote server**.

## Basic Usage

```bash
html-ussd run --main <MAIN>
```

- `<MAIN>` : required — the entry point — can be a local file (eg: `index.html`) or a remote URL (`http://localhost:8888`)

## Options

| Option                            | Description                                                            |
| :-------------------------------- | :--------------------------------------------------------------------- |
| `-m`, `--main`                    | Main entry file or URL.                                                |
| `-l`, `--lang`                    | Language to use (`en`, `fr`, `mg`).                                    |
| `--phone`                         | Phone number to use for the session. Defaults to `0312345678`.         |
| `--query`                         | Extra query parameters to include in requests, e.g. `id=1,source=web`. |
| `--header`                        | Custom HTTP headers for remote requests, e.g. `X-Custom=Value`.        |
| `--access-token`                  | Shortcut to add a Bearer token in the `Authorization` header.          |
| [`--no-cache`](./how-cache-works) | Disable cache. Forces reloading of HTML files or pages.                |
| `-h`, `--help`                    | Show help information.                                                 |
| `-v`, `--version`                 | Show current version.                                                  |

> [!TIP]
> `--query`, `--header`, and `--access-token` are especially useful when working with authenticated APIs or dynamic content. [Learn more](./security).

## Phone Number

the `phone` number will be included in every remote request as a query parameter:

```bash
html-ussd run --main "https://html-ussd-example.onrender.com/main-page" --phone 0341234567
```

Resulting request: `https://html-ussd-example.onrender.com/main-page?phone=0341234567`

This helps simulate a real user session and can be used by the backend for user identification.

> [!IMPORTANT] 
> `?phone=0341234567` will be sent with every request.

## Examples

### From local file

```bash
html-ussd run --main index.html
```

### From remote server

```bash
html-ussd run --main "https://html-ussd-example.onrender.com/main-page"
```

### With custom phone number and query parameters

```bash
html-ussd run --main "https://html-ussd-example.onrender.com/main-page" --phone "0320001111" --query "user_id=42,mode=preview"
```

### Using headers and access token

```bash
html-ussd run --main "https://html-ussd-example.onrender.com/main-page" --access-token "xyz"
```

Which is equivalent to:

```bash
html-ussd run --main "https://html-ussd-example.onrender.com/main-page" --header "Authorization=Bearer my-secret-token"
```

### In French and without cache

```bash
html-ussd run --main ./examples/index.html --lang fr --no-cache
```

## Special Key Shortcuts

- Press `0` → Go back to the previous screen.
- Press `00` → Return to the home screen

## Tips

- **File not found** → Ensure the file path is correct and the file exists.
- **Unsupported tags** → Make sure your HTML follows the [HTML Language Rules](./html-rules).
- **Connection error** → If using a URL, ensure the server is running and reachable.

## Uninstalling the App

To remove the **HTML-USSD** CLI from your system, run the following command:

```bash
html-ussd uninstall
```
