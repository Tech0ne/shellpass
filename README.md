# shellpass

[![Release](https://github.com/Tech0ne/shellpass/actions/workflows/release.yml/badge.svg?branch=main)](https://github.com/Tech0ne/shellpass/actions/workflows/release.yml)

A terminal-based password manager written in Rust.

## Features

- Encrypted vault stored locally (AES-256-GCM, Argon2 key derivation)
- Organize credentials into **profiles**, each containing **entries**
- Per-entry fields: username, password, website, and **custom data**
- Copy fields to clipboard with an automatic 10-second clear timer for passwords
- Responsive TUI

## Vault

The vault is a single file (`vault.dat`) stored in your system's local data directory by default. It is encrypted at rest — the master password is never stored. A custom path can be passed via `--vault-dir/-v`.

## Navigation

| Key | Action |
|---|---|
| `j` / `k` | Move down / up |
| `h` / `Esc` | Go back |
| `l` / `↵` | Open / confirm |
| `g` / `G` | Jump to first / last |

## Pages

**Profiles** — top-level list of credential groups.
`n` new · `r` rename · `d` delete · `Ctrl+S` save · `Ctrl+X` save and quit

**Entries** — credentials within a profile.
`n` new · `e` edit · `d` delete · `↵` view detail

**Entry detail** — view fields and copy to clipboard.
`j/k` move between fields · `↵` copy focused field to clipboard

**Edit entry** — create or update an entry.
Switches between Normal and Insert mode (`i` to enter, `Esc` to leave).
`Tab` advances to the next field. On the Custom Data field, `Tab` and `↵` insert literal whitespace.
`Ctrl+S` saves and returns.

## Building

This project uses [just](https://github.com/casey/just) for building. You can use `just build -r` to build a `shellpass` binary in release mode.

## Usage

```
./shellpass
```

## TODO

- [ ] Improve codebase / reworks
- [ ] Add more customisability, through config file/cli args
- [ ] Add (more) vim-like controls (:w, :q...)
- [x] Add github actions Ci/Cd for automatic releases
- [x] Add mouse support
