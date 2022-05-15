import init, { initApp, onWindowResize } from "../pkg/triadica_space";
import { main_$x_ } from "../js-out/control.core.mjs";

window.onload = () => {
  init().then(() => {
    initApp();
    main_$x_();
    window.addEventListener("resize", (event) => {
      onWindowResize();
    });
  });
};
