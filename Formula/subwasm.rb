class Subwasm < Formula
  desc "A command line utility written in Rust download, inspect and compare Substrate based chains WASM Runtimes"
  homepage "https://github.com/chevdor/subwasm"
  url "https://github.com/chevdor/subwasm/releases/download/v0.13.1/subwasm_macos_v0.13.1.tar.gz"
  sha256 "64f9a01d84b99f39d5ec27b4c6ca0368d30cf2c72bd2edd4738a56bb4e33c6b3"
  version "0.13.1"

  def install
    bin.install "subwasm"
  end
end
