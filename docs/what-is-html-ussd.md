# What is HTML-USSD?

**HTML-USSD** is a tool that lets you create and test USSD menus using simple HTML files — and run them right in your terminal.

## Why?

USSD (like `*141#`, `*111#`, etc.) is widely used on mobile phones for checking balance, making payments, or navigating simple menus.  
But building and testing USSD apps usually requires real phones or telecom tools.

**HTML-USSD** makes that easier:  
You just write a few HTML files with a specific structure, and it simulates a USSD menu flow on your terminal.

## What can it do?

- Run **USSD-like menus** from HTML
- Let users **navigate with numbers** or **input text**
- Work with local files or online URLs
- Support multiple languages: English, French, Malagasy 🇲🇬
- Help you build, test, or demo USSD apps without a phone

## Who is it for?

- **Developers** simulating USSD flows without real telecom access.
For example: your app requires a mobile-style payment flow, but you can't integrate with real USSD services — HTML-USSD lets you prototype and simulate it easily.
- **Exploring** how USSD works in practice.
- **Prototyping** USSD menus for quick feedback.

## How does it work?

HTML-USSD reads structured HTML files:
- The `<title>` becomes the screen title
- The `<body>` shows text, menus (`<a>`), or input forms
- Users navigate by typing numbers or filling input

Ready to try? Start with the [Getting Started guide](/getting-started).
