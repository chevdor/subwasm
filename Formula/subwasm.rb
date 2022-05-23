class Subwasm < Formula
  desc "A command line utility written in Rust download, inspect and compare Substrate based chains WASM Runtimes"
  homepage "https://github.com/chevdor/subwasm"
  url "https://github.com/chevdor/subwasm/releases/download/v0.17.1/subwasm_macos_v0.17.1.tar.gz"
  sha256 "a72a03c37a225eaf5adb4d9bc4daff12b835afe14e9e0f66990d8d352d9e1617"
  version "0.17.1"

  def install
    bin.install "subwasm"
  end
end
