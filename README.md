# Svgbob

<a href="https://crates.io/crates/svgbob"><img src="https://img.shields.io/crates/v/svgbob.svg" alt="svgbob’s current version badge" title="svgbob’s current version badge"></a><a href="https://travis-ci.org/ivanceras/svgbob">
<img src="https://api.travis-ci.org/ivanceras/svgbob.svg"/>
</a>

Svgbob can create a nice graphical representation of your text diagrams.

Svgbob provides a cli which takes text as an input and creates an svg image as an output.

[**Demo**](https://ivanceras.github.io/svgbob-editor/)



[Specification](https://ivanceras.github.io/content/Svgbob/Specification.html)

<a href="https://liberapay.com/ivanceras/donate"><img alt="Donate using Liberapay" src="https://liberapay.com/assets/widgets/donate.svg"></a>

## Getting started

```powershell
# To try svgbob without installation run via Docker
docker build -t svgbob:local .
docker run --rm svgbob:local svgbob_cli -s  "
                                    .-------------------.
                                    |   Hello svgbob !  |
                                    '-------------------'

           .-.           .-.           .-.           .-.           .-.           .-.
          |   |         |   |         |   |         |   |         |   |         |   |
       .---------.   .--+---+--.   .--+---+--.   .--|   |--.   .--+   +--.   .------|--.
      |           | |           | |   |   |   | |   |   |   | |           | |   |   |   |
       '---------'   '--+---+--'   '--+---+--'   '--|   |--'   '--+   +--'   '--|------'
          |   |         |   |         |   |         |   |         |   |         |   |
           '-'           '-'           '-'           '-'           '-'           '-'
" > ./example.svg
```
