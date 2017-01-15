with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "derive-error";
  buildInputs = [ gcc ];
}
