ifdef OS
	TOOLCHAIN = +stable-i686-pc-windows-msvc
	BINARYNAME = samp_rng.dll
	OUTPUTNAME = samp_rng.dll
	CP_RELEASE = cp .\target\release\${BINARYNAME} .\plugins\${OUTPUTNAME}
	CP_DEBUG = cp .\target\debug\$(BINARYNAME) .\plugins\$(OUPUTNAME)
else
	ifeq ($(shell uname), Linux)
		TOOLCHAIN = +stable-i686-unknown-linux-gnu
		BINARYNAME = libsamp_rng.so
		OUPUTNAME = samp_rng.so
		CP_RELEASE = cp target/release/$(BINARYNAME) plugins/$(OUPUTNAME)
		CP_DEBUG = cp target/debug/$(BINARYNAME) plugins/$(OUPUTNAME)
	endif
endif

release:
	cargo $(TOOLCHAIN) build --release
	${CP_RELEASE}

debug:
	cargo $(TOOLCHAIN) build
	${CP_DEBUG}

clean:
	cargo clean