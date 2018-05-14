with import <nixpkgs> {
  overlays = [ (import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz)) ];
};

stdenv.mkDerivation {
  name = "efm32gg11b820";
  buildInputs = [
    rustChannels.stable.rust
  ];
}
