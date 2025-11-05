import("stdfaust.lib");

// We use our homemade sine oscillator:
counter(i, n) = ((_ + i) % n) ~ _;
phasor(f) = counter(f/ma.SR, 1.0);

sine(f) = sin(phasor(f) * 2.0 * ma.PI);

// Use the 'par' primitive, which allows to 
// programtically put signal expressions 
// in parallel:
process = 
    par(
        i, // iteration index variable;
        8, // number of iterations;
        sine(220*(n+1)) // signal expression to duplicate 
    ) 
    :> *(1/16)  // merge & adjust the amplitude 
    <: _,_;     // split to stereo


// OR we could list the frequencies that we want manually:

frequencies = (
    220,
    440,
    550,
    551,
    880,
    1200
);

process = frequencies : par(i, 6, sine);