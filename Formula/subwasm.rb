class Subwasm < Formula
  desc "A command line utility written in Rust download, inspect and compare Substrate based chains WASM Runtimes"
  homepage "https://github.com/chevdor/subwasm"
  url "https://github.com/chevdor/subwasm/releases/download/v0.13.0/subwasm_macos_v0.13.0.tar.gz"
  sha256 "38dab03da8bdc741694072e682d10b0fa52bc230f626f227d9d5edf34ec85da3"
  version "0.13.0"

  def install
    bin.install "subwasm"
  end
end
