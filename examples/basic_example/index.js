import init from "./pkg/basic_example.js";

const runWasm = async () => {
    await init("./pkg/basic_example_bg.wasm");
};
runWasm();