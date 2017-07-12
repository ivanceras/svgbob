
/// exact location of point
/// relative to the Character Block
/// The block is divided in to 5x5 small blocks
enum Block{
    A,B,C,D,E,
    F,G,H,I,J,
    K,L,M,N,O,
    P,Q,R,S,T,
    U,V,W,X,Y
}

impl Block{
    
    /// +*-_|\/
    fn static_list() -> Vec<(char, Vec<Block>)> {
        let mut list = vec![
        ('+', vec![C,K,O,W]),
        ('*', vec![A,C,E,K,O,U,W,Y]),
        ('-', vec![K,O]),
        ('_', vec![U,Y]),
        ('|', vec![C,W]),
        ('\\', vec![A,Y]),
        ('/', vec![E,U])
        ];
    }

    fn elements(){
        [
        ('+', vec![line(K,O),C,W)]),
        ('*', vec![solid_circle(M,tw2)],
        ('-', vec![line(K,O)]
        ]
    }

    /// +.,oO*~`'()[]
    fn dynamic_list() Vec<(char, Vec<Block>) {
        vec![
        ('+', vec![A,C,E,K,O,U,W,Y]),
        ('.', vec![A,B,C,D,E,F,J,K,O,P,T,U,V,W,X,Y]),
        (',', vec![A,B,C,D,E,F,J,K,O,P,T,U,V,W,X,Y]),
        ('o', vec![A,B,C,D,E,F,J,K,O,P,T,U,V,W,X,Y]),
        ('O', vec![A,B,C,D,E,F,J,K,O,P,T,U,V,W,X,Y]),
        ('*', vec![A,B,C,D,E,F,J,K,O,P,T,U,V,W,X,Y]),
        ('`', vec![A,C,E,F,J,K,O,P,T,U,Y]),
        ('\'', vec![A,C,E,F,J,K,O,P,T,U,Y]),
        ('(', vec![C,W,E,Y]),
        (')', vec![A,U,C,W]),
        ('x', vec![A,Y,E,U])
        ('X', vec![A,Y,E,U])
        ]
    }

    /// which points connects to neighbor points
    fn connection_list(&self) -> Vec<(Dir,Block)> {
        (A, vec![
                (Top,U),
                (TopLeft, Y),
                (Left,E),
            ]
        ),
        (B, vec![
                (Top,B),
                (Top,W),
                (Top,U)
             ]
        ),

    }

}


/// accumulates elements for each location
struct Fragment{
    /// the location of this Drawing element relative to the Grid
    loc: Loc,
    /// the elements at this locatio
    elements: Vec<Element>,
    /// whether or not the character in this location
    /// reacts to adjascent cell and will be rendered as svg element
    /// otherwise will be text
    interacted: bool,
    /// the character is consumed by the neighboring character
    /// thus will not be rendered by it's default elemental behavior
    /// if false, then the element will be rendered to its default svg
    /// deformed is also consumed
    /// since it's render is now delegated to its adjacent character
    consumed: bool,
}

struct Accumulator(Vec<Fragment>);

impl Accumulator{
    
    /// push elements at this location as interacted and NOT consumed
    fn push(&mut self, loc: &Loc, elements: Vec<Element>){
        self.0.push(
            Fragment{
                loc: loc.clone(),
                elements: elements,
                interacted: true,
                consumed: false,
            }
        )
    }

    /// push elements at this location as interacted and consumed
    fn push_consumed(&mut self, loc: &Loc, elements: Vec<Element>){
        self.0.push(
            Fragment{
                loc: loc.clone(),
                elements: elements,
                interacted: true,
                consumed: true,
            }
        )
    }

}

