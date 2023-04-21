class Subwasm < Formula
  desc "A command line utility written in Rust download, inspect and compare Substrate based chains WASM Runtimes"
  homepage "https://github.com/chevdor/subwasm"
  url "https://github.com/chevdor/subwasm/releases/download/v0.19.1/subwasm_macos_v0.19.1.tar.gz"
  sha256 "a23ce6b27f59f63dc6ae417f3a99f5ee788648e66a647dad94b3fe5f3ef760d8"
  version "0.19.1"

  def install
    bin.install "subwasm"
  end
end
