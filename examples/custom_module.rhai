import "math_utils.rhai" as math;

let n = 4;
let sq = math::square(n);
print("square(" + n.to_string() + ") = " + sq.to_string());
sq
