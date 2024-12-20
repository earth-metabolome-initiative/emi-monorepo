{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Basic build inputs
    zlib
    readline
    openssl
    libxml2
    icu
    libuuid

    # LLVM for JIT support
    llvmPackages.llvm
    llvmPackages.llvm.dev

    # Compression libraries
    lz4
    zstd

    # System libraries
    systemd
    linux-pam

    # Python support
    python3

    # Kerberos support
    libkrb5

    # Native build inputs
    makeWrapper
    pkg-config
    removeReferencesTo
    nukeReferences

    # Documentation and parsing tools
    bison
    flex
    perl
    docbook_xml_dtd_45
    docbook-xsl-nons
    libxslt
    rustup
    cargo

    glibc
    glibc.dev
    clang
  ];

  # Set up environment variables if needed
  shellHook = ''
    export PGDATA="$PWD/data"
    export PGHOST="$PWD/run"
    export C_INCLUDE_PATH=${pkgs.glibc.dev}/include
    export LIBCLANG_PATH=${pkgs.llvmPackages.libclang.lib}/lib
  '';
}