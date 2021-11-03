class Subwasm < Formula
  desc "A command line utility written in Rust download, inspect and compare Substrate based chains WASM Runtimes"
  homepage "https://github.com/chevdor/subwasm"
  url "https://github.com/chevdor/subwasm/releases/download/v0.16.0/subwasm_macos_v0.16.0.tar.gz"
  sha256 "56752df1598acdefd3131d0349169e6c1834d9654765a2a0d9f6858338437278"
  version "0.16.0"

  def install
    bin.install "subwasm"
  end
end
