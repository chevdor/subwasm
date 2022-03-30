class Subwasm < Formula
  desc "A command line utility written in Rust download, inspect and compare Substrate based chains WASM Runtimes"
  homepage "https://github.com/chevdor/subwasm"
  url "https://github.com/chevdor/subwasm/releases/download/v0.17.0/subwasm_macos_v0.17.0.tar.gz"
  sha256 "61d34155b83961f1a299e8e853db0dc1987c1e308e48bc8bee51ef7a0e454bd5"
  version "0.17.0"

  def install
    bin.install "subwasm"
  end
end
