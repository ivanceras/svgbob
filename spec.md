## Svgbob specification

Svgbob is a diagramming model which uses common typing characters to approximate the shape

Characters:
    -  dash
    _ underscore
    |  or
    /  forward slash
    \  backslash
    +  plus
    .  dot
    ,  comma
    '  single quote
    `  backtick
    (  open parenthesis
    )  close parenthesis
    [  open braces
    ]  close braces
    *  asterisk
    o  lowercase letter `o`
    O  uppercase letter `O`
    ^  caret
    v  lowercase letter `v`
    V  lowercase letter `V`
    <  lesser than
    >  greater than
    =  equal sign

## Principle
The rendered shape should closely resembles to that of the ascii
drawing / formation.

## Horizontal line
    Dash line `-` if alone it will be treated as text
    2 or more next to left or right will be rendered as line

    Example 1:
     --   <line x1 y1 x2 y2>

    Example 2:
     --------   <line x1 y1 x2 y2>

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
   Or symbol `|` will treated as text when alone

    Example 1:
        |    <line x1 y1 x2 y2></line>
        |

    use as or expression
    Example 2: 
        a||b <text>a||b</text>


## forward slash
