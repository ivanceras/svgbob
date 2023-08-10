# Svgbob Architecture and Design phases

Svgbob creates an svg drawing based on the input ascii art diagrams.
It achieves this by creating a corresponding fragment for each character, and then this little fragments
are then merged to form lines and arcs. The lines and arcs are then endorsed into high level shapes such as rect, circles.

### Name inspiration:
- svg for svg document and drawing.
- bob for Alice and Bob as common characters in most diagrams
   Bob Ross - a painter who like to draws happy little trees.

### Library used
- [nalgebra](https://www.nalgebra.org/) and [parry2d](https://parry.rs/) for geometric function calculations such as calculating whether lines are intersecting, collinear. Computing the clipping of lines and boxes.
- [pom](https://github.com/J-F-Liu/pom) for parsing the styling directives(Legend) at the bottom of the document
- [sauron](https://github.com/ivanceras/sauron) for building the svg document object tree.


### **Iterations, re-architecture rewrites**

####  Phase 1
Exploding if statements. This was in elm
   [fullcode](https://github.com/ivanceras/elm-examples/blob/master/elm-bot-lines/Grid.elm)

```elm
getElement x y model =
    let
        char = get x y model
    in
        case char of
            Just char ->
                if isVertical char
                    && not (isNeighbor left isAlphaNumeric)
                    && not (isNeighbor right isAlphaNumeric) then
                    Just Vertical
                else if isHorizontal char
                    && not (isNeighbor left isAlphaNumeric)
                    && not (isNeighbor right isAlphaNumeric) then
                    Just Horizontal
                else if isIntersection char then
                    let
                        isVerticalJunctionLeft =
                            isNeighbor top isVertical
                            && isNeighbor(bottomOf  x y model) isVertical
                            && isNeighbor(leftOf  x y model) isHorizontal

                        isVerticalJunctionRight =
                            isNeighbor top isVertical
                            && isNeighbor bottom isVertical
                            && isNeighbor right isHorizontal

                        isHorizontalJunctionTop =
                            isNeighbor left isHorizontal
                            && isNeighbor right isHorizontal
                            && isNeighbor top isVertical

                        isHorizontalJunctionBot =
                            isNeighbor left isHorizontal
                            && isNeighbor right isHorizontal
                             && isNeighbor bottom isVertical

                        isTopLeftIntersection =
                            isNeighbor bottom isVertical && isNeighbor right isHorizontal

                        isTopRightIntersection =
                            isNeighbor bottom isVertical && isNeighbor left isHorizontal

                        isBottomRightIntersection =
                            isNeighbor top isVertical && isNeighbor left isHorizontal

                        isBottomLeftIntersection =
                            isNeighbor top isVertical && isNeighbor right isHorizontal

                        isCrossIntersection =
                            isNeighbor top isVertical
                            && isNeighbor bottom isVertical
                            && isNeighbor left isHorizontal
                            && isNeighbor right isHorizontal

    ...  200 more lines...

```
Though elm is fast, but if you throw a lot of conditional branching to it, it will slow it down.
At least I don't get to have runtime errors here if it was written in js.
Adding an edgecase is just appending a new if else statement at the bottom of the statements.

**Pros:** Very simple design. Just if statements and return the appropriate shape the character will take form
    Adding edge case behaviour is just appending an `else if` to the nearest conditional(`if`) behavior.

**Caveats:** The fragments/drawing elements are named. Naming is hard, we can not name all of them. Consistency is broken.




#### Phase2:
Now in rust. The character behavior is stored in a `Vec<(condition, drawing_elements)>`
This is already close to the current architecture.

  **Improvements:**
   - Runs a lot faster than elm. Converting the code from elm to rust, accelerate my learning of the usage of functional programming in rust.
   - Consumed elements, if certain group of elements matches a higher level shapes, those elements are consumed/remove from the grid to
   avoid generating additional drawing elements when iterated with the rest of the characters in the grid.


```rust
    //get the paths in the location x,y
    //if non path, then see if it can return a text path
    fn get_elements(&self, x:isize, y:isize, settings: &Settings) -> Option<Vec<Element>>{
        ...
        //common path lines
        let vertical = Element::solid_line(center_top, center_bottom);
        let horizontal = Element::solid_line(mid_left, mid_right);
        let slant_left = Element::solid_line(high_left, low_right);
        let slant_right = Element::solid_line(low_left, high_right);
        let low_horizontal = Element::solid_line(low_left, low_right);


        let match_list: Vec<(bool, Vec<Element>)> =
            vec![
                /*
                      .-
                      |
                */
                (self.is_char(this, is_round)
                 && self.is_char(right, is_horizontal)
                 && self.is_char(bottom, is_vertical),
                 vec![cxdy_cxey.clone(), arc_excy_cxdy.clone()]
                ),
                /*
                      -.
                       |
                */
                (self.is_char(this, is_round)
                 && self.is_char(left, is_horizontal)
                 && self.is_char(bottom, is_vertical),
                 vec![cxdy_cxey.clone(), arc_cxdy_axcy.clone()]
                ),
                /*
                     |
                     '-
                */
                (self.is_char(this, is_round)
                 && self.is_char(right, is_horizontal)
                 && self.is_char(top, is_vertical),
                 vec![cxay_cxby.clone(), arc_cxby_excy.clone()]
                ),
                /*
                     |
                    -'
                */
                (self.is_char(this, is_round)
                 && self.is_char(left, is_horizontal)
                 && self.is_char(top, is_vertical),
                 vec![cxay_cxby.clone(), arc_axcy_cxby.clone()]
                ),
                /*
                    .-
                   /
                */
                (self.is_char(this, is_round)
                 && self.is_char(right, is_horizontal)
                 && self.is_char(bottom_left, is_slant_right),
                 vec![axey_bxdy.clone(), arc_excy_bxdy.clone()]
                ),
                /*
                   -.
                     \
                */
                (self.is_char(this, is_round)
                 && self.is_char(left, is_horizontal)
                 && self.is_char(bottom_right, is_slant_left),
                 vec![exey_dxdy.clone(), arc_dxdy_axcy.clone()]
                ),
                /*
                   -.
                   /
                */
                (self.is_char(this, is_round)
                 && self.is_char(left, is_horizontal)
                 && self.is_char(bottom_left, is_slant_right),
                 vec![axey_bxdy.clone(), arc_bxdy_axcy.clone()]
                ),
                /*
                   .-
                    \
                */
                (self.is_char(this, is_round)
                 && self.is_char(right, is_horizontal)
                 && self.is_char(bottom_right, is_slant_left),
                 vec![exey_dxdy.clone(), arc_excy_dxdy.clone()]
                ),
                /*
                   \
                    '-
                */
                (self.is_char(this, is_round)
                 && self.is_char(right, is_horizontal)
                 && self.is_char(top_left, is_slant_left),
                 vec![axay_bxby.clone(), arc_bxby_excy.clone()]
                ),
                /*
                     /
                    '-
                */
                (self.is_char(this, is_round)
                 && self.is_char(right, is_horizontal)
                 && self.is_char(top_right, is_slant_right),
                 vec![dxby_exay.clone(), arc_dxby_excy.clone()]
                ),
                /*
                    \
                    -'
                */
                (self.is_char(this, is_round)
                 && self.is_char(left, is_horizontal)
                 && self.is_char(top_left, is_slant_left),
                 vec![axay_bxby.clone(), arc_axcy_bxby.clone()]
                ),
                /*
                      /
                    -'
                */
                (self.is_char(this, is_round)
                 && self.is_char(left, is_horizontal)
                 && self.is_char(top_right, is_slant_right),
                 vec![dxby_exay.clone(), arc_axcy_dxby.clone()]
                ),
            ]
```

```rust

        // Circle 12
        //        _
        //      .' '.
        //     (  +  )
        //      `._.'
        if self.in_left(3).is('(')
            && self.in_right(3).is(')')
            && self.in_top(2).is('_')
            && self.bottom().is('_')
            && self.top().in_left(2).any(",.")
            && self.top_left().is('\'')
            && self.top_right().any("`'")
            && self.top().in_right(2).is('.')
            && self.bottom().in_left(2).any("`'")
            && self.bottom_left().is('.')
            && self.bottom_right().any(".,")
            && self.bottom().in_right(2).is('\'')
        {
            elm.push(open_circle(m, 12));
            consumed.extend(vec![
                left3(),
                right3(),
                top2(),
                bottom(),
                top_left2(),
                top_left(),
                top_right(),
                top_right2(),
                bottom_left2(),
                bottom_left(),
                bottom_right(),
                bottom_right2(),
            ]);
        }
```
**Caveats:**
    - Merging of small fragments requires checking against all the other fragments of the entire grid. Runtime complexity is at least O(n^2)
    - Endorsing to shapes requires a lot of if statement comparisons and every cell is checked even for cell that has only a few elements that couldn't form into a certain shapes is tested.
    - Processing high level stage and low level fragment stage is one execution.
    - Drawing elements are still named.




#### Phase 3:
Attempts to add a signal strength to characters depending on their
neighboring character whether they should connect or not. This makes the dynamic behavior flexible
but the control flow is not very intuitive.

 - Strong + Strong should connect
 - Medium + Medium connects
 - Medium + Weak may connect
 - Weak + Weak should not connect.

```rust

    /// get the characteristic of a character
    /// it's behavior and the intended behavior
    ///
    ///    ┌─┬─┬─┬─┬─┐
    ///    │a│b│c│d│e│
    ///    ├─┼─┼─┼─┼─┤
    ///    │f│g│h│i│j│
    ///    ├─┼─┼─┼─┼─┤
    ///    │k│l│m│n│o│
    ///    ├─┼─┼─┼─┼─┤
    ///    │p│q│r│s│t│
    ///    ├─┼─┼─┼─┼─┤
    ///    │u│v│w│x│y│
    ///    └─┴─┴─┴─┴─┘
    ///
    fn get_characteristic(&self) -> Option<Characteristic> {
        ///////////////////////////
        //
        // ., dot or period and comma
        //
        ///////////////////////////
        if self.any(".,") {
            Some(Characteristic {
                is_static: false,
                intensify: vec![
                    //  -.  +.
                    (
                        K,
                        Condition {
                            loc: left(),
                            can: ConnectTo(O, Medium),
                        },
                    ),
                    //  .-  .+
                    (
                        O,
                        Condition {
                            loc: right(),
                            can: ConnectTo(K, Medium),
                        },
                    ),
                    //  _.
                    (
                        U,
                        Condition {
                            loc: left(),
                            can: ConnectTo(Y, Strong),
                        },
                    ),
                    //  ._
                    (
                        Y,
                        Condition {
                            loc: right(),
                            can: ConnectTo(U, Strong),
                        },
                    ),
                    //      .
                    //     /
                    (
                        U,
                        Condition {
                            loc: bottom_left(),
                            can: ConnectTo(E, Strong),
                        },
                    ),
                    //      /    only for / else   _
                    //     .                        .   will connect
                    (
                        E,
                        Condition {
                            loc: top_right(),
                            can: IsStrongAll(vec![E, U]),
                        },
                    ),
                    //      .
                    //       \
                    (
                        Y,
                        Condition {
                            loc: bottom_right(),
                            can: ConnectTo(A, Strong),
                        },
                    ),
                    ...
                ],
                intended_behavior: vec![
                    //     .-
                    //    /
                    (vec![O, U], vec![arc(o, q, 4), line(q, u)]),
                    //     .-
                    //      \
                    (vec![O, Y], vec![arc(o, s, 4), line(s, y)]),
                    //     -.
                    //       \
                    (vec![K, Y], vec![arc(s, k, 4), line(s, y)]),
                    //     -.
                    //     /
                    (vec![K, U], vec![line(u, q), arc(q, k, 2)]),
                    //       /
                    //      .
                    //     /
                    (vec![U, E], vec![line(u, e)]),
                    //     \
                    //      .
                    //       \
                    ...
                ],
                properties: vec![
                    (O, Weak, vec![arc(o, r, 2)]),
                    (K, Weak, vec![arc(r, k, 2)]),
                    (W, Medium, vec![line(r, w)]),
                    (U, Weak, vec![line(q, u)]),
                    (Y, Weak, vec![line(s, y)]),
                    (A, Weak, vec![line(m, a)]),
                    (E, Weak, vec![line(m, e)]),
                    (F, Weak, vec![line(m, f)]),
                    (J, Weak, vec![line(m, j)]),
                ],
            })
        }
```

**Pros:**
 - Characters are assigned with certain properties. This allows similar characters such as dash(-) and line drawing (-) to have the same behavior
     without explicitly coding for each of those variations.


#### Phase 4.

**Improvements:**
- Uses of Buffers
   - StringBuffer, input strings are slices into rows and columns
   - CellBuffer, which cells contains which character.
   - FragmentBuffer, which cell contains what fragments(drawing elements)
   - PropertyBuffer, what is the property of each cell based on the the character it contains.

PropertyBuffer is calculated only once for each character, so the succeeding lookup should not waste execution time to recompute.


**How the fragments are conceived based on a character?**

**Neighbor character:** There are 8 neighbors of a character and each character on the input is checked against this 8 neighbor for appropriate drawing element

```bob
  +---------+  +------+  +--------+
  |  TopLeft|  | Top  |  |TopRight|
  +---------+  +------+  +--------+
  +---------+  +------+  +--------+
  |  Left   |  |(char)|  | Right  |
  +---------+  +------+  +--------+
 +----------+  +------+  +-----------+
 |BottomLeft|  |Bottom|  |BottomRight|
 +----------+  +------+  +-----------+
```

**Character Grid:** a 5x5 grid which covers the most significant points for a character to be converted into drawing elements.

Character grid: / is the line connecting E to U. Dash is connecting K to O, etc.
```bob

  0 1 2 3 4           B C D
 0┌─┬─┬─┬─┐        A┌─┬─┬─┬─┐E
 1├─┼─┼─┼─┤         │ │ │ │ │
 2├─┼─┼─┼─┤        F├─G─H─I─┤J
 3├─┼─┼─┼─┤         │ │ │ │ │
 4├─┼─┼─┼─┤        K├─L─M─N─┤O
 5├─┼─┼─┼─┤         │ │ │ │ │
 6├─┼─┼─┼─┤        P├─Q─R─S─┤T
 7├─┼─┼─┼─┤         │ │ │ │ │
 8└─┴─┴─┴─┘        U└─┴─┴─┴─┘Y
                      V W X

```
These fragments are processed such as merging collinear lines that are touching their endpoints.

```bob

+--------------+        +------------+         +----------------+          +-----------------+
| StringBuffer |------> | CellBuffer |-------->| FragmentBuffer |--------->|   Svg drawing   |
+--------------+        +------------+         +----------------+          +-----------------+
                              \                                                           ^
                               \    +-------+                                            /
                                `-->| Spans |                                           /
                                    +-------+                                          /
                                      \                                               /
                                       \    +---------------+    .----------------.  /
                                        `-->|Contact groups |---/ endorse shapes /--'
                                            +---------------+  '----------------'
```

- **Optimizations.**
    - Usage of span and contact groups.
       Span group together that are neighbors. Contact groups group together fragments
       that are touching together. Cells don't need to be checked against other cells
       when they are far from each other. Merging of fragments such as lines into longer
       lines needs to interact only elements that are within its group.
- Endorsing group of fragments into higher level shapes.
    - rect, rounded rect, circles, arcs are higher level shapes that are from small fragment components: arc,lines,

- **Tagging shapes.**
    Text inside of a shape with the pattern "{", <ident> "}" will become a tag of the enclosing shape.
    At the DOM level, the shape is an svg dom element such as: rect,circle,path and the tag is the element `class`
    which you can use css to apply a style to the element. The legend part at the bottom of the document is parsed
    and converted into css which is then appended to the svg document.

```rust

    ///
    ///      0 1 2 3 4           B C D
    ///     0┌─┬─┬─┬─┐        A┌─┬─┬─┬─┐E
    ///     1├─┼─┼─┼─┤         │ │ │ │ │
    ///     2├─┼─┼─┼─┤        F├─G─H─I─┤J
    ///     3├─┼─┼─┼─┤         │ │ │ │ │
    ///     4├─┼─┼─┼─┤        K├─L─M─N─┤O
    ///     5├─┼─┼─┼─┤         │ │ │ │ │
    ///     6├─┼─┼─┼─┤        P├─Q─R─S─┤T
    ///     7├─┼─┼─┼─┤         │ │ │ │ │
    ///     8└─┴─┴─┴─┘        U└─┴─┴─┴─┘Y
    ///                          V W X
    pub static ref ASCII_PROPERTIES: BTreeMap<char, Property> = {

            ...

            vec![

            //////////////////////
            // dot period .
            //////////////////////
            (
                '.',
                vec![
                    (Medium, vec![line(m,w)]), // connects down
                    (Weak, vec![line(m,k)]), // connects left
                    (Weak, vec![line(m,o)]), // connects right
                ],
                Arc::new(
                        move|top_left, top, top_right, left, right, bottom_left, bottom, bottom_right| {
                        vec![
                            // .
                            // |
                            (bottom.line_strongly_overlap(c,h), vec![line(r,w)]),
                            //   .
                            //  / \
                            (bottom_left.line_strongly_overlap(e,i) && bottom_right.line_strongly_overlap(a,g), vec![line(m,u), line(m,y)]),
                            //  .-
                            //  |
                            (right.line_overlap(k,l) && bottom.line_overlap(c,h), vec![arc(o,r,unit2), line(r,w)]),
                            //   .-
                            //  |
                            (right.line_overlap(k,l) && bottom_left.line_overlap(c,h), vec![arc(m,cell.bottom_left().c(),unit4), line(m,o)]),
                            // -.
                            //  |
                            (left.line_overlap(n,o) && bottom.line_overlap(c,h), vec![arc(r,k,unit2), line(r,w)]),
                            // -.
                            //   |
                            //  exemption that bottom right is not a backquote
                            (!bottom_right.is('`') && left.line_overlap(n,o) && bottom_right.line_overlap(c,h), vec![arc(cell.bottom_right().c(),m,unit4), line(k,m)]),
                            //     .-
                            //    /
                            (right.line_overlap(k,l) && bottom_left.line_overlap(e,i), vec![arc(o, q, unit4), line(q, u)]),
                            //     .-
                            //      \
                            (right.line_overlap(k,l) && bottom_right.line_overlap(a,g) , vec![arc(o, s, between1_2), line(s, y)]),
                            //     -.
                            //       \
                            (left.line_overlap(n,o) && bottom_right.line_overlap(a,g), vec![arc(s, k, unit4), line(s, y)]),
                            //     -.
                            //     /
                            (left.line_overlap(n,o) && bottom_left.line_overlap(e,i), vec![arc(q, k, between1_2), line(u, q)]),

                            ...
                        ]}
                    )
            ),
```

#### **Endorse to higher level shapes**

```rust

    /// First phase of endorsing to shapes, in this case, rects and rounded_rects
    ///
    /// This function is calling on endorse methods that is applicable
    /// to fragments that are touching, to be promoted to a shape.
    /// These includes: rect, roundedrect,
    fn endorse_rects(groups: Vec<Contacts>) -> (Vec<Fragment>, Vec<Contacts>) {
        let mut fragments = vec![];
        let mut un_endorsed_rect: Vec<Contacts> = vec![];
        for group in groups {
            if let Some(fragment) = is_rect(group) {
                fragments.push(fragment);
            } else {
                un_endorsed_rect.push(group);
            }
        }
        (fragments, un_endorsed_rect)
    }

    ...

    /// group of fragments can be check if they form:
    /// - rectangle
    fn is_rect(fragments: &Vec<Fragment>) -> bool {
        if fragments.len() == 4 {
            let parallels = parallel_aabb_group(fragments);
            if parallels.len() == 2 {
                let (a1, a2) = parallels[0];
                let (b1, b2) = parallels[1];
                let line_a1 = fragments[a1].as_line();
                let line_b1 = fragments[b1].as_line();
                let line_a2 = fragments[a2].as_line();
                let line_b2 = fragments[b2].as_line();
                line_a1.is_touching_aabb_perpendicular(line_b1)
                    && line_a2.is_touching_aabb_perpendicular(line_b2)
            } else {
                false
            }
        } else {
            false
        }
    }

    ...

    /// [X](Done) TODO: search only the subset of contacts that matches the circle.
    /// if it is a subset then the circle is matched and the non-matching ones are returned
    pub fn endorse_circle(search: &Vec<Contacts>) -> Option<(&Circle, Vec<usize>)> {
        FRAGMENTS_CIRCLE.iter().rev().find_map(|(contacts, circle)| {
            let (matched, unmatched) = is_subset_of(contacts, search);
            if matched { Some((circle, unmatched)) } else { None }
        })
    }

    ...

    /// This function is calling on endorse algorithm on fragments that
    /// are neighbors, but not necessarily touching to be promoted to a shape.
    /// These includes: circle, arc, and line with arrow heads.
    fn endorse_circles_and_arcs(groups: Vec<Contacts>) -> (Vec<Fragment>, Vec<Contacts>) {
        let mut fragments = vec![];
        let mut un_endorsed_circles: Vec<Contacts> = vec![];
        if let Some((circle, unmatched)) = circle_map::endorse_circle(&groups) {
            fragments.push(circle.clone().into());
            for um in unmatched {
                un_endorsed_circles.push(groups[um].clone());
            }
        } else if let Some(arc) = circle_map::endorse_arc(&groups) {
            fragments.push(arc.clone().into());
        } else {
            un_endorsed_circles.extend(groups)
        }
        (fragments, un_endorsed_circles)
    }
```

```rust

    //   ascii art,  Center Cell, Center Point, radius
    pub static ref CIRCLE_MAP: Vec<(&'static str, Cell, Point, f32)> =
        vec![
            // CIRCLE_1
            //center 0,0,o, radius = 0.5
            (r#"
            ()
            "#, Cell::new(0,0), Cell::new(0,0).o(), 0.5),

            ...

            // CIRCLE_4
            //center: 2,1,m radius: 2.0
            (r#"
             ,-.
            (   )
             `-'
            "#, Cell::new(2,1), Cell::new(2,1).m(), 2.0),


            // CIRCLE_12
            //center:6,3,m radius: 6.0
            (r#"
                _____
              ,'     `.
             /         \
            (           )
             \         /
              `._____.'
            "#, Cell::new(6,3), Cell::new(6,3).m(), 6.0),

            // CIRCLE_17
            //center: 8,4,o radius: 8.5
            (r#"
                .--------.
              ,'          `.
             /              \
            |                |
            |                |
            |                |
             \              /
              `.          .'
                `--------'
            "#, Cell::new(8,4), Cell::new(8,4).o(), 8.5),


            ...

            // CIRCLE_20
            // center: 10,5,m radius: 10
            (r#"
                _.-'''''''-._
              ,'             `.
             /                 \
            .                   .
            |                   |
            |                   |
            |                   |
             \                 /
              `._           _.'
                 '-.......-'
            "#, Cell::new(10,5), Cell::new(10,5).m(), 10.0),
        ];

```

#### Flexibility:
- Adding behaviours and edge-cases is still simple
- Due to the grouping of spans and contacts, it is now more efficient to check whether a combination
    of fragments can be endorsed into a high level shapes.
- Behavior can be coded according to the properties of their neighboring characters,
    and/or can also specify that a neighbor should match a specific character. (ie: neighboring character top should be a caret `^`, then this is the behavior)

#### Modular:
-  Adding more shapes it can endorse to, such as in the circle map is merely putting the ascii art
to right next to the existing ones, as oppused to the multiple if-statements in Phase 2
- Adding endorse code to certain shapes is merely describing the filter rules on the combination of the fragments

#### Extensiblity:
- Since the new architecture is now implemented through the use of Buffers. It opens to a lot of possible improvements.
- Shapes are now properly endorsed, which can be styled with css standard. Which means, users can add crazy css-animation to the shapes.
- Making the cell buffer as a canvas. Meaning you can draw lines and shapes on it, while the system will try to match
    the closest character appropriate to the input shape. A possibility of generating an ascii drawing from svg diagrams.
    The reverse of the functionality of svgbob.

#### Adaption of svgbob
- As archlinux [package](https://aur.archlinux.org/packages/svgbob-git/)
- As diagram module for [asciidoctor](https://asciidoctor.org/docs/asciidoctor-diagram/)
- [Asciigrid](https://gitlab.com/mbarkhau/asciigrid/)
- [kroki.io](https://kroki.io/)

