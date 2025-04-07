# `<input>` Tag

The `<input>` element is used inside a [`<form>`](./form-tag) to **capture user input**.

## Rules

- Only allowed `type` are `text` or `number`.
- Must have `name` and `placeholder` attributes.

> [!IMPORTANT]
>
> - The `name` attribute in the `<input>` corresponds to the key in the query string or request body.


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
