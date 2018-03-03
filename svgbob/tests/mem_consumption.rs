#![feature(libc)]
extern crate libc;
extern crate svgbob;
extern crate sys_info;
extern "C" {
    fn je_stats_print(
        write_cb: extern "C" fn(*const libc::c_void, *const libc::c_char),
        cbopaque: *const libc::c_void,
        opts: *const libc::c_char,
    );
}
extern "C" fn write_cb(_: *const libc::c_void, message: *const libc::c_char) {
    print!(
        "{}",
        String::from_utf8_lossy(unsafe {
            std::ffi::CStr::from_ptr(message as *const i8).to_bytes()
        })
    );
}

#[test]
fn show_mem_consumption() {
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

    println!("before: {:?}", sys_info::mem_info().unwrap());
    unsafe { je_stats_print(write_cb, std::ptr::null(), std::ptr::null()) };
    self::svgbob::to_svg(arg);
    println!("after: {:?}", sys_info::mem_info().unwrap());
    unsafe { je_stats_print(write_cb, std::ptr::null(), std::ptr::null()) };
}
