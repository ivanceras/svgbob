## TODO
- [ ] detect rectangles
- [x] detect whether used as text or used as graphs (if any of the 8 has no get character then it is used as text) 
- [ ] complete the specs
- [ ] Make the string literal escape work multiline
- [x] Make a swap out interface. 
      - useful for detecting broken lines such as - - - and replace it with ~~~~~. This way
        it will be easier to process by the fragment emitter
- [ ] Group traced elements together, then reduce as needed
- [ ] Add `#` as square start marker for lines
