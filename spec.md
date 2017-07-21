## Svgbob specification



Svgbob is a diagramming model which uses common typing characters to approximate the shape

Characters:
    -  dash for horizontal lines
    _ underscore for horizontal lines
    |  or symbol for vertical lines
    /  forward slash for lines slanted to the right
    \  backslash for lines slanted to the left
    +  plus for sharp intersection
    .  dot for rounded corner intersection
    ,  comma for rounded corner intersection
    '  single quote for rounded corner intersection
    `  backtick for rounded corner intersection
    (  open parenthesis rounded side of an arc
    )  close parenthesis rounded side of an arc
    [  open braces rectangular corner
    ]  close braces rectangular corner
    *  asterisk for emphasized intersection
    o  lowercase letter `o` for intersection
    O  uppercase letter `O` for intersection
    ^  caret for arrow up
    v  lowercase letter `v` for arrow down
    V  lowercase letter `V` for arrow down
    <  lesser than for arrow left
    >  greater than for arrow right
    =  equal sign double horizontal line
    x  lower case letter `x` for intersection
    X  upper case letter `X` for intersection

## Principle
The rendered shape should closely resembles to that of the ascii
drawing / formation.

## Non-goals
The goal is NOT to be able to make graphs and diagrams with less effort.

## Measurements
Text height is 2 times the text width, both should be a multiple of 4.
Default values:
    - text width = 8px;
    - text height = 16px;

## Horizontal line
    Dash line `-` will be rendered as horizontal line along the
    middle of the character block.
    if alone it will be rendered as text
    2 or more next to left or right will be rendered as line

    Example 1:
     --   <line x1="0" y1="4" x2="16" y2="4"/>

    Example 2:
     ----   <line x1="0" y1="4" x2="32" y2="4">

    Alone
    Example 3:
     -  <text>-</text>
    
    Used together with text
    Example:
     opt-in <text>opt-in</text>

##  Underscore 
    `_` will be rendered as lines, rendered as text when
        alone or used together with texts

    Example 1:
      __   <line x1 y1 x2 y2></line>

    Example 2:

      foo_bar   <text>foo_bar</text>

## Vertical line
   Or symbol `|` will be rendered as text when alone

    Example 1:
        |    <line x1 y1 x2 y2></line>
        |

    use as or expression
    Example 2: 
        a||b <text>a||b</text>


## Forward slash
  `/` will be rendered as text when alone
  Will be rendered as text if used together with text
    
    Example 1:
         /    <line x1 y1 x2 y2></line>
        /

    Example 2:
        folder/   <text>folder/</text>
        /usr/bin  <text>/usr/bin</text>

## Backward slash 
  `\` will be rendered as text when alone.
  Will be rendered as text if used together with text
  Will be rendered as line when it connects to other
  drawing characters

    Example 1:
        \     <line x1 y1 x2 y2></line>
         \
    Example 2:
        C:\\users    <text>C:\\users</text>

## Rounded corners

    .-
    |   rounded corner on the top left

    -.
     |  rounded corner on the top right

    |   |  rounded corner on the bottom left
    `-  '-

     |  rounded corner on the bottom right
    -'

## Circles
    o - will be rendered as circle only when connected to lines
        the radius is 1/2 of the text width
    O - will be rendered as circle only when connected to lines
        the radius is 3/4 of the text width
    .-.   
   (   )  - will be rendered as circle with a radius equal to the text height 
    `-'   

## Arrows
    <- arrow left
    -> arrow right
    ^  arrow up
    |

    |  arrow down
    v

    ^
     \  arrow to top left

     ^
    /   arrow to top right

     /  
    v   arrow to bottom left
  
    \
     v  arrow to bottom right
