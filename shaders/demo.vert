
uniform float lookDistance;
uniform vec3 forward;
uniform vec3 upward;
uniform vec3 rightward;

uniform float coneBackScale;
uniform float viewportRatio;

uniform vec3 cameraPosition;

attribute vec3 a_position;

varying float v_r;
varying float v_s;

float square(float a) {
  return a * a;
}

float sumSquares2(float a, float b) {
  return a * a + b * b;
}

float sumSquares3(float a, float b, float c) {
  return a * a + b * b + c * c;
}

struct PointResult {
  vec3 point;
  float r;
  float s;
};

PointResult transform_perspective(vec3 p) {
  vec3 moved_point = p - cameraPosition;

  float s = coneBackScale;

  float x = moved_point.x;
  float y = moved_point.y;
  float z = moved_point.z;

  float r = dot(moved_point, forward) / lookDistance;

  if (r < (s * -0.9)) {
    // make it disappear with depth test since it's probably behind the camera
    return PointResult(vec3(0.0, 0.0, 10000.), r, s);
  }

  float screen_scale = (s + 1.0) / (r + s);
  float y_next = dot(moved_point, upward) * screen_scale;
  float x_next = - dot(moved_point, -rightward) * screen_scale;
  float z_next = r;

  return PointResult(
    vec3(x_next, y_next / viewportRatio, z_next),
    r, s
  );
}

void main() {
  PointResult result = transform_perspective(a_position);
  vec3 pos_next = result.point;

  v_s = result.s;
  v_r = result.r;
  float zoom = 0.0004;
  // if (result.r > 0.0) {
    gl_Position = vec4(pos_next.x * zoom, pos_next.y * zoom, pos_next.z * zoom, 1.0);
  // } else {
  //   gl_Position = vec4(0.0, -1000.0, 0.0, 0.0);
  //   // gl_Position = vec4(a_position.x/1000.0, a_position.y/1000., 0.1, 1.0);
  // }
}