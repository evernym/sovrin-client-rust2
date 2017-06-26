# sovrin-client-rust

# Windows build

## Get/build dependencies

All prebuilt can be downloaded from
https://repo.evernym.com/deb/windows-bins/indy-sdk-deps/

### Binary deps

- https://www.npcglib.org/~stathis/downloads/openssl-1.0.2k-vs2017.7z
- https://download.libsodium.org/libsodium/releases/libsodium-1.0.12-msvc.zip

### Source deps

- http://www.sqlite.org/2017/sqlite-amalgamation-3180000.zip
- https://github.com/miracl/milagro-crypto-c/
- https://github.com/evernym/libzmq-pw

### Build sqlite

Download http://www.sqlite.org/2017/sqlite-amalgamation-3180000.zip

Create empty static library project and add sqlite.c file and 2 headers from exctraced
archive. Then just build it.

### Build milagro-crypto-c

Checkout https://github.com/miracl/milagro-crypto-c/ repository.
- cmake -DBUILD_SHARED_LIBS=OFF -DCMAKE_BUILD_TYPE=Release -DCMAKE_POSITION_INDEPENDENT_CODE=ON -G "Visual Studio 15 2017 Win64" .
- open AMCL.sln
- disable custom build steps
- build it

### Build libzmq-pw

Checkout https://github.com/evernym/libzmq-pw repository.
- build builds/msvc/vs2017/libzmq.sln (it may print errors while
  building tests which can be ignored)

## Build

- Get binary dependencies (libamcl*, openssl, libsodium, libzmq, sqlite3).
- Put all *.{lib,dll} into one directory and headers into include/ subdirectory.
- open MSVS development console
- execute "C:\Program Files (x86)\Microsoft Visual Studio\2017\Community\VC\Auxiliary\Build\vcvars64.bat"
- Point path to this directory using environment variables:
  - set SOVRIN_PREBUILT_DEPS_DIR=C:\BIN\x64
  - set SODIUM_LIB_DIR=C:\BIN\x64
  - set OPENSSL_INCLUDE_DIR=C:\BIN\x64\include
  - set OPENSSL_LIB_DIR=C:\BIN\x64
  - set LIBZMQ_LIB_DIR=C:\BIN\x64
  - set LIBZMQ_INCLUDE_DIR=C:\BIN\x64\include
- set static flag for libsodium build
  - set SODIUM_STATIC=y
- change dir to sovrin-client and run cargo (you may want to add --release --target x86_64-pc-windows-msvc keys to cargo)

## openssl-sys workaround

When your windows build fails complaining on gdi32.lib you should edit

```
  ~/.cargo/registry/src/github.com-*/openssl-sys-*/build.rs
```

and add

```
  println!("cargo:rustc-link-lib=dylib=gdi32");
```

to the end of main() function.

Then try to rebuild whole project.
