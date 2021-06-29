class Subwasm < Formula
  desc "CLI utility to get information about Substrate based chains Runtime WASM"
  homepage "https://github.com/chevdor/subwasm"
  url "https://github.com/chevdor/subwasm/releases/download/v0.11.0/subwasm_macos_v0.11.0.tar.gz"
  sha256 "5254ec365052936beeaa440a924a8f711d5d7fe7aa2cf46684094c71b8647b61"
  version "0.11.0"

  def install
    bin.install "subwasm"
  end
end
