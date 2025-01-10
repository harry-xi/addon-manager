# Addon Manager

> [!IMPORTANT]
> Unstable and with known bugs, be sure to check the list of issues before using it.

## Usage

```Text
addon-manager    version:0.1.2
A command line tool for installing and managing addons on bds

Usage: addon-manager [OPTIONS] [FILE] [COMMAND]

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
