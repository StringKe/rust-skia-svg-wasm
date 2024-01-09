EMSDK ?= ~/.local/share/mise/installs/emsdk/$(shell mise current emsdk | tr -s ' ' | cut -d ' ' -f 2)

# 定义一个 array
EXPORTED_FUNCTIONS = '_malloc','_free','_main','_init_font_provider','_get_font_family_names','_check_font_exists','_registry_font','_get_svg_path'
EXPORTED_RUNTIME_METHODS = 'ccall','cwrap','allocateUTF8','getValue','setValue','UTF8ToString','stringToUTF8'

BUILD = EMCC_CFLAGS="-sEXPORT_ALL -sEXPORTED_FUNCTIONS=[$(EXPORTED_FUNCTIONS)] -sEXPORTED_RUNTIME_METHODS=[$(EXPORTED_RUNTIME_METHODS)] -sASSERTIONS=1 -sSAFE_HEAP=0 -sERROR_ON_UNDEFINED_SYMBOLS=0 -sALLOW_MEMORY_GROWTH=1 -sABORTING_MALLOC=1 -sMODULARIZE=1 -sEXPORT_NAME='initModule' -sENVIRONMENT='web'" EMSDK=$(EMSDK) RUST_BACKTRACE=1 cargo build --target wasm32-unknown-emscripten

.PHONY: build clean rebuild serve

build:
	$(BUILD) --release

clean:
	rm -rf ../target/wasm32-unknown-emscripten/release/build

rebuild: clean build

serve:
	pnpx http-server -p 8080 -c-1 ../
