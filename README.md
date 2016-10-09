# svgbobrus
SvgBobRus is an ascii to svg converter.

![image](https://ivanceras.github.io/svgbobrus/examples/demo.svg)

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
Originally written in elm from the original [project](https://github.com/ivanceras/svgbob)

[Demo site](https://ivanceras.github.io/svgbobrus/)
