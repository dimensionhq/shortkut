# Shortkut

A cross-platform, performant terminal shortcut manager.

## 🔨 Build Status

| Feature                  | Windows | MacOS | Linux |
| ------------------------ | ------- | ----- | ----- |
| Install Shortcut Bundle  | ✅      | ✅    | ✅    |
| Register Custom Shortcut | ✅      | ✅    | ✅    |
| Remove Shortcut Bundle   | ✅      | ✅    | ✅    |
| Remove Custom Shortcut   | ✅      | ✅    | ✅    |

## 📦 Installation

### Rapid Install

Install `shortkut`:

On Windows:

```ps1
Set-ExecutionPolicy -Scope CurrentUser RemoteSigned -force
iwr -useb get.shortkut.sh | iex
```

On MacOSX or Linux:

```bash
bash -c "$(sudo curl -fsSL https://shortkut.sh/install.sh)"
```

To update the `shortkut` itself, rerun the above script. It will replace the current version without touching Shortkut's configuration.

### Other Options

Download the latest release for your operating system from the [Releases](https://github.com/XtremeDevX/shortkut/releases) page and run the installer or extract the required files.

Type `shortkut` for a help menu.

```ps1
shortkut 1.0.0
* add - Add a shortcut
* remove - Remove a shortcut
* show - Show a shortcut pack
* search - Search for a shortcut pack
```
