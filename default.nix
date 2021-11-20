{ lib, stdenv
, fetchFromGitHub
, rustPlatform
, asciidoctor
, installShellFiles
, pkg-config
, Security
}:

rustPlatform.buildRustPackage rec {
  pname = "nux";
  version = "0.1.3";

  src = fetchFromGitHub {
    owner = "NuxPackage";
    repo = pname;
    rev = version;
    sha256 = lib.fakeHash;
  };

  cargoSha256 = "sha256-ztWYRFNwrWZPGKIujplQdiDSlLtgPy14DmbkdIPPqSQ=";

  preFixup = ''
    installManPage $releaseDir/build/nux-*/out/nux.1
    installShellCompletion $releaseDir/build/nux-*/out/nux.{bash,fish}
    installShellCompletion $releaseDir/build/nux-*/out/_nux

  '';
  nativeBuildInputs = [ asciidoctor installShellFiles ];
  buildInputs = [pkg-config];

  meta = with lib; {
    description = "A wrapper over the nix cli.";
    homepage = "https://github.com/NuxPackage/nux";
    license = with licenses; [ gpl3 ];
    maintainers = with maintainers; [ drzoidberg ];
    mainProgram = "nux";
  };
}
