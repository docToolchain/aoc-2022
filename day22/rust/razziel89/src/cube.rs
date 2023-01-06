use nalgebra::{Matrix3, Vector3};

use crate::data::{Point, Tile};
use anyhow::{Error, Result};
use std::cmp::Ordering;
use std::collections::HashMap;

const PI: f64 = std::f64::consts::PI;

pub type V3 = Vector3<isize>;
pub type M3 = Matrix3<isize>;

#[derive(Debug)]
pub struct PointData {
    pos: V3,
    north: V3,
    norm: V3,
}

#[derive(Debug)]
struct FieldNeighDat<'a> {
    pos: &'a Point,
    neighs: Vec<&'a Point>,
}

fn point_to_field_id(point: &Point, max: &Point, points_per_edge: &usize) -> usize {
    let ppe = *points_per_edge as isize;

    let max_x_field = max.x / ppe;
    // let max_y_field = max.y / ppe;

    let x_field = point.x / ppe;
    let y_field = point.y / ppe;

    (y_field * max_x_field + x_field) as usize
}

// Construct a rotation matrix containing only integers around an axis in multiples of 90 degrees.
// To do so with nalgebra, we need to move to floats for a while and convert back.
fn rotmat(axis: V3, angle_in_degrees: isize) -> Result<M3> {
    if angle_in_degrees % 90 != 0 {
        return Err(Error::msg("can only rotate by multiples of 90 degrees"));
    }
    let float_ax =
        Vector3::<f64>::from_iterator(axis.iter().map(|el| el.clone() as f64)).normalize();
    let ang_in_rads = PI * (angle_in_degrees as f64) / 180.0;
    let axang = float_ax * ang_in_rads;

    let float_rotmat = nalgebra::Rotation3::<f64>::new(axang).to_homogeneous();
    println!("{:?}\n", float_rotmat);

    // The resulting matrix also contains information about scaling or translation, but we do not
    // want that. Thus, ignore that information by taking the top left 3x3 matrix contained in thos
    // 4x4 matrix. Also convert to integers because we only have nice rotations.
    Ok(M3::from_iterator(
        float_rotmat.iter().enumerate().filter_map(|(idx, el)| {
            if idx % 4 < 3 && idx < 4 * 3 {
                Some(el.clone().round() as isize)
            } else {
                None
            }
        }),
    ))
}

pub fn build<'b>(
    occ_map: &'b HashMap<Point, Tile>,
    max: &Point,
    points_per_edge: usize,
) -> Result<HashMap<&'b Point, PointData>> {
    let points_per_field = points_per_edge * points_per_edge;

    let mut points = occ_map
        .iter()
        .filter(|(_point, tile)| tile != &&Tile::None)
        // .map(|(point, _tile)| PointData {
        //     pos: V3::new(point.x, point.y, 0),
        //     north: V3::new(0, 1, 0),
        //     norm: V3::new(0, 0, 1),
        // })
        .map(|(point, _tile)| point)
        .collect::<Vec<_>>();

    // Sort by field id first, then by x-coordinate and then by y-coordinate. That way, we can
    // easily find the field ID a point belongs to.
    points.sort_by(|el1, el2| {
        let field_cmp = point_to_field_id(&el1, max, &points_per_edge).cmp(&point_to_field_id(
            &el2,
            max,
            &points_per_edge,
        ));
        if field_cmp != Ordering::Equal {
            field_cmp
        } else {
            let x_cmp = el1.x.cmp(&el2.x);
            if x_cmp != Ordering::Equal {
                x_cmp
            } else {
                el1.y.cmp(&el2.y)
            }
        }
    });

    // Sanity check the size of the playing field. A cube has 6 sides.
    assert!(points.len() % points_per_field == 0 && points.len() / points_per_field == 6);

    // These reference points identify a field. They are the top left corners in each field in the
    // original orientation.
    let ref_points = points
        .iter()
        .enumerate()
        .filter_map(|(idx, el)| {
            if idx % points_per_field == 0 {
                Some(el)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // Determine which field is direct neighbour to which other fields. That way, we can determine
    // which field we need to fold.
    let field_neigh = ref_points
        .iter()
        .map(|el| {
            let ppe = points_per_edge as isize;
            let mut neighs = vec![];
            let poss_neighs = vec![
                Point::new(el.x, el.y - ppe),
                Point::new(el.x, el.y + ppe),
                Point::new(el.x - ppe, el.y),
                Point::new(el.x + ppe, el.y),
            ];
            for pn in poss_neighs {
                if let Some((point, _tile)) = occ_map.get_key_value(&pn) {
                    neighs.push(point);
                }
            }
            FieldNeighDat { pos: el, neighs }
        })
        .collect::<Vec<_>>();

    // Build 3d representations of all on a flat surface in the same order as in `points`.
    let mut points_3d = points
        .iter()
        .map(|point| PointData {
            pos: V3::new(point.x, point.y, 0),
            north: V3::new(0, 1, 0),
            norm: V3::new(0, 0, 1),
        })
        .collect::<Vec<_>>();

    println!("{:?}\n", points);
    println!("{:?}\n", ref_points);
    println!("{:?}\n", field_neigh);

    let x_rot = rotmat(V3::new(1, 0, 0), -90)?;
    println!("{:?}\n", x_rot);

    Ok(HashMap::<&Point, PointData>::from_iter(
        points.into_iter().zip(points_3d.into_iter()),
    ))
}
