class Subwasm < Formula
  desc "A command line utility written in Rust download, inspect and compare Substrate based chains WASM Runtimes"
  homepage "https://github.com/chevdor/subwasm"
  url "https://github.com/chevdor/subwasm/releases/download/v0.15.0/subwasm_macos_v0.15.0.tar.gz"
  sha256 "dd933b09335b7710c7944620197e4a95db65a0ddbccacda2692f529ec6636656"
  version "0.15.0"

  def install
    bin.install "subwasm"
  end
end
