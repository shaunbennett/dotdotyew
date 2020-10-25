# dotdotyew

### Compiling

I use [trunk](https://github.com/thedodd/trunk) to build and package the assets. View the repo for installation instructions.


Once installed, you can build the project. The output files are placed in `dist/`
```
trunk build
```

### Running

Trunk can host your files and rebuild whenever you make a change.

```
trunk serve
```

### Production

Trunk does not run any wasm optimization tools like `wasm-snip` or `wasm-opt`. Instead, I manually run these on the output wasm file before deploying to production.
