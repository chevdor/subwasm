## Downloads

Download the binary for your OS from below:
- **Linux**: [Debian package]({{ DEBIAN_URL }})
- **MacOS**: [.tgz Archive]({{ MACOS_TGZ_URL }})
## Install

### From source

```
cargo install --git https://github.com/chevdor/subwasm
```

### Linux
```
wget {{ DEBIAN_URL }} -O subwasm.deb
sudo dpkg -i subwasm.deb
subwasm --help
```

### MacOS

```
brew tap chevdor/subwasm https://github.com/chevdor/subwasm
brew update
brew install chevdor/subwasm/subwasm
```

{{ CHANGELOG }}
