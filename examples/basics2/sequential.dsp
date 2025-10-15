import("stdfaust.lib");

process = os.sawtooth(440) : fi.lowpass(1, 440);