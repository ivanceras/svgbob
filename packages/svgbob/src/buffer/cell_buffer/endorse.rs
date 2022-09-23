use crate::{
    fragment::{Bounds, Rect},
    Fragment,
};

/// if a group of fragment can be endorse as rect, return the bounds point for the
/// rectangle
pub fn endorse_rect(fragments: &[&Fragment]) -> Option<Rect> {
    if is_rect(fragments) {
        let is_any_broken =
            fragments.iter().any(|fragment| fragment.is_broken());
        let all_points = fragments.iter().fold(vec![], |mut acc, frag| {
            let (p1, p2) = frag.bounds();
            acc.push(p1);
            acc.push(p2);
            acc
        });
        let min = all_points.iter().min();
        let max = all_points.iter().max();
        if let (Some(min), Some(max)) = (min, max) {
            Some(Rect::new(*min, *max, false, is_any_broken))
        } else {
            None
        }
    } else {
        None
    }
}

/// group of fragments can be check if they form:
/// - rectangle
fn is_rect(fragments: &[&Fragment]) -> bool {
    if fragments.len() == 4 {
        let parallels = parallel_aabb_group(fragments);
        if parallels.len() == 2 {
            let (a1, a2) = parallels[0];
            let (b1, b2) = parallels[1];
            let line_a1 = fragments[a1].as_line().expect("expecting a line");
            let line_b1 = fragments[b1].as_line().expect("expecting a line");
            let line_a2 = fragments[a2].as_line().expect("expecting a line");
            let line_b2 = fragments[b2].as_line().expect("expecting a line");
            line_a1.is_touching_aabb_perpendicular(line_b1)
                && line_a2.is_touching_aabb_perpendicular(line_b2)
        } else {
            false
        }
    } else {
        false
    }
}

/// qualifications:
///  - 8 fragments
///  - 2 parallell pair
///  - 4 aabb right angle arc (top_left, top_right, bottom_left, bottom_right)
///  - each of the right angle touches 2 lines that are aabb_perpendicular
pub fn endorse_rounded_rect(fragments: &[&Fragment]) -> Option<Rect> {
    if let (true, arc_radius) = is_rounded_rect(fragments) {
        let is_any_broken =
            fragments.iter().any(|fragment| fragment.is_broken());
        let all_points = fragments.iter().fold(vec![], |mut acc, frag| {
            let (p1, p2) = frag.bounds();
            acc.push(p1);
            acc.push(p2);
            acc
        });
        let min = all_points.iter().min();
        let max = all_points.iter().max();
        if let (Some(min), Some(max)) = (min, max) {
            //TODO: compute the radius from
            Some(Rect::rounded_new(
                *min,
                *max,
                false,
                arc_radius.expect("expecting arc radius"),
                is_any_broken,
            ))
        } else {
            None
        }
    } else {
        None
    }
}

fn is_rounded_rect(fragments: &[&Fragment]) -> (bool, Option<f32>) {
    if fragments.len() == 8 {
        let parallels = parallel_aabb_group(fragments);
        let right_arcs = right_angle_arcs(fragments);
        //TODO: throroughly check the arc composition to be top_left, top_right, bottom_left,
        //bottom_right
        if parallels.len() == 2 && right_arcs.len() == 4 {
            let first_right_arc_index = right_arcs[0];
            let arc_fragment = &fragments[first_right_arc_index];
            let arc_radius =
                arc_fragment.as_arc().expect("expecting an arc").radius;
            let (a1, a2) = parallels[0];
            let (b1, b2) = parallels[1];
            let line_a1 = fragments[a1].as_line().expect("expecting a line");
            let line_b1 = fragments[b1].as_line().expect("expecting a line");
            let line_a2 = fragments[a2].as_line().expect("expecting a line");
            let line_b2 = fragments[b2].as_line().expect("expecting a line");
            let passed = line_a1.is_aabb_perpendicular(line_b1)
                && line_a2.is_aabb_perpendicular(line_b2);
            (passed, Some(arc_radius))
        } else {
            (false, None)
        }
    } else {
        (false, None)
    }
}

/// return the index of the fragments that are right angle arc
fn right_angle_arcs(fragments: &[&Fragment]) -> Vec<usize> {
    fragments
        .iter()
        .enumerate()
        .filter_map(|(index, frag)| {
            if let Some(arc) = frag.as_arc() {
                if arc.is_aabb_right_angle_arc() {
                    Some(index)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

/// return the indexes of the fragments that are aabb parallel
fn parallel_aabb_group(fragments: &[&Fragment]) -> Vec<(usize, usize)> {
    let mut parallels = vec![];
    for (index1, frag1) in fragments.iter().enumerate() {
        for (index2, frag2) in fragments.iter().enumerate() {
            if index1 != index2
                && !parallels.iter().any(|(pair1, pair2)| {
                    index1 == *pair1
                        || index1 == *pair2
                        || index2 == *pair1
                        || index2 == *pair2
                })
                && frag1.is_aabb_parallel(frag2)
            {
                parallels.push((index1, index2));
            }
        }
    }
    parallels
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{buffer::CellGrid, fragment::line};

    #[test]
    fn test_parallel_grouping() {
        let a = CellGrid::a();
        let e = CellGrid::e();
        let u = CellGrid::u();
        let y = CellGrid::y();

        let line_ae = line(a, e);
        let line_uy = line(u, y);
        let group = parallel_aabb_group(&[&line_ae, &line_uy]);
        println!("group: {:#?}", group);
        assert_eq!(group, vec![(0, 1)])
    }

    #[test]
    fn test_parallel_grouping_with4() {
        let a = CellGrid::a();
        let e = CellGrid::e();
        let u = CellGrid::u();
        let y = CellGrid::y();

        let line_ae = line(a, e);
        let line_uy = line(u, y);
        let line_au = line(a, u);
        let line_ey = line(e, y);
        let group = parallel_aabb_group(&[
            &line_ae,
            &line_au,
            &line_uy,
            &line_ey,
        ]);
        println!("group: {:#?}", group);
        assert_eq!(group, vec![(0, 2), (1, 3)]);

        let rect = endorse_rect(&[
            &line_ae,
            &line_au,
            &line_uy,
            &line_ey,
        ]);
        assert!(rect.is_some());
        assert_eq!(rect, Some(Rect::new(a, y, false, false)));
        assert!(is_rect(&[&line_ae, &line_au, &line_uy, &line_ey]));
    }

    #[test]
    fn parallel_and_perpendicular_but_not_touching_should_not_be_rect() {
        let a = CellGrid::a();
        let e = CellGrid::e();
        let u = CellGrid::u();
        let y = CellGrid::y();
        let g = CellGrid::g();
        let q = CellGrid::q();
        let i = CellGrid::i();
        let s = CellGrid::s();

        let line_ae = line(a, e);
        let line_uy = line(u, y);
        let line_gq = line(g, q);
        let line_is = line(i, s);

        assert!(!is_rect(&[&line_ae, &line_uy, &line_gq, &line_is]));
    }
}
