# Changelog

## 0.6.1
Improve code readability on view by using arrays instead of vec
Modify the algorithm for endorse such that circles and arcs are matched by their ascii art rather than their computed fragments, this way, it can intersect with other fragments

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
