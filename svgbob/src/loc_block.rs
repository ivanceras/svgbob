use block::Block::{A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y};
use loc::Loc;
use point::Point;
use point_block::PointBlock;
use settings::Settings;

pub struct LocBlock {
    pub loc: Loc,
    pub settings: Settings,
}

impl LocBlock {
    fn text_width(&self) -> f32 {
        self.settings.text_width
    }

    fn text_height(&self) -> f32 {
        self.settings.text_height
    }

    fn loc_x(&self) -> f32 {
        self.loc.x as f32
    }

    fn loc_y(&self) -> f32 {
        self.loc.y as f32
    }

    /// 1 unit in x dimension is 1/4 of the textwidth
    /// used in calculating the radius, adjustments
    /// in the Fragment PointBlock
    pub fn unit_x(&self) -> f32 {
        self.text_width() * 1.0 / 4.0
    }

    /// 1 unit along y dimension is 1/4 of the textwidth
    #[allow(unused)]
    fn unit_y(&self) -> f32 {
        self.text_height() * 1.0 / 4.0
    }

    /// x coordinate on increment of 1/4 of text width
    fn x0(&self) -> f32 {
        self.loc_x() * self.text_width()
    }

    fn x1(&self) -> f32 {
        (self.loc_x() + 1.0 / 4.0) * self.text_width()
    }

    fn x2(&self) -> f32 {
        (self.loc_x() + 1.0 / 2.0) * self.text_width()
    }

    fn x3(&self) -> f32 {
        (self.loc_x() + 3.0 / 4.0) * self.text_width()
    }

    fn x4(&self) -> f32 {
        (self.loc_x() + 1.0) * self.text_width()
    }

    /// y coordinate on increment of 1/4 of text_height
    fn y0(&self) -> f32 {
        self.loc_y() * self.text_height()
    }

    fn y1(&self) -> f32 {
        (self.loc_y() + 1.0 / 4.0) * self.text_height()
    }

    fn y2(&self) -> f32 {
        (self.loc_y() + 1.0 / 2.0) * self.text_height()
    }

    fn y3(&self) -> f32 {
        (self.loc_y() + 3.0 / 4.0) * self.text_height()
    }

    fn y4(&self) -> f32 {
        (self.loc_y() + 1.0) * self.text_height()
    }

    /// 1st row a,b,c,d,e
    fn a(&self) -> Point {
        Point::new(self.x0(), self.y0())
    }

    fn b(&self) -> Point {
        Point::new(self.x1(), self.y0())
    }

    fn c(&self) -> Point {
        Point::new(self.x2(), self.y0())
    }

    fn d(&self) -> Point {
        Point::new(self.x3(), self.y0())
    }

    fn e(&self) -> Point {
        Point::new(self.x4(), self.y0())
    }

    /// 2nd row f,g,h,i,j
    fn f(&self) -> Point {
        Point::new(self.x0(), self.y1())
    }

    fn g(&self) -> Point {
        Point::new(self.x1(), self.y1())
    }

    fn h(&self) -> Point {
        Point::new(self.x2(), self.y1())
    }

    fn i(&self) -> Point {
        Point::new(self.x3(), self.y1())
    }

    fn j(&self) -> Point {
        Point::new(self.x4(), self.y1())
    }

    /// 3rd row k,l,m,n,o
    fn k(&self) -> Point {
        Point::new(self.x0(), self.y2())
    }

    fn l(&self) -> Point {
        Point::new(self.x1(), self.y2())
    }

    fn m(&self) -> Point {
        Point::new(self.x2(), self.y2())
    }

    fn n(&self) -> Point {
        Point::new(self.x3(), self.y2())
    }

    fn o(&self) -> Point {
        Point::new(self.x4(), self.y2())
    }

    /// 4th row p,q,r,s,t
    fn p(&self) -> Point {
        Point::new(self.x0(), self.y3())
    }

    fn q(&self) -> Point {
        Point::new(self.x1(), self.y3())
    }

    fn r(&self) -> Point {
        Point::new(self.x2(), self.y3())
    }

    fn s(&self) -> Point {
        Point::new(self.x3(), self.y3())
    }

    fn t(&self) -> Point {
        Point::new(self.x4(), self.y3())
    }

    /// 5th row u,v,w,x,y
    fn u(&self) -> Point {
        Point::new(self.x0(), self.y4())
    }

    fn v(&self) -> Point {
        Point::new(self.x1(), self.y4())
    }

    fn w(&self) -> Point {
        Point::new(self.x2(), self.y4())
    }

    fn x(&self) -> Point {
        Point::new(self.x3(), self.y4())
    }

    fn y(&self) -> Point {
        Point::new(self.x4(), self.y4())
    }

    pub fn to_point(&self, pb: &PointBlock) -> Point {
        // move loc to the additional location relative to itself
        let loc = if let Some(ref pbloc) = pb.location {
            self.loc.from_location(&pbloc)
        } else {
            self.loc.clone()
        };
        let lb = LocBlock {
            loc: loc,
            settings: self.settings.clone(),
        };
        let mut p = match pb.block {
            A => lb.a(),
            B => lb.b(),
            C => lb.c(),
            D => lb.d(),
            E => lb.e(),
            F => lb.f(),
            G => lb.g(),
            H => lb.h(),
            I => lb.i(),
            J => lb.j(),
            K => lb.k(),
            L => lb.l(),
            M => lb.m(),
            N => lb.n(),
            O => lb.o(),
            P => lb.p(),
            Q => lb.q(),
            R => lb.r(),
            S => lb.s(),
            T => lb.t(),
            U => lb.u(),
            V => lb.v(),
            W => lb.w(),
            X => lb.x(),
            Y => lb.y(),
        };
        let unit = self.unit_x();
        p.adjust(pb.adjust_x * unit, pb.adjust_y * unit);
        p
    }
}
