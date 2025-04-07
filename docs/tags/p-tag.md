# `<p>` Tag

The `<p>` tag allows you to add **paragraphs of text** to your USSD screens.

## Rules

- Place inside [`<body>`](./body-tag), before any [`<a>`](./a-tag) or [`<form>`](./form-tag).
- Multiple `<p>` tags are allowed.
::: info
Plain text is allowed, but using `<p>` is recommended

```html
  Welcome to our service.
  Choose an option below.
```

:::

::: danger
Nesting `<p>` tags will cause an error.

```html
  <p>Welcome to our service.
    <p>Choose an option below.</p>
  </p>
```

:::

## Example

```html
<body>
  <p>Paragraph 1</p>
  <p>Paragraph 2</p>
</body>
```

::: details Result

```bash
Paragraph 1
Paragraph 2
```

:::
