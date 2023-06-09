class Subwasm < Formula
  desc "A command line utility written in Rust download, inspect and compare Substrate based chains WASM Runtimes"
  homepage "https://github.com/chevdor/subwasm"
  url "https://github.com/chevdor/subwasm/releases/download/v1.0.0-alpha9/subwasm_macos_v1.0.0-alpha9.tar.gz"
  sha256 "dba2a8ee8efd85a576a1d4e9764608458bdb6a21a42f1406d025fcbcaf381449"
  version "1.0.0-alpha9"

  def install
    bin.install "subwasm"
  end
end
