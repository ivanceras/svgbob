# Changelog

## Unreleased
- feat: **Breaking** make all the styles be contained under a .svgbob class, in order to prevent the other svg elements from clashing with other svg elements in the document outside of svgbob svg
   - Fixes [Issue #100](https://github.com/ivanceras/svgbob/issues/100)
- feat: remove multiple Polygon tag for arrow characters
- refactor: remove is_shared_x and is_shared_y since it is not used
- refactor: change the names of the circle according to their index instead of their diameter
- feat: implement THREE_QUARTERS_ARC circles, you can now draw circles 3/4 through
   ![svgbob-circles](https://user-images.githubusercontent.com/7972655/232184480-b20bc2c3-10f7-4467-8598-ef497122e3c9.png)
- feat: remove merge_line_with_polygon as it is buggy as well
- feat: improve endorse function to use re_endorse
- feat: use the old algorithmn for adding multiple fragments, remove buggy code for merging line and marker_line
- refactor: give a name to the cells in the half arc first
- disable merge_marker_line since it is buggy
- refactor: use index_map for the Circle span to preserve the order of the circles being inserted
- feat: fix and improve spans for HALF_ARC_SPAN
- fix: localize points to bottom_half and right_half of HALF_ARCS
- improvement on the HALF ARC span, extracted from the circle_art instead of assembling it from the QUARTER_ARCS
- refactor: use correct names of variables
- fix: remove panicking in the fragment_span merge
- refactor: convert group_recursive into using Merge trait
- convert into_<..> methods to impl From
- refactor: improve second_pass_merge
- breaking: remove settings from the low-level function calls
- refactor: improve the code by unifying the algorithmn of merging objects using the Merge trait
- feat: add a trait Merge to unify common algorithmns for merging fragments
- feat: add re_endorse function for retrying re-endorsing the rest of the fragments into possible shapes
- feat: add dynamic behavior for asterisk with horizontal dashes
- feat: add a new function which returns regular fragments and touching fragments
- feat: use FragmentSpan for most fragments
- refactor: move the functions to their modules
- feat: make fragment buffer contains the original char
- feat: add a function to test bounding cells for contacts
- feat: add FragmentSpan
- feat: add a method to check if a cell hits a span
- feat: rename intersects_bounds to is_bounded and checking must be all cells are inside the bounds specified
- feature: add intersect_bounds method for Span

## 0.6.7
- bump dep sauron to 0.50.5

## 0.6.6
- bump dep sauron to 0.45.0

## 0.6.5
- added svgbob_server package for serving svgbob in a rest api call
- bump dep sauron to 0.44.0

## 0.6.4

- simplify circle art
- improve the implementation of quarter arc making them consistent and work correctly.

## 0.6.3
- move Settings to the top level of this crate

## 0.6.2
- Reorganize directory structure, update to sauron 0.43.4
- reexport nalgebra, add logging on points ord

## 0.6.1
- Improve code readability on view by using arrays instead of vec
- Modify the algorithm for endorse such that circles and arcs are matched by their ascii art rather than their computed fragments, this way, it can intersect with other fragments

## 0.6.0
- Update to sauron 0.41.0
- Fix jss
## 0.5.5
- issue#38
- Add test for issue#38 and remove panicking code due to char boundary error

## 0.5.4
- Update sauron version to `0.40`
- Use `sauron::jss` macro to simplify creating a css from settings
- Add a compressed variant to the generated svg
- Add a little arrowed arc in unicode map
- Use the `parry2d` as a new new of the old library `ncollide2d`

# 0.5.3
- Update to the lastest version of `nalgebra` and `ncollide2d`.
- Add a public function for Rect fragment to determine whether it is using a rounded rectangle or not

# 0.5.2
- overhaul circle map algorithm calculating centers and radius based on the circle art(number of cells occupied horizontally), specified edge_case, and offset from top cell to the circle_radius

# 0.5.1
- Improve implementation of is_intersecting
- Include circle in intersection test
- Add a function to return regular shapes such as circle and rect, this will be used for testing hit, which fragment is hit
- Initial implementation for testing hit/intersection AABB to any fragment
- refactor group_node_and_fragments to clearly group the fragments first, as this will be used to test for fragment hit test

# 0.5.0
- All the changes in 0.5.0-alpha.* is now on this version.

## 0.5.0-alpha.12
- Adds a parameter in the Settings to specify whether or not Arrow heads will merge with lines. The default value is false, hence the arrow heads will be rendered as polygons instead of a marker to a line
    - Improve polygons for arrow, open circles, filled_circles, squares and diamonds
    - Pass the settings to merge functions, so the user can opt to chose whether or not to allow mergin_line_with_shapes
- Add DIAMETER_CIRCLE lookup and addition circle art.
- Add a function to convert ascii to svg with an override size
- Add default implementation for StringBuffer

## 0.5.0-alpha.11
- Reinstated double quote escaping of text that are not meant to be interpreted as drawing element
- Fix merging of polygons and lines, arrows and lines, circles and lines.
- Disabled merging of marker_line to any other shape for now.

## 0.5.0-alpha.4
- update to sauron `0.24.0`

## 0.5.0-alpha.3
- remove local paths
- update to sauron `0.23.0`

## 0.5.0-alpha.2
-  re-export `sauron::Node`

## 0.5.0-alpha.1
- Expose the with-dom feature in svgbob, this is to allow usage of the generated node readily usable as is in sauron projects, provided they uses the same version of sauron library
- Make functions that return Node<()> into a return generics Node<MSG> to allow usage of Node<MSG> generics in sauron apps
- Fix the behaviour of merging line and polygon tags, the direction of the arrow is now correct and only dealing with line that is in the same direction of the arrow marker
- Merging line with arrow and heading with opposite direction is not yet dealth

## 0.5.0-alpha.0
- rewrite core architecture to support shape tagging and styling
