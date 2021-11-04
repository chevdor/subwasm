class Subwasm < Formula
  desc "A command line utility written in Rust download, inspect and compare Substrate based chains WASM Runtimes"
  homepage "https://github.com/chevdor/subwasm"
  url "https://github.com/chevdor/subwasm/releases/download/v0.16.1/subwasm_macos_v0.16.1.tar.gz"
  sha256 "7cdfac41219bc40e89a67ea259b23b65ca08f7ed7e70bacdb029a268f0df3127"
  version "0.16.1"

  def install
    bin.install "subwasm"
  end
end
