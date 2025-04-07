# `<body>` Tag

The `<body>` tag contains the **visible content** of your USSD page.

## Rules

- Must be inside [`<html>`](./html-tag).
- Holds text, [`<p>`](./p-tag) paragraphs, [`<a>`](./a-tag) links, or [`<form>`](./form-tag).

> [!IMPORTANT]
>
> - If the `<body>` contains neither [`<a>`](./tags/a-tag) nor a [`<form>`](./tags/form-tag), it is considered a terminal screen. This means the program ends, and no user input is expected.


## Example

```html
<body>
  Welcome to our service!
  <p>Please choose an option:</p>
  <a href="http://localhost:8888/pay">Pay Now</a>
</body>
```
