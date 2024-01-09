function innerInitFontProvider(Module) {
    return Module._init_font_provider();
}

function innerRegistryFont(Module, fontProvider, fontFamily, fontDataArrayBuffer) {
    const fontFamilyPtr = Module.allocateUTF8(fontFamily);
    const fontDataPtr = Module._malloc(fontDataArrayBuffer.byteLength);
    Module.HEAPU8.set(new Uint8Array(fontDataArrayBuffer), fontDataPtr);

    const result = Module._registry_font(fontProvider, fontFamilyPtr, fontFamily.length, fontDataPtr, fontDataArrayBuffer.byteLength);

    Module._free(fontFamilyPtr);
    Module._free(fontDataPtr);

    return result;
}

function innerGetFontFamilyNames(Module, fontProvider) {
    const namesPtr = Module._get_font_family_names(fontProvider);

    const names = Module.UTF8ToString(namesPtr);
    Module._free(namesPtr);

    return names.split("\n");
}

function innerCheckFontExist(Module, fontProvider, fontFamily) {
    const fontFamilyPtr = Module.allocateUTF8(fontFamily);
    return Module._check_font_exists(fontProvider, fontFamilyPtr, fontFamily.length);
}


initModule().then((Module) => {
    function withModule(fn) {
        return function (...args) {
            return fn(Module, ...args);
        }
    }

    function withReturnBoolean(fn) {
        return function (...args) {
            return Boolean(fn(...args));
        }
    }

    const initFontProvider = withModule(innerInitFontProvider);

    const fontProvider = initFontProvider();

    function withFontProvider(fn) {
        return function (...args) {
            return fn(fontProvider, ...args);
        }
    }


    const registryFont = withReturnBoolean(withFontProvider(withModule(innerRegistryFont)));
    const getFontFamilyNames = withFontProvider(withModule(innerGetFontFamilyNames));
    const checkFontExist = withReturnBoolean(withFontProvider(withModule(innerCheckFontExist)));


    fetch("./test.ttf").then((res) => res.arrayBuffer()).then((fontDataArrayBuffer) => {
        console.log("Module", Module)

        const fontFamily = "test";
        const result = registryFont(fontFamily, fontDataArrayBuffer);

        console.log("registryFont", result);
        const names = getFontFamilyNames();
        console.log("getFontFamilyNames", names);

        const exist = checkFontExist(fontFamily);
        console.log("checkFontExist", exist);
        // console.log("getFontFamilyNames", getFontFamilyNames());
        // console.log("checkFontExist", checkFontExist(fontFamily));
    });
});
