{ project ? import ./nix { }
}:

# By default we want to have executables that have the correct "tendermint" and
# "anoma*" executables available in PATH no matter how the user calls anoma etc.
project.pkgs.runCommandNoCC "anoma-apps" { nativeBuildInputs = [ project.pkgs.makeWrapper ]; } ''
  mkdir -p $out/bin
  for p in ${project.apps}/bin/*; do
    makeWrapper $p $out/bin/$(basename $p) \
      --prefix PATH : ${project.apps}/bin \
      --prefix PATH : ${project.pkgs.tendermint}/bin
  done
''
