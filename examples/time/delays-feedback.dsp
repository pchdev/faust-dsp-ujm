// n: delay length (in samples)
// f: feedback coeff. [0, 1]
// x: input signal to be delayed
dfb(n, f, x) = (x + (_ * f)) @ n ~ _;
