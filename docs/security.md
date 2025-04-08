# Security and External Requests

When using `html-ussd` with remote URLs (for `<a href="...">` or `<form action="...">`), you may need to authenticate or pass additional context with each request.

This page explains how to securely communicate with remote servers using query parameters and headers.

> [!IMPORTANT]
> These metadata (query, header) will be sent with every request.

## Query Parameters

You can pass custom query parameters to all external requests using the `--query` CLI option.

**Example:**

```bash
html-ussd run --main "https://html-ussd-example.onrender.com/main-page" --query "token=abc,user_id=42"
```

These will be appended to every request (GET or POST) as part of the URL.

> [!NOTE]
> If the original URL already has query parameters, your custom ones will be merged.

## Authorization Headers

To add HTTP headers to your requests (like authentication), use the --header option:

```bash
html-ussd run --main "https://html-ussd-example.onrender.com/main-page" --header "Authorization=Bearer 123,X-Custom=Value"
```

## Access Token Shortcut

If you're only passing a Bearer token, you can simplify the CLI with:

```bash
--access-token "xyz"
```

This is equivalent to:

```bash
--header "Authorization=Bearer xyz"
```
