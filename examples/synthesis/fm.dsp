import("stdfaust.lib");

// We modulate the frequency of an oscillator by another:
minimal_fm(f,m) = os.osci(
    f + f * os.osci(m)
);

process = minimal_fm(440, 20);