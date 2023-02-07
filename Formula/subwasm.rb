class Subwasm < Formula
  desc "A command line utility written in Rust download, inspect and compare Substrate based chains WASM Runtimes"
  homepage "https://github.com/chevdor/subwasm"
  url "https://github.com/chevdor/subwasm/releases/download/v0.19.0/subwasm_macos_v0.19.0.tar.gz"
  sha256 "7c5a9e09ef2a56990a02453f180206546580c837f7db5e1fdc1e953bb4829e2c"
  version "0.19.0"

  def install
    bin.install "subwasm"
  end
end
