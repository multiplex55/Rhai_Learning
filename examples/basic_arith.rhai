let sum = 0;
for n in 1..=10 {
    sum += n;
}
if sum > 50 {
    print(`sum is ${sum}`);
} else {
    print("sum too small");
}
sum
