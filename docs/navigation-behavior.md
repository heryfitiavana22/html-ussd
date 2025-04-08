# Navigation Behavior

This page explains how the USSD simulation engine handles user interactions and transitions between screens.

## Links ([`<a>`](./tags/a-tag))

Users select a numeric option (e.g. `1`, `2`, `3`) to navigate between screens.

- When a link is selected, the simulator fetches the target from the href attribute.
- The user input (the selected number) is automatically appended to the URL as a query parameter:
  `user_entry=<selected number>`

**Example**

```html
<a href="https://html-ussd-example.onrender.com/handle-offre">Offre 1</a>
<a href="https://html-ussd-example.onrender.com/handle-offre">Offre 2</a>
```

If the user presses `1`, the request becomes:

```bash
https://html-ussd-example.onrender.com/handle-offre?user_entry=1
```

## Form ([`<form>`](./tags/form-tag)):

Users are prompted to enter a value (text or number). Forms must include:

- `action` → the URL to send the submission to
- `method` (**get** or **post**) → defaults to get if not specified

Depending on the method, the form data is either:

- sent as query parameters (`GET`)
- or in the body (`POST`) — with the `Content-Type` header set to `application/x-www-form-urlencoded`.

## Exit Condition

If a screen contains no link (`<a>`) and no form (`<form>`), the application exits automatically.

This marks the end of the USSD session.
