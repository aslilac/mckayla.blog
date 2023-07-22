---
title: Kayla's guide to setting up a VM
description: I can't ever remember how Linux works, so I made myself a guide. :^)
author: Kayla Washburn
date: 2023.7.22
accent_color: "#aaa"
cover:
  avif: https://cdn.mckayla.cloud/-/iv3ycbia418fvf/vm.avif
  default: https://cdn.mckayla.cloud/-/iv3ycbia418fvf/vm.webp
---

## Prelude

This isn't a comprehensive guide, but might prove useful! I wanted a resource to consult whenever spinning up a new VM to play with from providers like DigitalOcean or Hetzner. (Not sponsored or advocating for anything, those are just the two that I use personally.) I _vaguely_ remember the process, but end up looking up a bunch of the steps every time. The hope is that I'll no longer need to do that, and maybe neither will you!

## Setting up your user

I'm assuming here that you're using a cloud provider with sane defaults, like requiring key-based SSH authentication, and who will create an authorized_keys file for the root user.

First, we're going to log in as root to create our _actual_ user.

```sh
# Starting from your machine
ssh root@new.vm
# Now you should be connected to your VM!
YOU="your name here"
echo $YOU # Shells are weird, make sure this works!
adduser $YOU
mkdir /home/$YOU/.ssh/
chown $YOU /home/$YOU/.ssh/
chgrp $YOU /home/$YOU/.ssh/
cp /root/.ssh/authorized_keys /home/$YOU/.ssh/authorized_keys
chown $YOU /home/$YOU/.ssh/authorized_keys
chgrp $YOU /home/$YOU/.ssh/authorized_keys
usermod -a -G wheel $YOU # wheel allows you to run `sudo`
```

Now disconnect, and login as your new less-priviledged user

## Setting up your system

### RHEL/Rocky

- `sudo yum install gcc`
- `sudo yum install tar`
- `sudo yum install epel-release`
  - `sudo yum install htop`

## Getting ready for development

### SSH Key

You'll probably want an SSH key for Github and such.

```sh
ssh-keygen -t ed25519 -C "your@email.com"
cat ~/.ssh/id_ed25519.pub
```

### A folder for your stuff

```sh
sudo mkdir /Source/
# Fun fact: `$USER` persists through `sudo` (it will still be *your* username, not "root")
sudo chown $USER /Source/
sudo chgrp $USER /Source/
cd /Source/
```

### Rust

Just install [rustup](https://rustup.rs) the normal way.

### Helix (from source)

Requires Rust and a C compiler. Now that we have both, let's get our text editor set up.

```sh
cd /Source/
git clone git@github.com:helix-editor/helix.git
cd helix/
cargo install --path helix-term/
```

### Go

Figure out the URL of the latest tarball for your platform from the [Go downloads] page.

```sh
cd ~
wget $GO
tar -xzf go1.*
mv go/ .go/
```

Make sure you add `$HOME/.go/bin` to your PATH

[go downloads]: https://go.dev/dl/
