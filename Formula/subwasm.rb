class Subwasm < Formula
  desc "A command line utility written in Rust download, inspect and compare Substrate based chains WASM Runtimes"
  homepage "https://github.com/chevdor/subwasm"
  url "https://github.com/chevdor/subwasm/releases/download/v0.16.1/subwasm_macos_v0.16.1.tar.gz"
  sha256 "a16821c02737dd83ceb08c742d6a0d2cbfd368a690d9936e236dd52efc82e060"
  version "0.16.1"

  def install
    bin.install "subwasm"
  end
end
