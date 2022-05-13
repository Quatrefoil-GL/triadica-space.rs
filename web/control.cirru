
ns control.core
  :require
    touch-control.core :refer $ render-control! start-control-loop!
    |../pkg/triadica_space :refer $ onControl

defn main! ()
  render-control!
  start-control-loop! 10 $ fn (elapsed states delta)
    let
        resetting? $ and (:left-b? states) (:right-b? states)
      when
        or (not= zero (:left-move states)) (not= zero (:right-move delta)) resetting?
        let-sugar
            ([] lx ly) (:left-move states)
            ([] rx ry) (:right-move states)
            ([] rdx rdy) (:right-move delta)
          onControl elapsed lx ly rx ry rdx rdy (:left-a? states) resetting?

def zero $ [] 0 0
