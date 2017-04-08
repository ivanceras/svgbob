#[cfg(test)]
mod bench_test{
    #[feature(test)]
    extern crate test;
    use test::Bencher;
    #[bench]
    fn bench_svgbob(b: &mut Bencher) {
        b.iter(|| {
            to_svg(get_arg());
        });

            fn get_arg() -> &'static str{

            let arg = r#"
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
            "#;

            arg
            }
    }

}
