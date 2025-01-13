# Addon Manager

> [!IMPORTANT]
> Unstable and with known bugs, be sure to check the list of issues before using it.

## Usage

### 1. To install a package with defualt settings

```bash
addon-manager install /path/to/your_package.mcpack  # (or *.mcaddon / *.zip)
```

or just use

```bash
addon-manager /path/to/your_package.mcpack  # (or *.mcaddon / *.zip)
```

### The out put of subcammnd help

```text
addon-manager    version:0.2.0
A command line tool for installing and managing addons on bds

Usage: addon-manager [OPTIONS] [FILE]
       addon-manager <COMMAND>

Commands:
  list     list all installed addon(s). Default will show all in-use resource and behavior packages
  install  install addon to the level
  remove   Uninstall the addon to install to the level
  help     Print this message or the help of the given subcommand(s)

Arguments:
  [FILE]  The addon to be installed

Options:
  -w, --world <WORLD>                  Declare the name of the world you want to operate on. Invalid when the work path is a level [default: "Bedrock level"]
      --force-dirtype <FORCE_DIRTYPE>  [possible values: bds, level]
  -h, --help                           Print help (see more with '--help')
  -V, --version                        Print version
```
