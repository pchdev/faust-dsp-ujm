import("stdfaust.lib");

counter(i, n) = ((_ + i) % n) ~ _;

// We define a maximum length for our buffer:
BUFFER_MAX = 48000 * 10;

// r: read position in samples.
// w: write position in samples.
// x: input signal to write.
buffer(r, w, x) = rwtable(BUFFER_MAX, 0, w, x, r);

// Define our read/write cursors:
record = counter(button("record"), BUFFER_MAX);
play = counter(button("play"), BUFFER_MAX);

// And we have now a simple looper:
process = buffer(play, record, _);

