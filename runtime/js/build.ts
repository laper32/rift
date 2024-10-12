import * as esbuild from "esbuild";
// import { aliasPath } from "esbuild-plugin-alias-path";
// import * as path from "node:path";

await esbuild.build({
  logLevel: "info",
  bundle: true,
  entryPoints: ["./src/index.ts"],
  format: "esm",
  outfile: "dist/index.js",
  minifyWhitespace: true,
  define: {
    global: "globalThis",
    "__filename": "\"\"",
  },
});
