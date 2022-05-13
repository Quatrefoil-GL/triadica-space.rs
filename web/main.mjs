import init, { initApp } from "../pkg/triadica_space";

window.onload = () => {
  init().then(() => {
    initApp();
  });
};
