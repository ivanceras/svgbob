# Svgbob

<a href="https://crates.io/crates/svgbob"><img src="https://img.shields.io/crates/v/svgbob.svg" alt="svgbob’s current version badge" title="svgbob’s current version badge"></a><a href="https://travis-ci.org/ivanceras/svgbob">
<img src="https://api.travis-ci.org/ivanceras/svgbob.svg"/>
</a>

Svgbob is a utility tool and library for generating nice graphical representation from simple ascii diagrams.
It uses a combination of characters which are readily available in your keyboards.


Example ascii diagrams:

Rectangles:
```txt
+-------+    .--------.
|       |    |        |
+-------+    `--------'
```

Circle and arcs:
```txt

    ____            __  __
  ,'    `.        ,'      `.
 /        \      /          \
 \        /
  `.____.'       \          /
                  `.__  __.'

```


Interactive demo with more examples can be found [**here**](https://ivanceras.github.io/svgbob-editor/)


[Specification](https://ivanceras.github.io/content/Svgbob/Specification.html)


## TODO
- Support for pills with variable radius for the rounded corner

```txt

    _____________
  ,'             `.
 /   This is a     \
 \      pill       /
  `._____________.'


    _____________
  ,'             `.
 /   This is a     \
 |                 |
 |       v         |
 |       e         |
 |       r         |
 |       t         |
 |       i         |
 |       c         |
 |       a         |
 |       l         |
 |                 |
 \      pill       /
  `._____________.'
```
<a href="https://liberapay.com/ivanceras/donate"><img alt="Donate using Liberapay" src="https://liberapay.com/assets/widgets/donate.svg"></a>
