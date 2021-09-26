export CRATE_CC_NO_DEFAULTS=1
export TARGET_CFLAGS="-target mips -mcpu=mips2 -msingle-float -mlittle-endian -mno-check-zero-division"
export TARGET_CC=clang
export PATH=/usr/local/opt/llvm/bin:${PATH}

cargo psp
