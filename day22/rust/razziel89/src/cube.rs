use nalgebra::{Matrix3, Vector3};

use crate::data::{Point, Tile};
use anyhow::{Context, Error, Result};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

const PI: f64 = std::f64::consts::PI;

pub type V3 = Vector3<isize>;
pub type M3 = Matrix3<isize>;

#[derive(Debug)]
pub struct PointData {
    pub pos: V3,
    pub north: V3,
    pub norm: V3,
}

#[derive(Debug)]
struct FieldNeighDat<'a> {
    pos: &'a Point,
    neighs: Vec<&'a Point>,
    field: usize,
}

fn point_to_field_id(point: &Point, max: &Point, points_per_edge: &usize) -> usize {
    let ppe = *points_per_edge as isize;

    let max_x_field = max.x / ppe;

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

fn transrot(input: &PointData, ref_point: &V3, mat: &M3, trans: &V3) -> PointData {
    PointData {
        pos: (mat * (input.pos - ref_point)) + ref_point + trans,
        north: mat * input.north,
        norm: mat * input.norm,
    }
}

fn affected_fields(
    ref_point: &Point,
    neigh_data: &Vec<FieldNeighDat>,
    ppe: &usize,
    right_instead_of_down: bool,
) -> Result<Option<HashSet<usize>>> {
    let start_neigh_point = if right_instead_of_down {
        Point::new(ref_point.x + *ppe as isize, ref_point.y)
    } else {
        Point::new(ref_point.x, ref_point.y + *ppe as isize)
    };
    let maybe_start_neigh_data = neigh_data.iter().find(|el| *el.pos == start_neigh_point);
    // There are no affected fields because there is no such neighbouring field.
    if maybe_start_neigh_data.is_none() {
        return Ok(None);
    }
    let start_neigh_data = maybe_start_neigh_data.unwrap();

    let mut affected = start_neigh_data
        .neighs
        .iter()
        .filter(|&&el| el != ref_point)
        .collect::<HashSet<_>>();
    affected.insert(&start_neigh_data.pos);
    let mut num_affected = 0;

    while num_affected != affected.len() {
        // Assume we found all affected fields.
        num_affected = affected.len();

        let new_affected = affected
            .iter()
            .flat_map(|&&el| neigh_data.iter().find(|nd| nd.pos == el))
            .map(|el| &el.neighs)
            .flatten()
            .filter(|&&el| el != ref_point)
            .collect::<HashSet<_>>();

        affected = &affected | &new_affected;
    }
    let num_affected = affected.len();
    let affected_idx = affected
        .into_iter()
        .flat_map(|el| {
            neigh_data
                .iter()
                .find(|nd| &nd.pos == el)
                .map(|el| el.field)
        })
        .collect::<HashSet<_>>();
    assert_eq!(num_affected, affected_idx.len());

    Ok(Some(affected_idx))
}

fn as_cartesian_unit(vec: &V3) -> Result<V3> {
    if vec
        .iter()
        .flat_map(|el| if el != &0 { Some(()) } else { None })
        .count()
        == 1
    {
        Ok(V3::from_iterator(vec.iter().map(|el| {
            if el != &0 {
                el / el.abs()
            } else {
                0
            }
        })))
    } else {
        Err(Error::msg(format!("cannot be unit: {}", vec)))
    }
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
                Some(*el)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // Determine which field is direct neighbour to which other fields. That way, we can determine
    // which field we need to fold.
    let mut has_err = false;
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
                if let Some((point, tile)) = occ_map.get_key_value(&pn) {
                    if tile != &Tile::None {
                        neighs.push(point);
                    }
                }
            }
            let point_idx = points.iter().position(|p| p == el);
            let field = if let Some(idx) = point_idx {
                idx / points_per_field
            } else {
                has_err = true;
                0
            };
            FieldNeighDat {
                pos: *el,
                neighs,
                field,
            }
        })
        .collect::<Vec<_>>();
    // Ensure all field indices could be determined.
    assert!(!has_err);
    // Sanity check neighbour data.
    assert_eq!(field_neigh.len(), 6);
    assert_eq!(
        field_neigh
            .iter()
            .map(|el| &el.neighs)
            .flatten()
            .collect::<HashSet<_>>(),
        field_neigh.iter().map(|el| &el.pos).collect::<HashSet<_>>()
    );

    // Build 3d representations of all on a flat surface in the same order as in `points`.
    let mut points_3d = points
        .iter()
        .map(|point| PointData {
            pos: V3::new(point.x, point.y, 0),
            north: V3::new(0, 1, 0),
            norm: V3::new(0, 0, 1),
        })
        .collect::<Vec<_>>();

    // For one field after the next, fold first all fields connected to the right neighbour and
    // then all fields connected to the down member. We always fold downwards from the point of
    // view of the field we are looking at.
    for idx in 0..field_neigh.len() {
        let field = &field_neigh[idx];
        assert_eq!(idx, field.field);

        for is_right in [true, false] {
            let aff = affected_fields(&ref_points[idx], &field_neigh, &points_per_edge, is_right)?;
            if aff.is_none() {
                // If there are no affected fields, we don't have to fold anything.
                continue;
            }
            let affected_field_idx = aff.unwrap();

            let ref_point_2d = if is_right {
                Point::new(field.pos.x + points_per_edge as isize, field.pos.y)
            } else {
                Point::new(field.pos.x, field.pos.y + points_per_edge as isize)
            };

            let mat;
            let trans;
            let ref_point;

            {
                // Convert coordinates to 3d.
                let ref_point_3d = &points_3d[points
                    .iter()
                    .position(|&el| el == &ref_point_2d)
                    .ok_or(Error::msg("cannot find ref 2d point"))?];
                let field_point_3d = &points_3d[points
                    .iter()
                    .position(|&el| el == field.pos)
                    .ok_or(Error::msg("cannot find field 2d point"))?];

                // Build transformation.
                let diff = ref_point_3d.pos - field_point_3d.pos;
                let rot_ax =
                    as_cartesian_unit(&field_point_3d.norm.cross(&diff)).context("axis")?;
                let ang = 90;
                trans = as_cartesian_unit(&-diff).with_context(|| {
                    format!(
                        "translation from {} to {}",
                        field_point_3d.pos, ref_point_3d.pos
                    )
                })?;
                mat = rotmat(rot_ax, ang)?;
                ref_point = ref_point_3d.pos.clone();
            }

            // Transform those points that lie within affected fields by applying a transformation
            // function.
            let trans_fn = |el: &PointData| transrot(el, &ref_point, &mat, &trans);
            points_3d = points_3d
                .into_iter()
                .enumerate()
                .map(|(idx, el)| {
                    let field_idx = idx / points_per_field;
                    if affected_field_idx.contains(&field_idx) {
                        trans_fn(&el)
                    } else {
                        el
                    }
                })
                .collect::<Vec<_>>();
        }
    }

    Ok(HashMap::<&Point, PointData>::from_iter(
        points.into_iter().zip(points_3d.into_iter()),
    ))
}
