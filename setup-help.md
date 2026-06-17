# Setup Help

## Neovim `arduino-language-server` Crash

While opening `Game/Game.ino` in Neovim, the Arduino language server reported that details were in:

```text
/Users/quantf/.local/state/nvim/lsp.log
```

The useful part of the log was:

```text
panic: unimplemented request: workspace/semanticTokens/refresh
```

The server was otherwise configured correctly:

- `arduino-language-server` started.
- `arduino-cli` was found at `/opt/homebrew/bin/arduino-cli`.
- The board target was `arduboy:avr:arduboy`.
- The Arduboy2 library was discovered.
- `arduino-cli` produced a valid compile database.

The crash happened after `clangd` started indexing and sent a `workspace/semanticTokens/refresh` request through `arduino-language-server`. Arduino language server 0.7.7 does not handle that request and panics.

## Fix Applied

The active Neovim config is:

```text
/Users/quantf/.config/nvim
```

which is a symlink into:

```text
/Users/quantf/.dotfiles/nvim/.config/nvim
```

The Arduino LSP config was updated in:

```text
/Users/quantf/.config/nvim/lua/plugins/nvim-lspconfig.lua
```

The fix does two things:

1. Removes semantic token capabilities for `arduino_language_server`.
2. Runs `clangd` through a small shim instead of calling it directly.

The shim is:

```text
/Users/quantf/.local/bin/clangd-arduino-ls-shim
```

It runs `/usr/bin/clangd`, forwards normal LSP traffic, and replies `null` to `workspace/semanticTokens/refresh` so `arduino-language-server` does not crash.

The relevant Neovim server command is:

```lua
arduino_language_server = {
  capabilities = arduino_capabilities,
  cmd = {
    'arduino-language-server',
    '-clangd',
    vim.fn.expand '~/.local/bin/clangd-arduino-ls-shim',
    '-cli-config',
    vim.fn.expand '~/Library/Arduino15/arduino-cli.yaml',
    '-fqbn',
    'arduboy:avr:arduboy',
  },
}
```

## Verification

This command was used to verify the fix:

```sh
nvim --headless /Users/quantf/Documents/arduboy-game/Game/Game.ino \
  '+lua vim.defer_fn(function() print("clients", vim.inspect(vim.tbl_map(function(c) return { name = c.name, cmd = c.config.cmd } end, vim.lsp.get_clients({bufnr=0})))); vim.cmd("qa!") end, 9000)'
```

It showed `arduino_language_server` attached and using the shim:

```text
arduino-language-server -clangd /Users/quantf/.local/bin/clangd-arduino-ls-shim
```

The fresh LSP log then completed indexing and shut down cleanly, with no `workspace/semanticTokens/refresh` panic.

## If It Breaks Again

Restart the LSP inside Neovim:

```vim
:LspRestart arduino_language_server
```

Then inspect:

```sh
tail -200 /Users/quantf/.local/state/nvim/lsp.log
```

If the same semantic-token panic returns, confirm the Arduino LSP command still points at:

```text
/Users/quantf/.local/bin/clangd-arduino-ls-shim
```
