import("stdfaust.lib");

// We use our homemade sine oscillator:
counter(i, n) = ((_ + i) % n) ~ _;
phasor(f) = counter(f/ma.SR, 1.0);

sine(f) = sin(phasor(f) * 2.0 * ma.PI);

// Use the 'par' primitive, which allows to 
// programatically put signal expressions 
// in parallel.
sine8 = par(n, 8, sine(220*(n+1)));
process = sine8 :> _ * 1/16 <: _,_

// OR we could list the frequencies that we want manually:
frequencies = (
    220,
    440,
    550,
    551,
    880,
    1200
);
// And connect to a parallel group of sine oscillators:
process = frequencies : par(i, 6, sine);