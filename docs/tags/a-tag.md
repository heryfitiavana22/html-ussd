# `<a>` Tag

The `<a>` tag is used to **create navigation links** in the USSD interface.

## Rules

- Must have an `href` attribute pointing to another screen (local or remote).
- Must be placed inside [`<body>`](./body-tag).
- Cannot be mixed with [`<form>`](./form-tag) on the same page.

::: details `href` with local file

When using `href` with a local file, the value should be the **name of the file to retrieve**, relative to the location of the main file. Ensure the file exists in the same directory or provide the correct relative path.

```bash
html/
├─ sub/
│  ├─ s1.html
├─ index.html
├─ page1.html
```

::: code-group

```html [index.html]
<html>
  <head>
    <title>Main Page</title>
  </head>
  <body>
    Welcome
    <a href="page1.html">Page 1</a>
    <a href="sub/s1.html">Sub folder s1</a>
  </body>
</html>
```

```html [page1.html]
<html>
  <head>
    <title>Page 1</title>
  </head>
  <body>
    Welcome to page 1
  </body>
</html>
```

```html {7} [sub/s1.html]
<html>
  <head>
    <title>Sub s1</title>
  </head>
  <body>
    Welcome to sub s1
    <a href="index.html">Main page</a>
  </body>
</html>
```

:::

## Attributes

| Attribute | Values                        | Default | Description                             | Required |
| --------- | ----------------------------- | ------- | --------------------------------------- | -------- |
| `href`    | Local file path or server URL | —       | Link to another page (local or remote). | Yes     |

## Example

- with **server url**

```html
<body>
  <a href="http://localhost:8888/payment">Make a Payment</a>
  <a href="http://localhost:8888/balance">Check Balance</a>
</body>
```

- with **local file**

```html
<body>
  <a href="payment.html">Make a Payment</a>
  <a href="balance.html">Check Balance</a>
</body>
```

::: details Result

```bash
1. Make a Payment
2. Check Balance
```

:::
