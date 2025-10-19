import("stdfaust.lib");

SR = 48000;
gui = vbargraph("value[style:numerical]", 0, SR);

// Outputs the value '1' every second:
counter1s = ((+(1) % SR) ~ _) == 0;

// We increment from the trigger:
counterwf(trigger) = (+(trigger) % 4)~_;

process =  counterwf(counter1s) : gui;