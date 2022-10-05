import init, {
  initApp,
  onWindowResize,
  onControl,
} from "../pkg/triadica_space";
import { renderControl, startControlLoop } from "@triadica/touch-control";

let isZero = (point: [number, number]): Boolean => {
  return point[0] === 0 && point[1] === 0;
};

init().then(() => {
  initApp();
  renderControl();

  startControlLoop(10, (elapsed, states, delta) => {
    let resetting = states.leftB && states.rightB;
    if (!isZero(states.leftMove) || !isZero(states.rightMove) || resetting) {
      onControl(
        elapsed,
        states.leftMove[0],
        states.leftMove[1],
        states.rightMove[0],
        states.rightMove[1],
        delta.rightMove[0],
        delta.rightMove[1],
        states.leftA,
        resetting
      );
    }
  });

  window.addEventListener("resize", (event) => {
    onWindowResize();
  });
});