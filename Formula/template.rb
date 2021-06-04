class Subwasm < Formula
    desc "CLI utility to get information about Substrate based chains Runtime WASM"
    homepage "https://github.com/chevdor/subwasm"
    url "https://github.com/chevdor/subwasm/releases/download/v{{ version }}/subwasm-macos-v{{ version }}.tar.gz"
    sha256 "{{ sha256 }}"
    version "{{ version }}"

    def install
      bin.install "subwasm"
    end
  end
