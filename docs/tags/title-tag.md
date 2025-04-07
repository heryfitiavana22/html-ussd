# `<title>` Tag

The `<title>` element defines the **screen title** displayed at the top of the USSD interface.

## Rules

- Must be inside the [`<head>`](./head-tag) section.
- Only **one** `<title>` tag per page.
::: danger
Missing or multiple `<title>` tags will cause an error.
:::

## Example

```html
<head>
  <title>Welcome Page</title>
</head>
```

::: details Result

```bash
+================+
|  Welcome Page  |
+================+
```

:::
