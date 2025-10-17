import("stdfaust.lib");

// hslider("name", default, min, max, step)
saw_freq = hslider("saw_freq", 440, 20, 10000, 1);
cutoff = hslider("cutoff", 440, 20, 10000, 1);

saw_filter = os.sawtooth(saw_freq) : fi.lowpass(1, cutoff);

process =  saw_filter <: _,_;