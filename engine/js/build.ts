import * as esbuild from "esbuild";


await esbuild.build({
  logLevel: "info",
  bundle: true,
  entryPoints: ["./src/index.ts"],
  format: "cjs",
  outfile: "dist/index.js",
  minifyWhitespace: true,
  define: {
    global: "globalThis",
    "__filename": "\"\"",
  },
});
