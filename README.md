# dotdotyew

### Compiling

```
wasm-pack build --target web --out-name wasm --out-dir ./static
```

### Running

You just need to host a web server over the static directory. I use [http-server](https://www.npmjs.com/package/http-server).

```
http-server ./static -p 9999
```

