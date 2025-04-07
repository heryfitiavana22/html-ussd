# `<input>` Tag

The `<input>` element is used inside a [`<form>`](./form-tag) to **capture user input**.

## Rules

- Only allowed `type` are `text` or `number`.
- Must have `name` and `placeholder` attributes.

> [!IMPORTANT]
>
> - The `name` attribute in the `<input>` corresponds to the key in the query string or request body.
> - If the `type` is set to `number`, entering a non-numeric value will cause an error.

## Attributes

| Attribute     | Values           | Default | Description                         | Required |
| ------------- | ---------------- | ------- | ----------------------------------- | -------- |
| `type`        | `text`, `number` | —       | Specifies the type of input         | Yes      |
| `name`        | string           | —       | Name of the input, used as the key. | Yes      |
| `placeholder` | string           | —       | Text displayed on screen            | Yes      |

## Example

- with type **text**

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

- with type **number**

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
