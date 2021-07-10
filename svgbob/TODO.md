## TODO
- [x] detect rectangles
- [x] detect whether used as text or used as graphs (if any of the 8 has no get character then it is used as text)
- [ ] complete the specs
- [ ] ~~Make the string literal escape work multiline~~ Impractical
- [x] Make a swap out interface.
      - useful for detecting broken lines such as - - - and replace it with ~~~~~. This way
        it will be easier to process by the fragment emitter
     - [ ] Fix broken line implementation
- [ ] Group traced elements together, then reduce as needed
- [x] Add `#` as square start marker for lines
- [x] Support for geometric shapes https://en.wikipedia.org/wiki/Geometric_Shapes
- [x] Add enhance circle, detect circles then enhance it.
- [ ] Add test cases
- [ ] Re-implement the escape string with double quotes.
- [ ] Use perfect hashmap [phf](https://crates.io/crates/phf) to efficiently build the maps(unicode_map, circle_map, ascii_map) at compile time.
- [ ] Fix the double arrow issue
        When there is 2 arrows in the middle of a line `---->>-------` or `----<<----`
- [ ] Clean the project enforce deny warnings.
- [ ] Fix a bug where an escaped text has whitespaces, the whitespaces are gone.
- [ ] Revise calculation of Circle and Arc center by basing on the number of chars/width
