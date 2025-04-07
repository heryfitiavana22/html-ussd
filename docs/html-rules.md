# General Rules

To simulate USSD flows, **HTML-USSD** expects HTML files to follow strict structure rules.

This ensures a consistent and simple experience, similar to real USSD menus on mobile phones.

## Structure Rules

Each HTML file must:

- Contain a [`<html>`](./tags/html-tag) tag as the root element.
- Include a [`<head>`](./tags/head-tag)  section with:
  - A [`<title>`](./tags/title-tag.md) tag to define the screen title.
- Use the [`<body>`](./tags/body-tag)  section to define the screen content, which may include:
  - Plain text
  - Paragraphs [(`<p>`)](./tags/p-tag) 
  - Links [(`<a>`)](./tags/a-tag) 
  - A single form [(`<form>`)](./tags/form-tag)  containing one input [(`<input>`)](./tags/input-tag) 

> [!IMPORTANT]
>
> - A page cannot include both [`<a>`](./tags/a-tag) links and a [`<form>`](./tags/form-tag).
> - All Text or Paragraph must be before [`<a>`](./tags/a-tag) or [`<form>`](./tags/form-tag).
> - If the `<body>` contains neither [`<a>`](./tags/a-tag) nor a [`<form>`](./tags/form-tag), it is considered a terminal screen. This means the program ends, and no user input is expected.

## Navigation Behavior

- **Links ([`<a>`](./tags/a-tag))**:  
  Users select a numeric option to navigate to another page or URL.

- **Form ([`<form>`](./tags/form-tag))**:  
  Users input text or numbers. The form must have an `action` attribute pointing to a server URL. It may optionally include a `method` attribute (`get` by default). Upon submission, the app proceeds to the target specified in the form action.

If no interaction is found (no [`<a>`](./tags/a-tag) and no [`<form>`](./tags/form-tag)), the application automatically exits.

## Example: A Simple Menu

```html
<html>
  <head>
    <title>Main Page</title>
  </head>
  <body id="container">
    Welcome to html-usdd
    <p>This is a paragraph</p>
    <a href="http://localhost:8888/payment">Payment</a>
    <a href="http://localhost:8888/form-get">Form with Get</a>
  </body>
</html>
```

::: details Result

```bash
+=============+
|  Main Page  |
+=============+

Welcome to html-usdd
This is a paragraph
    
1. Payment
2. Form with Get

------------------------
[#] > 
```

:::

## Example: A Form Input

```html
<html>
  <head>
    <title>Form get</title>
  </head>
  <body id="container">
    This is form with get
    <form action="http://localhost:8888/handle-text" method="get">
      <input type="text" name="text" placeholder="Enter text" />
    </form>
  </body>
</html>

```

::: details Result

```bash
+============+
|  Form get  |
+============+

This is form with get

Entrer un text

------------------------
0. Back
00. Home
[abc] > 
```

:::

## Example: An end screen

```html
<html>
  <head>
    <title>End screen</title>
  </head>
  <body id="container">
    This is and screen
  </body>
</html>

```

::: details Result

```bash
+==============+
|  End screen  |
+==============+

This is and screen
```

:::

By following these simple HTML rules, you can create complex USSD flows easily and preview them in your terminal.

Ready to launch your first flow? Learn about [CLI usage](./index).
