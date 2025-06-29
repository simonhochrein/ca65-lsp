# CA65 Assembly Language Server

ca65-lsp is a language server, parser, and semantic analyzer for the [CA65](https://cc65.github.io/doc/ca65.html)
assembly dialect. It is part of ongoing efforts to improve tooling for the 6502 processor family.

> Note: This project is in active development. Some features may not work fully, others at all. Please watch/star the
> project to stay up to date!

## Quick Start

TBD

## Editor Setup

- [VSCode](https://github.com/techwritescode/ca65-code)
- [Zed](https://github.com/techwritescode/ca65-zed)
- Neovim
  ```lua
  -- init.lua
  require 'lspconfig.configs'.ca65 = {
  	default_config = {
  		cmd = { "/path/to/ca65-lsp" },
  		filetypes = { "s", "asm" },
  		root_dir = require 'lspconfig'.util.root_pattern('ca65.toml')
  	}
  }
  require 'lspconfig'.ca65.setup{}
  ```
- Helix
    - If it doesn't already exist, create a [
      `languages.toml` file](https://docs.helix-editor.com/languages.html#languagestoml-files). I put mine in
      my [config directory](https://docs.helix-editor.com/configuration.html) which for me (on Windows 11) is
      `~/AppData/Roaming/helix/`
    - Add the following to it:
      ```toml
      # languages.toml
      [language-server.ca65-lsp]
      command = "ca65-lsp"
      
      [[language]]
      name = "ca65"
      scope = "source.s"
      comment-tokens = ";"
      file-types = [ "ca65", "s", "asm" ]
      language-servers = [ "ca65-lsp" ]
      
      [[grammar]]
      name = "ca65"
      source = { git = "https://github.com/techwritescode/tree-sitter-ca65", rev = "9e73befb5c3c6852f905964c22740c9605b03af8" }
      ```
    - To get syntax highlighting working
        - Navigate to `<your helix install directory>/runtime/queries/`
        - Create a directory called `ca65`
        - Copy `highlights.scm` from the `techwritescode/tree-sitter-ca65` repo, under `queries/ca65/`, into the `ca65`
          directory you just created
        - You may need to run
          ```bash
          hx --grammar fetch && hx --grammar build
          ```
