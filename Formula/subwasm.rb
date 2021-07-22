class Subwasm < Formula
  desc "A command line utility written in Rust download, inspect and compare Substrate based chains WASM Runtimes"
  homepage "https://github.com/chevdor/subwasm"
  url "https://github.com/chevdor/subwasm/releases/download/v0.13.2/subwasm_macos_v0.13.2.tar.gz"
  sha256 "76b6705a851a2fade15cadd70935140d9401455e2d680c5191ea17d0868df5dd"
  version "0.13.2"

  def install
    bin.install "subwasm"
  end
end
