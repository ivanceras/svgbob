# SvgBobRus

<img src="https://img.shields.io/crates/v/svgbob.svg" alt="svgbob’s current version badge" title="svgbob’s current version badge">
[![Build Status](https://api.travis-ci.org/ivanceras/svgbobrus.svg)](https://travis-ci.org/ivanceras/svgbobrus)

SvgBobRus is an ascii to svg converter.

[Docs](https://docs.rs/svgbob)

<img src="https://ivanceras.github.io/svgbobrus/svgbob/examples/demo.svg"/>

The SVG image is generated from the ascii text below.

```text


+------+   +-----+   +-----+   +-----+
|      |   |     |   |     |   |     |
| Foo  +-->| Bar +---+ Baz |<--+ Moo |
|      |   |     |   |     |   |     |
+------+   +-----+   +--+--+   +-----+
              ^         |
              |         V
.-------------+-----------------------.
| Hello here and there and everywhere |
'-------------------------------------'


                        ____________
   .--------------.     \           \
  / a == b         \     \           \     __________
 (    &&            )     ) process   )    \         \
  \ 'string' ne '' /     /           /     / process /
   '--------------'     /___________/     /_________/


    __________________
    \_________________\
     \                 \
      . another process .
     /_________________/
    /_________________/

  User code  ^               ^ OS code
              \             /
               \        .--'
                \      /
  User code  <--- Mode ----> OS code
                /      \
            .--'        \___
           /                \
          v                  v 
       User code            OS code

             .---.  .---. .---.  .---.    .---.  .---.
    OS API   '---'  '---' '---'  '---'    '---'  '---'
               |      |     |      |        |      |
               v      v     |      v        |      v
             .------------. | .-----------. |  .-----.
             | Filesystem | | | Scheduler | |  | MMU |
             '------------' | '-----------' |  '-----'
                    |       |      |        |
                    v       |      |        v
                 .----.     |      |    .---------.
                 | IO |<----'      |    | Network |
                 '----'            |    '---------'
                    |              |         |
                    v              v         v
             .---------------------------------------.
             |                  HAL                  |
             '---------------------------------------'
             

   ____[]
  | ___ |
  ||   ||  device
  ||___||  loads
  | ooo |----------------------------------------------------------.
  | ooo |    |                          |                          |
  | ooo |    |                          |                          |
  '-----'    |                          |                          |
             |                          |                          |
             v                          v                          v
   .-------------------.  .---------------------------.  .-------------------.
   | Loadable module C |  |     Loadable module A     |  | Loadable module B |
   '-------------------'  |---------------------------|  |   (instrumented)  |
             |            |         .-----.           |  '-------------------'
             '------------+-------->| A.o |           |             |
                 calls    |         '-----'           |             |
                          |    .------------------.   |             |
                          |   / A.instrumented.o /<---+-------------'
                          |  '------------------'     |    calls
                          '---------------------------'   

        .--------------.
         \              \
          '--------------'

                                        .--> Base::Class::Derived_A
                                       /
                                      .----> Base::Class::Derived_B    
      Something -------.             /         \
                        \           /           .---> Base::Class::Derived
      Something::else    \         /             \
            \             \       /               '--> Base::Class::Derived
             \             \     /
              \             \   .-----------> Base::Class::Derived_C 
               \             \ /
                '------ Base::Class
                       /  \ \ \
                      '    \ \ \  
                      |     \ \ \
                      .      \ \ '--- The::Latest
                     /|       \ \      \
 With::Some::fantasy  '        \ \      '---- The::Latest::Greatest
                     /|         \ \
         More::Stuff  '          \ '- I::Am::Running::Out::Of::Ideas
                     /|           \
         More::Stuff  '            \
                     /              '--- Last::One
       More::Stuff  V 



```

#Using in command line

`cargo install svgbob`

`svgbob` normally operates on stdin and stdout:
```console
$ svgbob < examples/long.bob > long.svg
```
produces an SVG in `long.svg` similar to the one produced by the old `main` binary. `svgbob` also allows passing arguments instead:
```console
$ svgbob examples/long.bob -o long.svg
```
And you can mix and match:
```console
$ svgbob -o long.svg < examples/long.bob
$ svgbob examples/long.bob > long.svg
```

This is also documented in the output of `svgbob --help`:
```console
$ svgbob --help
svgbob 0.1.2
SvgBobRus is an ascii to svg converter

USAGE:
    svgbob [OPTIONS] [input]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output <output>    where to write svg output [default: STDOUT]

ARGS:
    <input>    svgbob text file to parse [default: STDIN]
```



Originally written in elm from the original [project](https://github.com/ivanceras/svgbob)

[Demo site](https://ivanceras.github.io/svgbobrus/)
