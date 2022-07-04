class Subwasm < Formula
  desc "A command line utility written in Rust download, inspect and compare Substrate based chains WASM Runtimes"
  homepage "https://github.com/chevdor/subwasm"
  url "https://github.com/chevdor/subwasm/releases/download/v0.18.0/subwasm_macos_v0.18.0.tar.gz"
  sha256 "20dc91978528ba09111543e3f07a24725a4a118ce252343ca55bdc8d2a60a7aa"
  version "0.18.0"

  def install
    bin.install "subwasm"
  end
end
