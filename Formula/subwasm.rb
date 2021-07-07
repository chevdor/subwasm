class Subwasm < Formula
  desc "A command line utility written in Rust download, inspect and compare Substrate based chains WASM Runtimes"
  homepage "https://github.com/chevdor/subwasm"
  url "https://github.com/chevdor/subwasm/releases/download/v0.12.1/subwasm_macos_v0.12.1.tar.gz"
  sha256 "a34068aba5a4d75aad1d435e542ff184285a9fc03f3f7538a7be07ca2560db0b"
  version "0.12.1"

  def install
    bin.install "subwasm"
  end
end
