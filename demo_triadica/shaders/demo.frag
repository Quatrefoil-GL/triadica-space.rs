precision mediump float;

varying float v_s;
varying float v_r;

void main() {
  if (v_r >= 0.0) {
    // gl_FragColor = vec4(1.0, 1.0, 8.0, 1.0);
    float factor = smoothstep(0.0, 0.4, 1.0 - v_r/10.0);
    gl_FragColor = vec4(0.6 + factor, 0.6 + factor, 1.0 - factor, 1.);
  } else if (v_r > -v_s) {
    gl_FragColor = vec4(0.6, 0.6, 1.0, 1.0);
  } else {
    // supposed to be hidden with depth test
    gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
  }

  // float vv = 1.0/z;
  // gl_FragColor = vec4(vv, vv, vv, 1.0);

  // gl_FragColor = vec4(1., 1., 1., 1.);

}
