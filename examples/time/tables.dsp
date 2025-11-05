import("stdfaust.lib");

counter(i, n) = ((_ + i) % n) ~ _;

// We define a maximum length for our buffer:
BUFFER_MAX = ma.SR * 10;

// r: read position in samples.
// w: write position in samples.
// x: input signal to write.
buffer(r, w, x) = rwtable(BUFFER_MAX, 0, w, x, r);

// Define our read/write cursors:
record = counter(button("record"), BUFFER_MAX);
play = counter(button("play"), BUFFER_MAX)

process = buffer(play, record);

