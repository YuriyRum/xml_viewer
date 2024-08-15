import init, { convert_to_html } from "./pkg/example_without_custom_component.js";
window.convert_to_html = convert_to_html;

let resolveFn;

window._completer = new Promise(resolve => {
    resolveFn = resolve;
});

const runWasm = async () => {
    await init("./pkg/example_without_custom_component_bg.wasm");
    resolveFn();
};
runWasm();