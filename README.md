# pip-upgrade
> Made in Rust 🦀

This tool is designed to help you upgrade your globally installed Python packages

## Installation

```bash
cargo install pip-upgrade
```

## Available Commands

### `pip-upgrade`

This command will upgrade all of your installed Python packages to the latest version.

![upgrade.gif](https://github.com/Araxeus/pip-upgrade/raw/master/assets/upgrade.gif "Upgrade command")

### `pip-upgrade --outdated`

This command will show you if any of your installed Python packages are out of date.

alias: `pip-upgrade -o` or `pip-upgrade o` or `pip-upgrade outdated`

![outdated.gif](https://github.com/Araxeus/pip-upgrade/raw/master/assets/outdated.gif "--outdated command")

### How it works

To get outdated packages, it will parse `pip list --outdated --format=json`

Then if upgrading, it will run `pip install --upgrade <package>`.
