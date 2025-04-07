# Getting Started

Learn how to quickly install and run **HTML-USSD** to simulate USSD flows.

## Running the App

To simulate a USSD session, you need an HTML file or a URL that follows the [HTML Language Rules](./html-rules);

**From a local file:**

```bash
html-ussd --main index.html
```

**From a remote server:**

```bash
html-ussd --main http://localhost:8888/
```

## Command Line Help

To view all available options:

```bash
html-ussd --help
```

Output:

```bash
Usage: html-ussd [OPTIONS] --main <MAIN>

Options:
  -l, --lang <LANG>   Language to use (en, fr, mg) [default: en]
      --no-cache      Disable caching when loading files
  -m, --main <MAIN>   Starting HTML file or URL
  -h, --help          Print help
  -V, --version       Print version
```

## Next Steps

- Learn how to [structure your HTML files](./html-rules).
- Explore all [CLI options](./getting-started).
