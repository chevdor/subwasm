class Subwasm < Formula
  desc "A command line utility written in Rust download, inspect and compare Substrate based chains WASM Runtimes"
  homepage "https://github.com/chevdor/subwasm"
  url "https://github.com/chevdor/subwasm/releases/download/v0.20.0/subwasm_macos_v0.20.0.tar.gz"
  sha256 "481404fd91398e8d23b7946b940a61c9b5e79fcb788cb17d4c91550a22ae874b"
  version "0.20.0"

  def install
    bin.install "subwasm"
  end
end
