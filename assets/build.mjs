import esbuild from "esbuild";
import { wasmLoader } from "esbuild-plugin-wasm";
import path from "path";

const args = process.argv.slice(2);
const watch = args.includes('--watch');
const deploy = args.includes('--deploy');

const loader = {
    ".js": "jsx",
    ".wasm": "binary"
};

const plugins = [
    wasmLoader()
];

let opts = {
    entryPoints: ["js/app.jsx"],
    bundle: true,
    logLevel: "info",
    target: "es2022",
    format: "esm",
    outdir: "../priv/static/assets",
    external: ["*.css", "fonts/*", "images/*"],
    nodePaths: ["../deps"],
    loader: loader,
    plugins: plugins
};

if (deploy) {
    opts = {
        ...opts,
        minify: true
    };
}

if (watch) {
    opts = {
        ...opts,
        sourcemap: "inline",
    };

    esbuild
        .context(opts)
        .then((ctx) => {
            ctx.watch();
        })
        .catch((_error) => {
            process.exit(1);
        });
} else {
    esbuild.build(opts);
}
