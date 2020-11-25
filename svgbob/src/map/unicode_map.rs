use crate::{
    buffer::{
        fragment::PolygonTag::{
            ArrowBottom, ArrowBottomLeft, ArrowBottomRight, ArrowLeft, ArrowRight, ArrowTop,
            ArrowTopLeft, ArrowTopRight, DiamondBullet,
        },
        Cell, CellGrid,
    },
    fragment::{arc, broken_line, circle, line, polygon, rect, Fragment},
    Property,
};
use lazy_static::lazy_static;
use std::collections::{BTreeMap, HashMap};

lazy_static! {
    /// a lookup table for character and their corresponding shapes
    /// static ref to provide a one time processing
    /// Characters found in
    /// - https://en.wikipedia.org/wiki/Box-drawing_character
    /// - http://asciimath.org/
    /// - https://en.wikipedia.org/wiki/Geometric_Shapes
    /// - https://www.unicode-search.net/unicode-namesearch.pl?term=CIRCLE
    /// - http://xahlee.info/comp/unicode_common_symbols.html
    /// - http://shapecatcher.com/
    ///
    /// # Inspect unicode character online:
    ///  https://apps.timwhitlock.info/unicode/inspect
    ///
    /// ```ignore
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
    /// ```                      V W X
    pub static ref UNICODE_FRAGMENTS : BTreeMap<char, Vec<Fragment>>  =  {
        #![allow(unused)]

        let a = CellGrid::a();
        let b = CellGrid::b();
        let c = CellGrid::c();
        let d = CellGrid::d();
        let e = CellGrid::e();
        let f = CellGrid::f();
        let g = CellGrid::g();
        let h = CellGrid::h();
        let i = CellGrid::i();
        let j = CellGrid::j();
        let k = CellGrid::k();
        let l = CellGrid::l();
        let m = CellGrid::m();
        let n = CellGrid::n();
        let o = CellGrid::o();
        let p = CellGrid::p();
        let q = CellGrid::q();
        let r = CellGrid::r();
        let s = CellGrid::s();
        let t = CellGrid::t();
        let u = CellGrid::u();
        let v = CellGrid::v();
        let w = CellGrid::w();
        let x = CellGrid::x();
        let y = CellGrid::y();

        /// cellgrids that have no names
        /// just name them with coordinate locations
        let _01 = CellGrid::point(0, 1);
        let _11 = CellGrid::point(1, 1);
        let _21 = CellGrid::point(2, 1);
        let _31 = CellGrid::point(3, 1);
        let _41 = CellGrid::point(4, 1);
        let _03 = CellGrid::point(0, 3);
        let _13 = CellGrid::point(1, 3);
        let _23 = CellGrid::point(2, 3);
        let _33 = CellGrid::point(3, 3);
        let _43 = CellGrid::point(4, 3);
        let _05 = CellGrid::point(0, 5);
        let _15 = CellGrid::point(1, 5);
        let _25 = CellGrid::point(2, 5);
        let _35 = CellGrid::point(3, 5);
        let _45 = CellGrid::point(4, 5);
        let _07 = CellGrid::point(0, 7);
        let _17 = CellGrid::point(1, 7);
        let _27 = CellGrid::point(2, 7);
        let _37 = CellGrid::point(3, 7);
        let _47 = CellGrid::point(4, 7);

        let unit1 = Cell::unit(1);
        let unit2 = Cell::unit(2);
        let unit3 = Cell::unit(3);
        let unit4 = Cell::unit(4);
        let unit5 = Cell::unit(5);
        let unit6 = Cell::unit(6);
        let unit7 = Cell::unit(7);
        let unit8 = Cell::unit(8);

        let map = vec![

            // dash
            ('─', vec![line(k, o)]),

            // en dash, E2 80 93
            ('–', vec![line(k, o)]),

            // em dash, E2 80 94
            ('—', vec![line(k, o)]),

            // broken horizontal line
            ('┄', vec![broken_line(k, o)]),

            // vertical line
            ('│', vec![line(c, w)]),

            // broken vertical line
            ('╎', vec![broken_line(c, w)]),

            // alternate broken vertical line
            ('┊', vec![broken_line(c, w)]),

            // alternate broken vertical line
            ('┆', vec![broken_line(c, w)]),

            // slant left
            ('╲', vec![line(a, y)]),

            // slant right
            ('╱', vec![line(e, u)]),

            // X line
            ('╳', vec![line(a, y), line(e, u)]),

            // cross line, plus
            ('┼', vec![line(c,w), line(k,o)]),

            // parallel horizontal line, equal
            ('═', vec![line(_03, _43), line(_05, _45)]),

            // square box
            ('□', vec![line(a,e), line(d,x),line(y,u), line(b,v)]),

            // vertical line left
            ('▏', vec![line(b,v)]),

            // vertical line right
            ('▕', vec![line(d,x)]),

            // double vertical line
            ('║', vec![line(b,v), line(d,x)]),

            // angle left
            ('∠', vec![line(e,u), line(u,y)]),

            // angle top
            ('⋀', vec![line(u,c), line(c,y)]),

            // triangle
            ('△', vec![line(c,y), line(y,u), line(u,c)]),

            // arrow down matching v
            //
            //  |
            //  ▾
            ('▾', vec![polygon(vec![f,j,w], true, vec![ArrowBottom])]),
            //
            //  ▴
            //  |
            ('▴', vec![polygon(vec![p,c,t], true, vec![ArrowTop])]),
            //
            // --▸
            //
            ('▸', vec![polygon(vec![f,o,p], true, vec![ArrowRight])]),
            //
            // ◂--
            //
            ('◂', vec![polygon(vec![j,k,t], true, vec![ArrowLeft])]),

            ('◆', vec![polygon(vec![k,h,o,r,k], true, vec![DiamondBullet])]),
            ('▪', vec![rect(f,t,true, false)]),

            // 1/8
            ('▁',vec![rect(_01,y,true,false)]),
            // 2/8
            ('▂',vec![rect(p,y,true,false)]),
            // 3/8
            ('▃',vec![rect(_05,y,true,false)]),
            // 4/8
            ('▄',vec![rect(k,y,true,false)]),
            // 5/8
            ('▅',vec![rect(_03,y,true,false)]),
            // 6/8
            ('▆',vec![rect(f,y,true,false)]),
            // 7/8
            ('▇', vec![rect(_01,y,true,false)]),
            // 8/8
            ('█', vec![rect(a,y,true,false)]),

            //              /
            // --▶    ▶    ▶
            //         \
            //
            ('▶', vec![polygon(vec![f,o,p], true, vec![ArrowRight, ArrowTopLeft, ArrowBottomLeft])]),

            //      \
            // ◀--   ◀    ◀
            //           /
            //
            ('◀', vec![polygon(vec![j,k,t], true, vec![ArrowLeft, ArrowBottomRight, ArrowTopRight])]),


            // L shape bottom-left box
            ('⌊', vec![line(a,u), line(u,w)]),

            // not equal sign
            ('≠', vec![line(_03,_43), line(_05, _45), line(e,u)]),

            // cross with double horizontal
            ('╪', vec![line(_03, _43), line(_05, _45), line(c,w)]),
            // cross with double vertical
            ('╫', vec![line(k,o), line(b,v), line(d,x)]),

            ('⊕', vec![line(c,w), line(k, o), circle(m, unit2, false)]),
            // Big O
            ('○', vec![circle(m, unit2, false)]),
            ('⦵', vec![circle(m, unit2, false), line(k,o)]),
            ('●', vec![circle(m, unit2, true)]),
            ('￮', vec![circle(m, unit1, true)]),
            ('┌', vec![line(m,o), line(m,w)]),
            ('┐', vec![line(m,k), line(m,w)]),

            ('┘', vec![line(c,m), line(k,m)]),

            ('└', vec![line(c,m), line(m,o)]),

            ('├', vec![line(c,w), line(m,o)]),

            ('┤', vec![line(c,w), line(k,m)]),

            ('┬', vec![line(k,o), line(m,w)]),

            ('┴', vec![line(k,o), line(c,m)]),

            /// rounded top left
            ('╭', vec![arc(o, r, unit2), line(r, w)]),
            /// rounded top right
            ('╮', vec![line(w, r), arc(r, k, unit2)]),
            /// rounded bottom-left
            ('╰', vec![line(c, h), arc(h, o, unit2)]),
            /// rounded bottom-right
            ('╯', vec![line(c, h), arc(k, h, unit2)]),

            // ◜
            ('◜', vec![arc(e, m, unit4), line(m, w)]),
            // ◝
            ('◝', vec![arc(m, a, unit4), line(m, w)]),
            // ◟
            ('◟', vec![arc(m, y, unit4), line(c, m)]),
            // ◞
            ('◞', vec![arc(u, m, unit4), line(m, c)]),

            ('║', vec![line(b, v), line(v, b), line(d, x), line(x, d)]),

            ('═', vec![line(k, o), line(p, t)]),

            ('╔', vec![line(o, l), line(l, v), line(t, s), line(s, x)]),

            ('╗', vec![line(k, n), line(n, x), line(p, q), line(q, v)]),

            ('╚', vec![line(b, q), line(q, t), line(d, n), line(n, o)]),

            ('╝', vec![line(p, s), line(s, d), line(k, l), line(l, b)]),

            ('╒', vec![line(m, w), line(m, o), line(r, t)]),

            ('╓', vec![line(l, o), line(l, v), line(n, x)]),

            ('╬', vec![
                        line(b, l),
                        line(l, k),
                        line(p, q),
                        line(q, v),
                        line(d, n),
                        line(n, o),
                        line(t, s),
                        line(s, x),
                    ]),

            ('╦', vec![line(k, o), line(p, q), line(q, v), line(t, s), line(s, x)]),

            ('╩', vec![line(p, t), line(k, l), line(l, b), line(d, n), line(n, o)]),

            ('╠', vec![line(b, v), line(d, n), line(n, o), line(t, s), line(s, x)]),

            ('╣', vec![line(d, x), line(b, l), line(l, k), line(p, q), line(q, v)]),

            ('╒', vec![line(m, w), line(m, o), line(r, t)]),

            ('╓', vec![line(l, o), line(l, v), line(n, x)]),

            ('╞', vec![line(c, w), line(m, o), line(r, t)]),

            ('╡', vec![line(c, w), line(k, m), line(p, r)]),

            ('╤', vec![line(k, o), line(p, t), line(r, w)]),

            ('╥', vec![line(k, o), line(l, v), line(n, x)]),

            ('╖', vec![line(k, n), line(n, x), line(l, v)]),

            ('╙', vec![line(l, o), line(l, b), line(n, d)]),

            ('╜', vec![line(k, n), line(l, b), line(n, d)]),

            ('╕', vec![line(m, w), line(k, m), line(p, r)]),

            ('╛', vec![line(c, r), line(r, p), line(k, m)]),

            ('╘', vec![line(c, r), line(m, o), line(r, t)]),

            ('╢', vec![line(d, x), line(b, v), line(k, l)]),

            ('╟', vec![line(d, x), line(b, v), line(n, o)]),

            ('╪', vec![line(c, w), line(k, o), line(p, t)]),

            ('╧', vec![line(k, o), line(p, t), line(c, m)]),

            ('╫', vec![line(k, o), line(b, v), line(d, x)]),

            ('╨', vec![line(k, o), line(b, l), line(d, n)]),

            // TODO:
            // parenthesis like: ⟮ ⟯（ ）
            //
            // ∈ ≡ ≤ ≥ ÷ ≠ · × ¬ ↑↓ ∧ ∨ ≈ ± ∃ ∀ ⊃ ⊂ ∪ ∩ ⊖ ⊕ « »

            // corners  ੮   ᓕ ᓕ ੭ ᜪ フ ᘄ  މ
            //  ረ ᓚ ᘇ   د  ১  ১
            //  ر ◞  ᓗ  ﺭ
            //  ⁔  ⏝
            //
            //  𝁼  ⌣
            //  メ
        ];
        // sort the fragments first before putting into the btreemap
        let mut btree = BTreeMap::new();
        for (ch,mut fragments) in map.into_iter(){
            fragments.sort();
            btree.insert(ch, fragments);
        }
        btree
    };

    /// the reverse of shape to character lookup
    pub static ref FRAGMENTS_UNICODE: BTreeMap<&'static Vec<Fragment>, char> =
        UNICODE_FRAGMENTS.iter()
            .fold(BTreeMap::new(), |mut acc, (ch, shapes)| {acc.insert(shapes, *ch); acc});

    pub static ref UNICODE_PROPERTIES: HashMap<char, Property> =
        UNICODE_FRAGMENTS.iter()
            .fold(HashMap::new(), |mut acc, (ch, frags)| {acc.insert(*ch, Property::with_strong_fragments(*ch, frags.clone())); acc});

}
