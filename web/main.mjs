import init, { initApp, onWindowResize } from "../pkg/triadica_space";

window.onload = () => {
  init().then(() => {
    initApp();
  });
};

window.addEventListener("resize", (event) => {
  onWindowResize();
});
