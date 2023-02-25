# pip-upgrade
> Made in Rust 🦀

This tool is designed to help you upgrade your globally installed Python packages

## Installation

```bash
cargo install pip-upgrade
```

## Available Commands

### `pip-upgrade --upgrade`

This command will upgrade all of your installed Python packages to the latest version.

alias: `pip-upgrade -u` or `pip-upgrade u` or `pip-upgrade update`

### `pip-upgrade --outdated`

This command will show you if any of your installed Python packages are out of date.

alias: `pip-upgrade -o` or `pip-upgrade o` or `pip-upgrade outdated`

### How it works

To get outdated packages, it will parse `pip list --outdated --format=json`

Then if upgrading, it will run `pip install --upgrade <package>`.
