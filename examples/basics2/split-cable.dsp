import("stdfaust.lib");

saw_filter = os.sawtooth(440) : fi.lowpass(1, 440);

process =  saw_filter <: _,_;