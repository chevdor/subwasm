class Subwasm < Formula
  desc "CLI utility to get information about Substrate based chains Runtime WASM"
  homepage "https://gitlab.com/chevdor/subwasm"
  url "https://github.com/chevdor/subwasm/releases/download/v0.10.0/subwasm-mac-v0.10.0.tar.gz"
  sha256 "b14fcc9e9629ca203c25a9f70f271740f461085bc46d9711bb2a7d88c6a9c8e0"
  version "0.10.0"

  def install
    bin.install "subwasm"
  end
end
