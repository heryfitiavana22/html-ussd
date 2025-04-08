# Getting Started

Learn how to quickly install and run **HTML-USSD** to simulate USSD flows.

## Installation

- **Linux/macOS/WSL**

```bash
curl -sSL https://raw.githubusercontent.com/heryfitiavana22/html-ussd/main/scripts/install.sh | bash
```

- **Windows(powershell)**

```bash
irm https://raw.githubusercontent.com/heryfitiavana22/html-ussd/main/scripts/install.bat | iex
```

## Running the App

To simulate a USSD session, you need an HTML file or a URL that follows the [HTML Language Rules](./html-rules);

**From a remote server (this is live page) :**

```bash
html-ussd run --main "https://html-ussd-example.onrender.com/main-page"
```

**From a local file:**

```bash
html-ussd run --main index.html
```

## Command Line Help

To view all commands:

```bash
html-ussd --help
```

Output:

```bash
Usage: html-ussd <COMMAND>

Commands:
  run        Run the main program
  uninstall  Uninstall the html-ussd CLI
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

To view all available options for `run`:

```bash
html-ussd run --help
```

Output:

```bash
Usage: html-ussd run [OPTIONS] --main <MAIN>

Options:
  -l, --lang <LANG>                  Language to use (en, fr, mg) [default: en]
      --no-cache                     Disable caching when loading files from disk or server
  -m, --main <MAIN>                  The starting URL or main file name (e.g. http://localhost:8888 or index.html)
      --phone <PHONE>                Phone number to use for the USSD session [default: 0312345678]
      --query [<QUERY>...]           Query parameters, e.g. id=1,q=dfa
      --header [<HEADER>...]         Headers, e.g. Authorization=Bearer123
      --access-token <ACCESS_TOKEN>  Access token to add as Bearer in Authorization header
  -h, --help
```

## Uninstalling the App

To remove the **HTML-USSD** CLI from your system, run the following command:

```bash
html-ussd uninstall
``` 

## Next Steps

- Learn how to [structure your HTML files](./html-rules).
- Explore all [CLI options](./getting-started).
