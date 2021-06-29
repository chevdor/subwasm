class Subwasm < Formula
  desc "A command line utility written in Rust download, inspect and compare Substrate based chains WASM Runtimes"
  homepage "https://github.com/chevdor/subwasm"
  url "https://github.com/chevdor/subwasm/releases/download/v0.12.0/subwasm_macos_v0.12.0.tar.gz"
  sha256 "c9c0d85cf91d1b3f0ceef1b66513ac552cc9fab48aef76c5f23fb61ae6dbf48f"
  version "0.12.0"

  def install
    bin.install "subwasm"
  end
end
