class Subwasm < Formula
  desc "A command line utility written in Rust download, inspect and compare Substrate based chains WASM Runtimes"
  homepage "https://github.com/chevdor/subwasm"
  url "https://github.com/chevdor/subwasm/releases/download/v0.14.0/subwasm_macos_v0.14.0.tar.gz"
  sha256 "d418f72083024f4201a416093f86ad6494e7c2304bffaa41ded6c702826c6e27"
  version "0.14.0"

  def install
    bin.install "subwasm"
  end
end
