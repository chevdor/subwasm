class Subwasm < Formula
  desc "A command line utility written in Rust download, inspect and compare Substrate based chains WASM Runtimes"
  homepage "https://github.com/chevdor/subwasm"
  url "https://github.com/chevdor/subwasm/releases/download/v0.21.2/subwasm_macos_v0.21.2.tar.gz"
  sha256 "8560eacdfb9b446958704ca010986f38a39fd71942a141284faed9ac64e49974"
  version "0.21.2"

  def install
    bin.install "subwasm"
  end
end
