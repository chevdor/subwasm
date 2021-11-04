class Subwasm < Formula
  desc "A command line utility written in Rust download, inspect and compare Substrate based chains WASM Runtimes"
  homepage "https://github.com/chevdor/subwasm"
  url "https://github.com/chevdor/subwasm/releases/download/v0.16.1/subwasm_macos_v0.16.1.tar.gz"
  sha256 "b9036e5229d7ba738200fa165b7c874ac57310746e02f6f7e2b1c8a9b8910fe5"
  version "0.16.1"

  def install
    bin.install "subwasm"
  end
end
