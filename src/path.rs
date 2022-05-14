use quaternions::{qi, Quaternion};

pub fn compute_cube_vertices() -> Vec<f32> {
  let geo: Vec<[f32; 3]> = vec![
    [-0.5, -0.5, 0.0],
    [-0.5, 0.5, 0.0],
    [0.5, 0.5, 0.0],
    [0.5, -0.5, 0.0],
    [-0.5, -0.5, -1.0],
    [-0.5, 0.5, -1.0],
    [0.5, 0.5, -1.0],
    [0.5, -0.5, -1.0],
  ];

  let indices = vec![0, 1, 1, 2, 2, 3, 3, 0, 0, 4, 1, 5, 2, 6, 3, 7, 4, 5, 5, 6, 6, 7, 7, 4];
  let mut points: Vec<[f32; 3]> = Vec::new();
  for i in 0..indices.len() {
    points.push(geo[indices[i]]);
  }

  let moved_points: Vec<_> = points.iter().map(|p| [p[0] * 400., p[1] * 400., p[2] * 400. - 1200.]).collect();
  let mut vertices: Vec<f32> = Vec::new();
  for p in moved_points {
    vertices.extend_from_slice(&p);
  }
  vertices
}

type Q32 = Quaternion<f32>;

pub fn compute_lamp_tree_vertices() -> Vec<f32> {
  let points = fold_line4(
    14,
    Quaternion::<f32>::default(),
    qi(0, 0, 100, 0),
    (qi(22, 0, 20, 0), qi(23, 16, 20, 0), qi(27, 16, 20, 0), qi(28, 0, 20, 0)),
    qi(50, 0, 0, 0).inverse(),
    0.16,
  );
  let mut vertices = Vec::new();
  for p in points {
    vertices.push(p.x);
    vertices.push(p.y);
    vertices.push(p.z);
  }
  vertices
}

pub fn fold_line4(level: u32, base: Q32, v: Q32, q4: (Q32, Q32, Q32, Q32), full_reversed: Q32, minimal_seg: f32) -> Vec<Q32> {
  let (a, b, c, d) = q4;
  let next_v = v * full_reversed;
  let branch_a = next_v * a;
  let branch_b = next_v * b;
  let branch_c = next_v * c;
  let branch_d = next_v * d;
  if level == 0 || v.square_length() < minimal_seg {
    vec![base + branch_a, base + branch_b, base + branch_c, base + branch_d, base + v]
  } else {
    let mut ret = vec![];
    ret.extend(fold_line4(level - 1, base, branch_a, q4, full_reversed, minimal_seg));
    ret.extend(fold_line4(
      level - 1,
      base + branch_a,
      branch_b - branch_a,
      q4,
      full_reversed,
      minimal_seg,
    ));
    ret.extend(fold_line4(
      level - 1,
      base + branch_b,
      branch_c - branch_b,
      q4,
      full_reversed,
      minimal_seg,
    ));
    ret.extend(fold_line4(
      level - 1,
      base + branch_c,
      branch_d - branch_c,
      q4,
      full_reversed,
      minimal_seg,
    ));
    ret.extend(fold_line4(level - 1, base + branch_d, v - branch_d, q4, full_reversed, minimal_seg));
    ret
  }
}
