# UPT — **U**niversal **P**ackage-management **T**ool

[![Build status](https://github.com/sigoden/upt/actions/workflows/ci.yml/badge.svg)](https://github.com/sigoden/upt/actions)
[![Crates.io](https://img.shields.io/crates/v/upt.svg)](https://crates.io/crates/upt)

Upt provides a unified command interface to manage packages for any operating system. 

Upt relies on the platform's package management tool to perform the task, it's more like a wrapper or adaptive alias.

## Install

**Use Cargo**

Upt is written in the rust so you can install it using [cargo](https://doc.rust-lang.org/stable/cargo/).

```sh
cargo install upt
```

**Use Shell (Mac, Linux)**

```
curl -fsSL https://raw.githubusercontent.com/sigoden/upt/main/install.sh | sh -s -- --to /usr/local/bin
```

**Binaries for macOS, Linux, Windows, BSD**

Download from [GitHub Releases](https://github.com/sigoden/upt/releases), unzip and add `upt` to your $PATH.

## Features

### Unified command interface

Each operating system (OS) has its own package management tool, which requires different commands to complete the same operation.
This can be inconvenient when switching between or trying new OSs. 

```sh
apt install $pkg          # Ubuntu, Debian, Linux Mint...
apk add $pkg              # Alpine
pacman -S $pkg            # Arch, Manjaro...
nix-env -i $pkg           # Nixos
xbps-install $pkg         # Voidlinux
emerge $pkg               # Gentoo
```

With `upt`, You just need to remember one command:

```sh
upt install $pkg          # Works on any OS
```

Upt identifies the os type and runs the appropriate package management tool to install `$pkg`.

### Act as another tool

Upt can act as another tool and use their syntax by renaming it.

```sh
cp upt brew
brew install $pkg

cp upt pacman
pacman -S $pkg

cp upt emerge
emerge $pkg
```

In this way, you can use the syntax of the tool you are most familiar with to manage packages.

### Supported tools

```
| Tool        | Install                     | Uninstall                   | Upgrade                         | Search                | Info                           | Update Index           | Upgrade All              | List Installed                    |
| ----------- | --------------------------- | --------------------------- | ------------------------------- | --------------------- | ------------------------------ | ---------------------- | ------------------------ | --------------------------------- |
| upt         | upt install $pkg            | upt remove/uninstall $pkg   | upt upgrade $pkg                | upt search $pkg       | upt info/show $pkg             | upt update             | upt upgrade              | upt list                          |
| apk         | apk add $pkg                | apk del $pkg                | apk upgrade $pkg                | apk search $pkg       | apk info $pkg                  | apk update             | apk upgrade              | apk list -I/--installed           |
| apt         | apt install $pkg            | apt remove $pkg             | apt install --only-upgrade $pkg | apt search $pkg       | apt show $pkg                  | apt update             | apt upgrade              | apt list -i/--installed           |
| brew        | brew install $pkg           | brew uninstall $pkg         | brew upgrade $pkg               | brew search $pkg      | brew info $pkg                 | brew update            | brew upgrade             | brew list                         |
| cards       | cards install $pkg          | cards remove $pkg           | cards install -u/--upgrade $pkg | cards search $pkg     | cards info $pkg                | cards sync             | cards upgrade            | cards list                        |
| choco       | choco install $pkg          | choco uninstall $pkg        | choco upgrade $pkg              | choco search $pkg     | choco info $pkg                | -                      | choco upgrade all        | choco list                        |
| dnf         | dnf install $pkg            | dnf remove $pkg             | dnf upgrade $pkg                | dnf search $pkg       | dnf info $pkg                  | dnf check-update       | dnf update               | dnf list --installed              |
| emerge      | emerge $pkg                 | emerge --depclean $pkg      | emerge --update $pkg            | emerge --search $pkg  | emerge --info $pkg             | emerge --sync          | emerge -vuDN @world      | qlist -lv                         |
| eopkg       | eopkg install $pkg          | eopkg remove $pkg           | eopkg upgrade $pkg              | eopkg search $pkg     | eopkg info $pkg                | eopkg update-repo      | eopkg upgrade            | eopkg list-installed              |
| flatpak     | flatpak install $pkg        | flatpak uninstall $pkg      | flatpak update $pkg             | flatpak search $pkg   | flatpak info $pkg              | -                      | flatpak update           | flatpak list                      |
| guix        | guix install $pkg           | guix remove $pkg            | guix upgrade $pkg               | guix search $pkg      | guix show $pkg                 | guix refresh           | guix upgrade             | guix package -I/--list-installed  |
| nala        | nala install $pkg           | nala remove $pkg            | nala install $pkg               | nala search $pkg      | nala show $pkg                 | nala update            | nala upgrade             | nala list -i/--installed          |
| nix-env     | nix-env -i/--install $pkg   | nix-env -e/--uninstall $pkg | nix-env -u/--upgrade $pkg       | nix-env -qaP $pkg     | nix-env -qa --description $pkg | nix-channel --update   | nix-env -u/--upgrade     | nix-env -q/--query --installed    |
| opkg        | opkg install $pkg           | opkg remove $pkg            | opkg upgrade $pkg               | opkg find $pkg        | opkg info $pkg                 | opkg update            | opkg upgrade             | opkg list --installed             |
| pacman      | pacman -S $pkg              | pacman -Rs $pkg             | pacman -S $pkg                  | pacman -Ss $pkg       | pacman -Si $pkg                | pacman -Sy             | pacman -Syu              | pacman -Q                         |
| pkg         | pkg install $pkg            | pkg remove $pkg             | pkg install $pkg                | pkg search $pkg       | pkg info $pkg                  | pkg update             | pkg upgrade              | pkg info -a/--all                 |
| pkg(termux) | pkg install $pkg            | pkg uninstall $pkg          | pkg install $pkg                | pkg search $pkg       | pkg show $pkg                  | pkg update             | pkg upgrade              | pkg list-installed                |
| pkgman      | pkgman install $pkg         | pkgman uninstall $pkg       | pkgman update $pkg              | pkgman search $pkg    | -                              | pkgman refresh         | pkgman update            | pkgman search -i -a               |
| prt-get     | prt-get install $pkg        | prt-get remove $pkg         | prt-get update $pkg             | prt-get search $pkg   | prt-get info $pkg              | ports -u               | prt-get sysup            | prt-get listinst                  |
| scoop       | scoop install $pkg          | scoop uninstall $pkg        | scoop update $pkg               | scoop search $pkg     | scoop info $pkg                | scoop update           | scoop update *           | scoop list                        |
| slackpkg    | slackpkg install $pkg       | slackpkg remove $pkg        | slackpkg upgrade $pkg           | slackpkg search $pkg  | slackpkg info $pkg             | slackpkg update        | slackpkg upgrade-all     | ls -1 /var/log/packages           |
| snap        | snap install --classic $pkg | snap remove $pkg            | snap refresh $pkg               | snap find $pkg        | snap info $pkg                 | -                      | snap refresh             | snap list                         |
| urpm        | urpmi $pkg                  | urpme $pkg                  | urpmi $pkg                      | urpmq -y/--fuzzy $pkg | urpmq -i $pkg                  | urpmi.update -a        | urpmi --auto-update      | rpm -q/--query --all              |
| winget      | winget install $pkg         | winget uninstall $pkg       | winget upgrade $pkg             | winget search $pkg    | winget show $pkg               | -                      | winget upgrade --all     | winget list                       |
| xbps        | xbps-install $pkg           | xbps-remove $pkg            | xbps-install -u/--update $pkg   | xbps-query -Rs $pkg   | xbps-query -RS $pkg            | xbps-install -S/--sync | xbps-install -u/--update | xbps-query -l/--list-pkgs         |
| yay         | yay -S $pkg                 | yay -Rs $pkg                | yay -S $pkg                     | yay -Ss $pkg          | yay -Si $pkg                   | yay -Sy                | yay -Syu                 | yay -Q                            |
| yum         | yum install $pkg            | yum remove $pkg             | yum upgrade $pkg                | yum search $pkg       | yum info $pkg                  | yum check-update       | yum update               | yum list --installed              |
| zypper      | zypper install $pkg         | zypper remove $pkg          | zypper update $pkg              | zypper search $pkg    | zypper info $pkg               | zypper refresh         | zypper update            | zypper search -i/--installed-only |
```

### OS Tools

```
+------------------------------------------------------+----------------------+
| OS                                                   | Tools                |
+------------------------------------------------------+----------------------+
| windows                                              | scoop, choco, winget |
+------------------------------------------------------+----------------------+
| macos                                                | brew, port           |
+------------------------------------------------------+----------------------+
| ubuntu, debian, linuxmint, pop, deepin, elementary   | apt                  |
| kali, raspbian, aosc, zorin, antix, devuan, bodhi    |                      |
| lxle, sparky                                         |                      |
+------------------------------------------------------+----------------------+
| fedora, redhat, rhel, amzn, ol, almalinux, rocky     | dnf, yum             |
| oubes, centos, qubes, eurolinux                      |                      |
+------------------------------------------------------+----------------------+
| arch, manjaro, endeavouros, arcolinux, garuda        | pacman               |
| antergos, kaos                                       |                      |
+------------------------------------------------------+----------------------+
| alpine, postmarket                                   | apk                  |
+------------------------------------------------------+----------------------+
| opensuse, opensuse-leap, opensuse-tumbleweed         | zypper               |
+------------------------------------------------------+----------------------+
| nixos                                                | nix-env              |
+------------------------------------------------------+----------------------+
| gentoo, funtoo                                       | emerge               |
+------------------------------------------------------+----------------------+
| void                                                 | xbps                 |
+------------------------------------------------------+----------------------+
| mageia                                               | urpm                 |
+------------------------------------------------------+----------------------+
| slackware                                            | slackpkg             |
+------------------------------------------------------+----------------------+
| solus                                                | eopkg                |
+------------------------------------------------------+----------------------+
| openwrt                                              | opkg                 |
+------------------------------------------------------+----------------------+
| nutyx                                                | cards                |
+------------------------------------------------------+----------------------+
| crux                                                 | prt-get              |
+------------------------------------------------------+----------------------+
| freebsd, ghostbsd                                    | pkg                  |
+------------------------------------------------------+----------------------+
| android                                              | pkg(termux)          |
+------------------------------------------------------+----------------------+
| haiku                                                | pkgman               |
+------------------------------------------------------+----------------------+
| windows/msys2                                        | pacman               |
+------------------------------------------------------+----------------------+
| *                                                    | apt, dnf, pacman     |
+------------------------------------------------------+----------------------+
```

Upt will determine which package management tool to use based on the above table.

Some platforms may support multiple package management tools, upt selects one of them in order.

You can specify the package manager that UPT should use by setting the `UPT_TOOL` environment variable.

```sh
UPT_TOOL=brew upt install $pkg            # equal to `brew install $pkg`
UPT_TOOL=nix-env upt install $pkg         # equal to `nix-env -i $pkg`
```

## License

Copyright (c) 2023-∞ upt-developers.

Upt is made available under the terms of either the MIT License or the Apache License 2.0, at your option.

See the LICENSE-APACHE and LICENSE-MIT files for license details.
