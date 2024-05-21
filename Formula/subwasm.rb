class Subwasm < Formula
  desc "A command line utility written in Rust download, inspect and compare Substrate based chains WASM Runtimes"
  homepage "https://github.com/chevdor/subwasm"
  url "https://github.com/chevdor/subwasm/releases/download/v0.21.1/subwasm_macos_v0.21.1.tar.gz"
  sha256 "f773fd1ad7e8d755669943d94190702ccd5a0723bfcd6f17dfd2a34863726602"
  version "0.21.1"

  def install
    bin.install "subwasm"
  end
end
