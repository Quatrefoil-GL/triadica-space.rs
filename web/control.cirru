
ns control.core
  :require
    touch-control.core :refer $ render-control! start-control-loop!

defn main! ()
  render-control!
  start-control-loop! 200 $ fn (elapsed states delta)
    if
      or (not= zero (:left-move states)) (not= zero (:right-move delta))
      println "|has change" states delta

def zero $ [] 0 0
