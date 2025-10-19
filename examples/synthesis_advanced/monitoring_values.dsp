
import("stdfaust.lib");

SR = 48000;
gui = vbargraph("value[style:numerical]", 0, SR);

process = (+(1) % SR)~ _ : gui;