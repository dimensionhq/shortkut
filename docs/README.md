---
home: true
heroImage: /logo.png
heroText: null
tagline: The easiest way to 10x your command line productivity.
actionText: Install Shortkut →
actionLink: ./install/
features:
  - title: Compatibility First
    details: Works on the most common shells on the most common operating systems. Use it everywhere!
  - title: Rust-Powered
    details: Brings the best-in-class speed and safety of Rust, to make your shortkut packs as quick and reliable as possible.
  - title: Open Source
    details: Shortkut is available on GitHub and contributing a pack is as simple as making a PR!
footer: Apache-2.0 Licensed | Copyright © 2021-present XtremeDevX

# Used for the description meta tag, for SEO
metaTitle: "Shortkut | 10x Your Command-line Productivity"
description: The easiest way to 10x your command line productivity. Quick installation available for Windows, MacOSX and Linux.
---

<div class="center">
  <img class="demo-video" src="/demo.gif">
</div>

### Install

Install the `shortkut` binary:

#### Install Latest Version

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
