class Subwasm < Formula
  desc "A command line utility written in Rust download, inspect and compare Substrate based chains WASM Runtimes"
  homepage "https://github.com/chevdor/subwasm"
  url "https://github.com/chevdor/subwasm/releases/download/v0.14.1/subwasm_macos_v0.14.1.tar.gz"
  sha256 "5d4af2428d28ab1a15db45e45be11aadae6667de9b7da28fdbf7e95fde4e7cd9"
  version "0.14.1"

  def install
    bin.install "subwasm"
  end
end
