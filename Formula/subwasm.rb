class Subwasm < Formula
  desc "A command line utility written in Rust download, inspect and compare Substrate based chains WASM Runtimes"
  homepage "https://github.com/chevdor/subwasm"
  url "https://github.com/chevdor/subwasm/releases/download/v0.21.0/subwasm_macos_v0.21.0.tar.gz"
  sha256 "1c844208e04a3fed4089c9d9cf35b43c74ac8a875644a651aa7be811816633a4"
  version "0.21.0"

  def install
    bin.install "subwasm"
  end
end
