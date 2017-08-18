## Svgbob specification 

[Rendered](https://ivanceras.github.io/spongedown-editor/?file=https://raw.githubusercontent.com/ivanceras/svgbobrus/master/spec.md)

Svgbob is a diagramming model which uses common typing characters to approximate the shape

|characters| names                | description              
|----------|----------------------|------------
|   `-`    | dash, hypen, minus   | for horizontal lines
|   `_`    | underscore           | for horizontal lines
|   `\|`     | pipe, or             | for vertical lines
|   `/`    | forward slash        | for lines slanted to the right
|   `\\`   | backslash            | for lines slanted to the left
|   `+`    | plus, add, cross     | for sharp intersection
|   `.`    | dot, period          | for rounded corner intersection
|   `,`    | comma                | for rounded corner intersection
|   `'`    | single quote         | for rounded corner intersection
|  `` ` `` | backtick             | for rounded corner intersection
|   `(`    | open parenthesis     | rounded side of an arc
|   `)`    | close parenthesis    | rounded side of an arc
|   `[`    | open braces          | rectangular corner
|   `]`    | close braces         | rectangular corner
|   `*`    | asterisk             | for emphasized intersection
|   `o`    | lowercase letter `o` | for intersection
|   `O`    | uppercase letter `O` | for intersection
|   `^`    | caret                | for arrow up
|   `v`    | lowercase letter `v` | for arrow down
|   `V`    | lowercase letter `V` | for arrow down
|   `<`    | lesser than          | for arrow left
|   `>`    | greater than         | for arrow right
|   `=`    | equal sign           | for double horizontal line
|   `x`    | lowercase letter `x` | for intersection
|   `X`    | uppercase letter `X` | for intersection

## Principle
The rendered shape should closely resembles to that of the ascii
drawing / formation.

## Non-goals
The goal is NOT to be able to make graphs and diagrams with less effort.

## Default sizes
Text height is 2 times the text width, both should be a multiple of 4.

Default values:
- text width = 8px;
- text height = 16px;

-------------------
## Horizontal line `-`
- if next to an alphanumeric character it will be rendered as text.
- if next to another drawing character, it will be rendered as a line.
- if alone it will be rendered as a line.

 **Example 1:**

|text  | svg                                   |rendered      
|------|---------------------------------------|--------------
|`-`   |`<line x1="0" y1="8" x2="8" y2="8"/>`  |<svg width="16" height="16"><line x1="0" y1="8" x2="8" y2="8"/></svg>
|`--`  |`<line x1="0" y1="8" x2="16" y2="8"/>` |<svg width="16" height="16"><line x1="0" y1="8" x2="16" y2="8"/></svg>
|`----`|`<line x1="0" y1="8" x2="32" y2="8"/>` |<svg width="32" height="16"><line x1="0" y1="8" x2="32" y2="8"/></svg>

**Example 3:** Next to an alphanumeric character

|text  |svg                           |rendered
|------|------------------------------|-------
|`1-`  |`<text x="2" y="12">1-</text>`|<svg width="32" height="16"><text x="2" y="12">1-</text></svg>
|`-a`  |`<text x="2" y="12">-a</text>`|<svg width="32" height="16"><text x="2" y="12">-a</text></svg>
    
**Example 4:** Used together with text

|text       |svg                                  |rendered
|-----------|-------------------------------------|----------
|`opt-in`   |`<text x="2" y="12">opt-in</text>`   |<svg width="48" height="16"><text x="2" y="12">opt-in</text></svg>
|`chat-room`|`<text x="2" y="12">chat-room</text>`|<svg width="144" height="16"><text x="2" y="12">chat-room</text></svg>

---------------------
## Vertical line  `|`
- if next to an alphanumeric character it will be rendered as text.
- if next to another drawing character it will be rendered as vertical line.
- if alone it will be rendered as vertical line

**Example 1:**

```
|
|
```
|text              | svg                         | rendered     
|------------------|-----------------------------|----------------
|`\|`<br>`\|`  |`<line x1="4" y1="0" x2="4" y2="32"></line>` |<svg width="8" height="32"><line x1="4" y1="0" x2="4" y2="32"></line></svg> |

    use as or expression
    Example 2: 
        a||b   <text>a||b</text>

------------------
## Forward slash  `/`
- if next to an alphanumeric character it will be rendered as text.
- if at least one if its 8 neighbors: (top,bottom,left,right, topleft, topright, bottomleft, bottomright)
  is a drawing character then it will be rendered as a slanted line to the right
- if used as text but next to a drawing element at the same time, rendering to drawing
  element takes precedence.
    
**Example 1:**
```
 /
/
```

|text    | svg    | rendered
|--------|--------|---------------
|&nbsp;`/`<br>`/`|`<line x1="0" y1="16" x2="16" y2="32"></line>`|<svg width="16" height="32"><line x1="0" y1="32" x2="16" y2="0"></line></svg>
   

**Example 2:**

|text | svg | rendered
|-----|-----|-----------
|`folder/`    |`<text x="2" y="12">folder/</text>`|<svg width="56" height="16"><text x="2" y="12">folder/</text></svg>
|`/usr/bin`   |`<text x="2" y="12">/usr/bin</text>`|<svg width="72" height="16"><text x="2" y="12">/usr/bin</text></svg>

**Example 3:** Aligned next to a drawing element

|text | svg | rendered
|-----|-----|-----------
|`folder/`<br>&nbsp;&nbsp;`/usr/bin`|`<line x1="40" y1="32" x2="56" y2="0"/>`<br>`<text x="2" y="12">folder</text>`<br>`<text x="10" y="28">/usr</text>`<br>`<text x="58" y="28">bin</text>`|<svg width="100" height="32"><line x1="40" y1="32" x2="56" y2="0"/><text x="2" y="12">folder</text><text x="10" y="28">/usr</text><text x="58" y="28">bin</text></svg>

--------------------
## Backward slash `\`
- if next to an alphanumeric character, then it will be rendered as text
- if connects to a drawing element, then it will be rendered as a slanted line to the left
- if alone then it will be rendered as slanted line to the left

**Example 1:**
```
 \
  \
```

|text    | svg     | rendered
|--------|---------|---------------
|`\\`<br>&nbsp;`\\`|`<line x1="0" y1="0" x2="16" y2="32"/>`|<svg width="16" height="32"><line x1="0" y1="0" x2="16" y2="32"/></svg>

**Example 2:**

|text     | svg | rendered
|---------|-----|-----------
|`C:\\users`|`<text x="2" y="12">C:\\users</text>`|<svg width="72" height="16"><text x="2" y="12">C:\\users</text></svg>

## Cross  `+`
- If the left side is horizontal, then this will become a horizontal line connecting midway to the left
- If the right side is horizontal, then this will come a horizontal line connecting midway to the right
- If top of this character is a vertical line, then this will become a vertical line connecting midway to the the top
- If bottom of this is a vertical line, then this will become a vertical line connecting miday to bottom

**Example 1:**

|text | svg | rendered
|-----|-----|----------
|`-+` |`<line x1="0" y1="4" x2="12" y2="4"/>`|<svg width="32" height="8"><line x1="0" y1="4" x2="12" y2="4"/></svg>
|`+-` |`<line x1="4" y1="4" x2="16" y2="4"/>`|<svg width="32" height="8"><line x1="4" y1="4" x2="16" y2="4"/></svg>
|`\|`<br>`+`   |`<line x1="4" y1="0" x2="4" y2="24"/>`|<svg width="8" height="32"><line x1="4" y1="0" x2="4" y2="24"/></svg>
|`+`<br>`\|`   |`<line x1="4" y1="8" x2="4" y2="32"/>`|<svg width="8" height="32"><line x1="4" y1="8" x2="4" y2="32"/></svg>

## Dot and Comma (.,)
- Primary purpose is to make rounded corners, top_left rounded corner and top_right rounded corner

** Example 1:**

|text | svg | rendered
|-----|-----|---------
|`.-`<br>`\|``|

