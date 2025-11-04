import("stdfaust.lib");

counter(i, n) = ((_ + i) % n) ~ _;
phasor(f) = counter(f/ma.SR, 1.0);

sawtooth(f) = phasor(f) * 2.0 - 1.0;
square(f) = ba.if(phasor(f) > 0.5, 1.0, -1.0);
sine(f) = sin(phasor(f) * 2.0 * ma.PI);
triangle(f) = (ba.if(phasor(f) < 0.5, phasor(f), 1.0-phasor(f)) - 0.25) * 4.0;

process = phasor(440) * 0.25;



