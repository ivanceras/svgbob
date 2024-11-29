# Svgbob server

This is running svgbob as a rest api

```sh
cargo install svgbob_server
PORT=3000 svgbob_server
```

Then you can use svgbob in port 3000 to convert text diagrams into svg

```sh
curl -X POST -F 'ascii=o------>' http://localhost:3000 > output.svg
```
