# Changelog

All notable changes to this project will be documented in this file.

## [1.0.0] - 2025-04-08

### Features

- Separate command module, add cli "phone" option
- HTTP client functionality and add query/header parsing in CLI
- Add access token handling in CLI and update header processing in Runner
- Add cache support to Html struct and update related functionality in UssdController and ValidatorAndTransformer

### Other

- Remove id attributes from body tags in HTML files and update related pages

### Refactor

- Rename fetch_html to fetch_page and update usage in runner
- Simplify query and header parsing in Runner
- Simplify cache handling in UssdController and fix uninstall command syntax in Runner

### Documentation

- Update README and index documentation
- Update documentation for CLI usage, navigation behavior, security, and caching; enhance examples and add uninstall command

## [0.1.0] - 2025-04-08

### Features

- Add adapter and dom_tree_adapter and implement page validation logic, Update parse-html dependency to version 0.4.1
- Introduce HtmlUssdTree and related structures for enhanced HTML representation
- Rename page_validator module to validator_and_transformer and update attribute types to HashMap
- Enhance implementation and error in ValidatorAndTransformer. test: ValidatorAndTransformer and DomTreeAdapter
- Implement Screen and Renderer for HTML representation and user interaction
- Update ValidatorAndTransformer to handle body content with links
- Update DomTreeAdapter to accept HTML input and handle parsing errors
- Enhance input handling in Renderer and TerminalRenderer
- Add placeholder attribute to Input and update validation logic
- Implement UssdController and remove Screen module
- Add express server with HTML pages for balance and payment functionalities
- Add server navigation and enhance href validation in UssdController
- Add action attribute to Form struct and enhance form method validation
- Response handling and send request with form
- Checking input name
- Integrate rustyline for improved terminal input handling
- Enhance RenderParams and DisplayParams to include is_main_page flag for better page context handling
- Update TerminalRenderer to conditionally display navigation options based on is_main_page flag
- Implement history tracking in UssdController for navigation management
- Enhance history management in UssdController with HistoryItem struct
- Enable history tracking by setting push_to_history to true in UssdController
- Add history_enabled flag to Html struct and update ValidatorAndTransformer to handle it
- Replace push_to_history with is_next_page in DisplayParams and check if history_enabled
- Define constants for back and home keys in UssdController for improved readability
- Enforce rule that text must appear before links or forms in HTML validation
- Add helper module with is_server_url function and update validator logic
- Validate form action URLs to ensure they are server URLs
- Add unit tests for is_server_url function to validate URL protocols
- Implement internationalization support with translation functionality
- Enhance terminal output with centered title and decorative borders
- Improve terminal input prompt styling with gray color
- Add muted text formatting function for terminal input prompts
- Add URL fetching functionality and CLI support  with clap
- Improve file loading functionality
- Rename 'pages' to 'cache_pages' for clarity in UssdController and NewController, and display using cache first
- Implement caching mechanism in UssdController and add no_cache option in CLI
- Refactor CLI structure to simplify file loading and remove subcommand options
- Enhance Renderer trait with render_text and render_error methods for improved error handling
- Add translation for "good_bye" in I18n and update terminal renderer to use it

### Bug Fixes

- Improve flow control in UssdController by adding early returns and cleaning up history handling
- Update comments for clarity in Cli struct
- Warning from clippy
- *(ci)* Correct directory context for dependency installation and build steps in deploy workflow
- *(ci)* Add missing 'with' configuration for pnpm action and ensure consistent formatting
- *(ci)* Add missing cache-dependency-path for Node setup and clean up whitespace

### Other

- Add Handlebars templates for various pages and forms
- Add new routes and pages for 'not in history' functionality
- Add HTML examples for form handling and navigation
- Add sub folder page and link in main page for improved navigation
- Add installation

### Refactor

- Enhance page validation logic and update error handling for HTML structure
- Variable naming in ValidatorAndTransformer for improved readability
- Remove unnecessary meta tags from HTML pages
- To own folder "renderer" and "adapter"
- Rename test functions for clarity and consistency
- Update Renderer trait to use RenderParams for improved parameter handling
- Replace get(0) with first() for improved readability and consistency
- Clean up code by removing unnecessary println! statements and whitespace
- Update function parameter names for clarity and add Runner struct for better organization
- Rename adapter field to tag_adapter for consistency in UssdController and NewController

### Documentation

- Add README
- Add CLI documentation and improve HTML rules clarity
- Add attributes section to multiple tag documentation files
- Improve CLI documentation for clarity and consistency
- Add "How cache works" section to documentation
- Remove default examples
- Improve CLI options table formatting and enhance clarity in usage examples
- Update GitHub link in config and add installation instructions in getting started guide
- Add 'Advanced' section with cache and interface extension guides
- Clarify the extensibility of HTML-USSD and add example for custom JSON adapter
- Update example for custom JSON adapter to use Runner struct
- Rename extending-interface to supporting-custom-input-formats
- Change header level for Custom JSON Adapter example for consistency
- Update hero text and feature descriptions for clarity and consistency
- Add documentation for creating a custom renderer and update supporting custom input formats
- Add comprehensive changelog detailing features, bug fixes, refactors, and documentation updates

### Testing

- Enhance form by adding action and method attributes
- Add validation tests for form attributes and child elements
- Add unit test for missing input name validation in HTML
- Add unit test for invalid input child in HTML validation
- Update is_server_url tests to use assert_eq! for clarity

### Miscellaneous Tasks

- Add GitHub Actions workflow for building and releasing the project
- Include CHANGELOG.md in release assets
- Add GitHub Actions workflows for Clippy and Rust tests
- Add GitHub Actions workflow for deploying documentation to GitHub Pages
- Release html-ussd version 0.1.0

<!-- generated by git-cliff -->
