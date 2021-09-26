# PSP.js
Modern JavaScript runtime for Sony PSP, based on rust-psp and QuickJS.

> ⚠️ Currently in PoC state, unusable for developing JavaScript apps yet.

## Build
Clone the repo:

``` sh
git clone https://github.com/doodlewind/psp-js.git
cd psp-js
git submodule update --init
```

Then download and unzip the [prebuilt PSPSDK](https://github.com/doodlewind/psp-test-app/releases/download/sdk/mipsel-sony-psp.zip) (built from [clang-psp](https://github.com/pspdev/clang-psp)) into the project root, build the runtime:

``` sh
cd runtime
./build.sh
```

The output app file is `runtime/target/mipsel-sony-psp/debug/EBOOT.PBP`.

## License
MIT
