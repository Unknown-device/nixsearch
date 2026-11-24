# Nixsearch

A tui wrapper for nix-search-cli used to search the nixpkgs for packages

## What is this exactly?

Built with rust and ratatui, this tui uses nix-search-cli for a json search of nixpkgs and parses it and finally displays it to the user. It is not async, but it is very minimal and as such quite fast and features color highlighting. Searches are served by search.nixos.org, so an internet connection is required.

### Screenshots/Demo/Examples

![firefox search example](https://crossroads.hackclub.com/uploads/3992e7d6f020bd7ff1ba692a863aaeb475f61fc0ac5d9c384af61571ad01e89f.png)

## How to run?

### With Nix (flakes)

Run it directly with flakes enabled:

```
nix run github:Unknown-device/nixsearch
```

Or install it permanently and then launch it with `nixsearch`:

```
nix profile install github:Unknown-device/nixsearch
```

Or enter a temporary shell that contains `nixsearch`:

```
nix shell github:Unknown-device/nixsearch
```

Supported systems are x86_64-linux, aarch64-linux and aarch64-darwin.
x86_64-darwin is currently unavailable because the pinned nixpkgs no longer supports it.

### Manual

Requirements: Rust 1.88 or newer and `nix-search` (nix-search-cli) on your PATH.

1. clone the repo and enter its directory:
```
git clone https://github.com/Unknown-device/nixsearch.git
cd nixsearch
```
2. if you have Nix, the flake's dev shell provides Rust and nix-search-cli:
```
nix develop
```
otherwise install Rust (e.g. via rustup) and nix-search-cli from your package manager.

3. run or build with:
```
cargo run
```
or
```
cargo build
```

## Usage

Press i to highlight search field and input search query and enter to execute.
Use arrow keys to navigate search results
Esc to exit search input and q to quit.

## License

This Project is licensed under the MIT license see [LICENSE.md](LICENSE.md) for more details.

--YSWS-- for hackclub
