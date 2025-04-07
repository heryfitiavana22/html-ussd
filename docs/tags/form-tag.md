# `<form>` Tag

The `<form>` element allows the user to **submit input** (text or numbers).

## Rules

- Must have an `action` attribute pointing to a server URL.
- The `method` attribute is optional and specifies how form data is sent to the server. It can be either **get** (default) or **post**.
- Must contain exactly **one** [`<input>`](./input-tag) tag.
- Cannot be mixed with [`<a>`](./a-tag) links on the same page.

> [!IMPORTANT]
>
> - When using **get**, the data is included in the _query string_.
> - When using **post**, the data is included in the _request body_, with the `Content-Type` header set to `application/x-www-form-urlencoded`.
> - With both **get** and **post**, the `name` attribute in the `<input>` corresponds to the key in the query string or request body.

## Attributes

| Attribute | Values        | Default | Description                                | Required |
| --------- | ------------- | ------- | ------------------------------------------ | -------- |
| `action`  | Server URL    | —       | URL where the form data will be submitted. | Yes      |
| `method`  | `get`, `post` | `get`   | Method used to submit the form data..      | No       |

## Example

- with method **get**

```html
<form action="http://localhost:8888/submit">
  <input type="text" name="code" placeholder="Enter code" />
</form>
```

::: details Result

```bash
Enter code
[abc] >
```

:::

- with method **post**

```html
<form action="http://localhost:8888/submit" method="post">
  <input type="number" name="code" placeholder="Enter code" />
</form>
```

::: details Result

```bash
Enter code
[123] >
```

:::
