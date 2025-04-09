# Cache

Caching helps speed up your simulations by storing HTML for faster access. Understanding how caching works and how to control it is important for ensuring your USSD flows behave as expected.

## How Caching Works

- When **HTML-USSD** loads html from file or server url for the first time, it saves a copy in the cache.
- If you navigate to the same page again, it uses the cached version, which speeds up the process.

## Disabling Cache

HTML-USSD uses caching to avoid reloading the same pages repeatedly.

There are **two ways** to disable caching:

### 1. CLI Option: `--no-cache`

Passing the `--no-cache` flag disables **all caching** during the session. Every page will be fetched fresh, even if it was already visited.

Example:

```bash
html-ussd run --main "https://html-ussd-example.onrender.com/main-page" --no-cache
```

### 2. HTML Attribute: `cache="false"`

If you want to selectively disable caching for specific pages only, you can add the attribute cache="false" on the root[ `<html>`](./tags/html-tag) tag of that page:

```html
<html cache="false">
  ...
</html>
```

This forces the system to re-fetch that page even if global caching is enabled.
