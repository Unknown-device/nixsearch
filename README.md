# Nixsearch

A tui wrapper for nix-search-cli used to search the nixpkgs for packages

## What is this exactly?

Built with rust and ratatui, this tui uses nix-search-cli for a json search of nixpkgs and parses it and finally displays it to the user. It is not async, but it is very minimal and as such quite fast and features color highlighting.

### Screenshots/Demo/Examples

![firefox search example](https://crossroads.hackclub.com/uploads/3992e7d6f020bd7ff1ba692a863aaeb475f61fc0ac5d9c384af61571ad01e89f.png)

## How to run?

Only runs on linux

### Nixos

with flakes enabled run: 
```
nix run https://github.com/Unknown-device/nixsearch
```
or
```
nix-shell -p https://github.com/Unknown-device/nixsearch
```

### Manual
1. install cargo and nix-search-cli for os repositories
2. clone the repo and run with:

```
cargo run
```
or build with 
```
cargo build
```

## Usage

Press i to highlight search field and input search query and enter to execute.
Use arrow keys to navigate search results
Esc to exit search input and q to quit.

## License

This Project is licensed under the MIT license see License.md for more details.

--YSWS-- for hackclub
