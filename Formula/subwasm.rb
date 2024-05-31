class Subwasm < Formula
  desc "A command line utility written in Rust download, inspect and compare Substrate based chains WASM Runtimes"
  homepage "https://github.com/chevdor/subwasm"
  url "https://github.com/chevdor/subwasm/releases/download/v0.21.3/subwasm_macos_v0.21.3.tar.gz"
  sha256 "c39065fccc8b20270232e3b12dc298e7dca1b129545a02e9188f9a1535041d3f"
  version "0.21.3"

  def install
    bin.install "subwasm"
  end
end
