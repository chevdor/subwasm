class Subwasm < Formula
  desc "A command line utility written in Rust download, inspect and compare Substrate based chains WASM Runtimes"
  homepage "https://github.com/chevdor/subwasm"
  url "https://github.com/chevdor/subwasm/releases/download/v1.0.0-alpha7/subwasm_macos_v1.0.0-alpha7.tar.gz"
  sha256 "732f1abec2ea7277ac1467e4f07060d22bdda00a7a09222cb5a9ba6f80e3f6e4"
  version "1.0.0-alpha7"

  def install
    bin.install "subwasm"
  end
end
