# Install

Install the `shortkut` binary:

## Install Latest Version

On Windows:

```
Set-ExecutionPolicy -Scope CurrentUser RemoteSigned -force
iwr -useb get.shortkut.sh | iex
```

On MacOSX or Linux:

```bash
bash -c "$(sudo curl -fsSL https://shortkut.sh/install.sh)"
```

To update the Shortkut itself, rerun the above script. It will replace the current version without touching Shortkut's configuration files.
