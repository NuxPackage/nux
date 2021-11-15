with (import <nixpkgs> {});

rustPlatform.buildRustPackage rec {
  pname = "nux";
  version = "0.1.0";

  src = fetchFromGitHub {
    owner = "NuxPackage";
    repo = pname;
    rev = version;
    sha256 = "sha256-vjB4h134UHmeQmSoOQcHmnb26MH0TFGD/JWycsjVSIk=";
  };

  cargoSha256 = "sha256-pmHjijvM8evSA1XseAclKEmM5/M4s3h8GRTJr3typL8=";

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
