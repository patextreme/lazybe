{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        nightlyVersion = "2025-03-24";
        rust = pkgs.rust-bin.nightly.${nightlyVersion}.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
          ];
          targets = [ ];
        };
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rust;
          rustc = rust;
        };
      in
      {
        checks = {
          lazybe = rustPlatform.buildRustPackage {
            name = "lazybe-checks";
            cargoLock.lockFile = ./Cargo.lock;
            src = pkgs.lib.cleanSource ./.;
            dontBuild = true;
            installPhase = "touch $out";
            checkPhase = ''
              cargo fmt --check
              cargo clippy --all-features --all-targets -- -D warnings
              cargo b --all-features --all-targets
              cargo test --all-features
            '';
          };
        };

        apps = {
          bump = {
            type = "app";
            program =
              (pkgs.writeShellApplication {
                name = "bump";
                runtimeInputs = with pkgs; [
                  git-cliff
                  jq
                  rust
                  taplo
                ];
                text = ''
                  NEW_VERSION=$(git-cliff --bump --context | jq -r .[0].version | sed s/^v//)
                  NEW_VERSION_TAG="v$NEW_VERSION"

                  echo "Preparing a new version: $NEW_VERSION (tag: $NEW_VERSION_TAG)"
                  git-cliff --bump -o CHANGELOG.md
                  sed -i -E "s/^version = .*\# bump$/version = \"$NEW_VERSION\" # bump/" Cargo.toml
                  sed -i -E "s/^version = .*\# bump$/version = \"$NEW_VERSION\" # bump/" lazybe/Cargo.toml

                  find . | grep '\.toml$' | xargs -I _ bash -c "echo running taplo on _ && taplo format _"

                  cargo update lazybe-macros --precise "$NEW_VERSION"
                  cargo update lazybe --precise "$NEW_VERSION"
                  git add CHANGELOG.md
                  git add Cargo.lock
                  git add Cargo.toml
                  git add lazybe/Cargo.toml

                  printf "\nPlease verify if everything is ok, then run the following command ...\n"
                  echo "> git commit -m \"chore(release): prepare for $NEW_VERSION release\""
                '';
              }).outPath
              + "/bin/bump";
          };
        };

        devShells.default =
          let
            rootDir = "$ROOT_DIR";
            scripts =
              let
                localDb = {
                  port = 5432;
                  username = "postgres";
                  password = "postgres";
                  dbName = "postgres";
                };
              in
              {
                format = pkgs.writeShellScriptBin "format" ''
                  cd ${rootDir}
                  find ${rootDir} | grep '\.nix$' | xargs -I _ bash -c "echo running nixfmt on _ && ${pkgs.nixfmt-rfc-style}/bin/nixfmt _"
                  find ${rootDir} | grep '\.toml$' | xargs -I _ bash -c "echo running taplo on _ && ${pkgs.taplo}/bin/taplo format _"
                  ${rust}/bin/cargo fmt
                '';

                dbUp = pkgs.writeShellScriptBin "dbUp" ''
                  ${pkgs.docker}/bin/docker run \
                    -d --rm \
                    --name ${localDb.dbName} \
                    -e POSTGRES_DB=${localDb.dbName} \
                    -e POSTGRES_USER=${localDb.username} \
                    -e POSTGRES_PASSWORD=${localDb.password} \
                    -p ${toString localDb.port}:5432 postgres:16
                '';

                dbDown = pkgs.writeShellScriptBin "dbDown" ''
                  ${pkgs.docker}/bin/docker stop ${localDb.dbName}
                '';
              };
          in
          pkgs.mkShell {
            buildInputs =
              (with pkgs; [
                # base
                curl
                git
                git-cliff
                hurl
                jq
                less
                ncurses
                pkg-config
                watchexec
                which
                # rust
                cargo-edit
                cargo-expand
                rust
              ])
              ++ (builtins.attrValues scripts);

            shellHook = ''
              export ROOT_DIR=$(${pkgs.git}/bin/git rev-parse --show-toplevel)
              ${pkgs.cowsay}/bin/cowsay "Working on project root directory: ${rootDir}"
              cd ${rootDir}
            '';

            RUST_LOG = "info,lazybe::db=debug,sqlx::query=debug";
          };
      }
    );
}
