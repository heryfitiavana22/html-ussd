# Cache

Caching helps speed up your simulations by storing HTML for faster access. Understanding how caching works and how to control it is important for ensuring your USSD flows behave as expected.

## How Caching Works

- When **HTML-USSD** loads html from file or server url for the first time, it saves a copy in the cache.
- If you navigate to the same page again, it uses the cached version, which speeds up the process.

## Disabling Cache

By default, caching is enabled, but you can disable it with the `--no-cache` option. This forces HTML-USSD to reload every time.

Example:

```bash
html-ussd --main index.html --no-cache
```
