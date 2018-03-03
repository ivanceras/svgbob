use fragments::Block;
use fragments::Block::{
    A,B,C,D,E,
    F,G,H,I,J,
    K,L,M,N,O,
    P,Q,R,S,T,
    U,V,W,X,Y
};

use fragments::Fragment;
use fragments::{
    line,
    arc,
};

use properties::Properties;
use properties::PointBlock;


pub fn box_drawing(ch: &char) -> (Vec<Block>, Vec<Fragment>) {
    let a = &PointBlock::block(A);
    let b = &PointBlock::block(B);
    let c = &PointBlock::block(C);
    let d = &PointBlock::block(D);
    let e = &PointBlock::block(E);
    let _f = &PointBlock::block(F);
    let _g = &PointBlock::block(G);
    let h = &PointBlock::block(H);
    let _i = &PointBlock::block(I);
    let _j = &PointBlock::block(J);
    let k = &PointBlock::block(K);
    let l = &PointBlock::block(L);
    let m = &PointBlock::block(M);
    let n = &PointBlock::block(N);
    let o = &PointBlock::block(O);
    let p = &PointBlock::block(P);
    let q = &PointBlock::block(Q);
    let r = &PointBlock::block(R);
    let s = &PointBlock::block(S);
    let t = &PointBlock::block(T);
    let u = &PointBlock::block(U);
    let v = &PointBlock::block(V);
    let w = &PointBlock::block(W);
    let x = &PointBlock::block(X);
    let y = &PointBlock::block(Y);
    //////////////////////////////
    //
    //  Static are all Strong signal
    //  and are used Box Drawing
    //
    ////////////////////////////////
    if ch.in_any(vec!['─','━']){
        (vec![K,O],
         vec![line(k,o)]
        )
    }
    else if ch.is('¯'){ // overscore?
        (vec![A,E],
         vec![line(a,e)]
         )
    }
    else if ch.is('│'){
        (vec![C,W],
         vec![line(c,w)]
        )
    }
    else if ch.is('┃'){
        (vec![E,Y],
         vec![line(e,y)]
         )
    }
    else if ch.is('╭'){
        (vec![O,W],
         vec![
            arc(o,r,2),
            line(r,w)
            ]
        )
    }
    else if ch.any("┌┍┎┏"){
        (vec![W,O],
         vec![
            line(m,w),
            line(m,o)
        ])
    }
    else if ch.is('╮'){
        (vec![K,W],
         vec![
            line(w,r),
            arc(r,k,2)
         ])
    }
    else if ch.any("┐┑┒┓"){
        (vec![W,K],
         vec![
             line(m,w),
             line(m,k)
        ])
    }
    else if ch.is('╰'){
        (vec![C,O],
         vec![line(c,h),
            arc(h,o,2)
            ]
        )
    }
    else if ch.any("┗└┕┖"){
        (vec![C,O],
         vec![line(m,c),
            line(m,o)
        ])
    }
    else if ch.is('╯'){
        (vec![C,K],
         vec![line(c,h),
            arc(k,h,2)
            ]
        )
    }
    else if ch.any("┘┙┚┛"){
        (vec![C,K],
         vec![line(m,c),
            line(m,k)
        ])
    }
    else if ch.any("┼┽┾┿╀╁╂╃╄╅╆╇╈╉╊╋"){
        (vec![C,W,K,O],
         vec![line(m,c),
            line(m,w),
            line(m,k),
            line(m,o),
        ])
    }
    else if ch.any("┬┭┮┯┰┱┲┳"){
        (vec![W,K,O],
         vec![line(m,w),
            line(m,k),
            line(m,o)
        ])
    }
    else if ch.any("┴┵┶┷┸┹┺┻"){
        (vec![C,K,O],
         vec![line(m,c),
            line(m,k),
            line(m,o)
        ])
    }
    else if ch.any("├┝┞┟┠┡┢┣"){
        (vec![C,W,O],
         vec![line(m,c),
            line(m,w),
            line(m,o)
        ])
    }
    else if ch.any("┤┥┦┧┨┩┪┫"){
        (vec![C,W,K],
         vec![line(m,c),
            line(m,w),
            line(m,k)
        ])
    }
    else if ch.is('║'){
        (vec![B,V,D,X],
         vec![line(b,v),
            line(v,b),
            line(d,x),
            line(x,d),
        ])
    }
    else if ch.is('═'){
        (vec![K,O,P,T],
         vec![line(k,o),
            line(k,o),
            line(p,t),
        ])
    }
    else if ch.is('╔'){
        (vec![O,V,T,X],
         vec![line(o,l),
            line(l,v),
            line(t,s),
            line(s,x)
        ])
    }
    else if ch.is('╗'){
        (vec![K,X,P,V],
         vec![line(k,n),
            line(n,x),
            line(p,q),
            line(q,v)
        ])
    }
    else if ch.is('╚'){
        (vec![B,T,D,O],
         vec![line(b,q),
            line(q,t),
            line(d,n),
            line(n,o)
        ])
    }
    else if ch.is('╝'){
        (vec![P,K,B,D],
         vec![line(p,s),
            line(s,d),
            line(k,l),
            line(l,b)
        ])
    }
    else if ch.is('╒'){
        (vec![W,O,T],
         vec![
            line(m,w),
            line(m,o),
            line(r,t),
        ])
    }
    else if ch.is('╓'){
        (vec![O,V,X],
          vec![
              line(l,o),
              line(l,v),
              line(n,x)
        ])
    }
    else if ch.is('╬'){
        (vec![B,D,V,X,K,P,O,T],
         vec![
            line(b,l), line(l,k),
            line(p,q), line(q,v),
            line(d,n), line(n,o),
            line(t,s), line(s,x)
         ])
    }
    else if ch.is('╦'){
        (vec![K,O,P,V,T,X],
         vec![
            line(k,o),
            line(p,q),
            line(q,v),
            line(t,s),
            line(s,x)
        ])
    }
    else if ch.is('╩'){
        (vec![P,T,K,B,D,O],
         vec![
            line(p,t),
            line(k,l),
            line(l,b),
            line(d,n),
            line(n,o)
        ])
    }
    else if ch.is('╠'){
        (vec![B,V,D,O,T,X],
         vec![
            line(b,v),
            line(d,n),
            line(n,o),
            line(t,s),
            line(s,x)
         ])
    }
    else if ch.is('╣'){
        (vec![D,X,B,K,P,V],
         vec![
            line(d,x),
            line(b,l), line(l,k),
            line(p,q), line(q,v)
         ])
    }
    else if ch.is('╱'){
        (vec![U,E],
         vec![line(u,e)]
        )
    }
    else if ch.is('╲'){
        (vec![A,Y],
         vec![line(a,y)],
        )
    }
    else if ch.is('╳'){
        (vec![A,Y,U,E],
         vec![line(a,y),line(u,e)]
        )
    }
    else if ch.is('╒'){
        (vec![W,O,T],
        vec![
            line(m,w),
            line(m,o),
            line(r,t)
        ])
    }
    else if ch.is('╓'){
        (vec![O,V,X],
        vec![
            line(l,o),
            line(l,v),
            line(n,x)
        ])
    }
    else if ch.is('╞'){
        (vec![C,W,O,T],
        vec![
            line(c,w),
            line(m,o),
            line(r,t)
        ])
    }
    else if ch.is('╡'){
        (vec![W,C,K,P],
        vec![
            line(c,w),
            line(k,m),
            line(p,r)
        ])
    }
    else if ch.is('╤'){
        (vec![K,O,P,T,W],
        vec![
            line(k,o),
            line(p,t),
            line(r,w)
        ])
    }
    else if ch.is('╥'){
        (vec![K,O,V,X],
        vec![
            line(k,o),
            line(l,v),
            line(n,x)
        ])
    }
    else if ch.is('╖'){
        (vec![K,X,V],
        vec![
            line(k,n),
            line(n,x),
            line(l,v)
        ])
    }
    else if ch.is('╙'){
        (vec![O,B,D],
        vec![
            line(l,o),
            line(l,b),
            line(n,d)
        ])
    }
    else if ch.is('╜'){
        (vec![K,B,D],
        vec![
            line(k,n),
            line(l,b),
            line(n,d)
        ])
    }
    else if ch.is('╕'){
        (vec![W,K,P],
        vec![
            line(m,w),
            line(k,m),
            line(p,r)
        ])
    }
    else if ch.is('╛'){
        (vec![C,P,K],
        vec![
            line(c,r),
            line(r,p),
            line(k,m)
        ])
    }
    else if ch.is('╘'){
        (vec![C,O,T],
        vec![
            line(c,r),
            line(m,o),
            line(r,t)
        ])
    }
    else if ch.is('╢'){
        (vec![D,B,K,V,X],
        vec![
            line(d,x),
            line(b,v),
            line(k,l)
        ])
    }
    else if ch.is('╟'){
        (vec![D,X,B,V,O],
        vec![
            line(d,x),
            line(b,v),
            line(n,o)
        ])
    }
    else if ch.is('╪'){
        (vec![C,W,K,O,P,T],
        vec![
            line(c,w),
            line(k,o),
            line(p,t)
        ])
    }
    else if ch.is('╧'){
        (vec![K,O,P,T,C],
        vec![
            line(k,o),
            line(p,t),
            line(c,m)
        ])
    }
    else if ch.is('╫'){
        (vec![K,O,B,V,D,X],
        vec![
            line(k,o),
            line(b,v),
            line(d,x)
        ])
    }
    else if ch.is('╨'){
        (vec![K,O,B,D],
        vec![
            line(k,o),
            line(b,l),
            line(d,n)
        ])
    }
    else{
        (vec![], vec![])
    }
}
