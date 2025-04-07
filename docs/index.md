---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: "HTML USSD"
  text: "Simulate USSD experiences directly from HTML"
  tagline: Build and preview USSD-like interfaces using simple HTML and run them in your terminal.
  actions:
    - theme: brand
      text: Get Started
      link: /getting-started
    - theme: alt
      text: CLI Usage
      link: /cli

features:
  - title: HTML-Based
    details: Write simple HTML files with custom rules to define USSD screens. No need for custom DSLs or frameworks.
  - title: Terminal Renderer
    details: Render USSD flows directly inside your terminal with a simple and clean UI experience.
  - title: Navigation & Forms
    details: Supports links and forms with input validation, server interactions, and local file-based flows.
  - title: Multilingual Support
    details: Easily localize your USSD interface in English, French, or Malagasy.
  - title: CLI Launcher
    details: Launch from an HTML file or a remote URL using the CLI with language selection and optional caching.
  - title: Extendable
    details: Built with separation of concerns (adapter, renderer, validator) following Hexagonal Architecture principles.
---
