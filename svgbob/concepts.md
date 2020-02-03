
Turns svg into text respresentation by approximating the lines,
circles, arcs into character blocks

A bounding box of a line is computed and the rectangles (AKA the cells) in the grid
are then tested which points for each rectangle intersects the line.
This points are then approximated into the closes SubCell location.

Each subcell will now contain drawing fragments such as line, arc, circles
and then tried to be match to which characters/letter/ascii would best
represent the drawing fragment.
