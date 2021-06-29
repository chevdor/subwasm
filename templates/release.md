# Description

You can find the changelogs below.

# Downloads

Download the binary for your OS from below:
- **Linux**
    - [Debian package]({{ DEBIAN_URL }})
- **MacOS**
    - [Archive]({{ MACOS_TGZ_URL }})
# Install

## From source

```
cargo install --git https://github.com/chevdor/subwasm
```

## Linux
```
wget {{ DEBIAN_URL }}
sudo dpkg -i subwasm_linux_amd64*.deb
subwasm --help
```

## MacOS

```
brew tap chevdor/subwasm https://github.com/chevdor/subwasm
brew update
brew install chevdor/subwasm/subwasm
```

{{ CHANGELOG }}
