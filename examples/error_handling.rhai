fn divide(x, y) {
    if y == 0 {
        throw "division by zero";
    }
    x / y
}

let message = "";
let value = 0;
try {
    value = divide(10, 0);
} catch (err) {
    message = err;
    value = -1;
}
print("Caught: " + message);

#{ msg: message, value: value }
