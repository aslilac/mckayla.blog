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
# Alternatively,
# wget https://github.com/$YOUR_GITHUB_USER.keys -O /home/$YOU/.ssh/authorized_keys
# wget https://gitlab.com/$YOUR_GITLAB_USER.keys -O /home/$YOU/.ssh/authorized_keys
# wget https://meta.sr.ht/\~$YOUR_SOURCEHUT_USER.keys -O /home/$YOU/.ssh/authorized_keys
chown $YOU /home/$YOU/.ssh/authorized_keys
chgrp $YOU /home/$YOU/.ssh/authorized_keys
usermod -a -G wheel $YOU # adds you to the wheel group, wheel allows you to run `sudo`
```

Now disconnect, and login as your new less-priviledged user

## Setting up your system

### SSH Key

You'll probably want an SSH key for Github and such.

```sh
ssh-keygen -t ed25519 -C "your@email.com"
cat ~/.ssh/id_ed25519.pub # Paste the output of this into Github, etc.
```

One day I want to figure out how to use an ssh-agent so keys aren't just laying around. Today is not that day.

### A folder for your stuff

```sh
sudo mkdir /Source/
# Fun fact: `$USER` persists through `sudo` (it will still be *your* username, not "root")
sudo chown $USER /Source/
sudo chgrp $USER /Source/
cd /Source/
```

### RHEL/Rocky

Lately I've been using Rocky Linux for this sort of thing. So I'll cover some of my setup I usually do, but from here on out, this is going to be much less general. It will also get less general as you read further. We'll start with common things most people will want, and end up with some things that are really just for me.

---

Make sure you have the basics around! Feel free to filter out stuff you don't use. This is mostly just a list to act as a quick reference of common package names.

```sh
sudo yum install \
  cmake \
  curl \
  dash \
  gcc \
  git \
  glibc-devel \
  golang \
  make \
  sqlite \
  tar \
  tree \
  wget
```

Optional EPEL stuff:

```sh
sudo yum install epel-release
sudo yum update
sudo yum install \
  bat \
  htop
```

I'll also install Rust using the usual [rustup](https://rustup.rs) script. Once you've done that, I'll usually run these:

```sh
rustup install nightly
rustup component add rust-analyzer --toolchain stable
rustup component add rust-analyzer --toolchain nightly
```

If you installed Go, these are nice to have around:

```sh
go install golang.org/x/tools/gopls@latest
go install honnef.co/go/tools/cmd/staticcheck@latest
go install mvdan.cc/gofumpt@latest
```

...and you'll also want to configure some env variables...

```sh
export GOPATH="$HOME/.go"
export PATH="$GOPATH/bin:$PATH"
```

Helix is my terminal editor of choice.

```sh
sudo yum copr enable varlad/helix
sudo yum update
sudo yum install helix
```

Now we'll install Kirbo (an nvm and npm alternative) and some other tools I've made, but that anyone is welcome to use:

```sh
cargo install \
  cargo-update \ # I didn't make this one, but it helps keep the others up to date!
  betty \
  kc \
  kirbo \
  mkscratch \
  okie
```

Then install node with Kirbo!

```sh
# TODO
```

At this point, we've got a machine that can provide a pretty well formed workstation for Go, Rust, and JavaScript/TypeScript code!

As a bit of an aside: unfortunately, as much as I love Gleam, Rocky doesn't currently have packages for Erlang, Rebar3, Elixir, or Gleam itself. Setting up a COPR Gleam package is currently on my to-do list, since I use Fedora whenever I want a Linux install with a desktop, and that would also stand to benefit from the work. I'll update this once I've got a working "Gleam on Rocky" setup to share.

Some tweaks to your `PATH`, maybe a few more packages of your choosing, and you should have a mostly functional dev machine!
