import("stdfaust.lib");

// Sliders:
oscFreq = hslider("oscFreq",80,50,500,0.01);
lfoFreq = hslider("lfoFreq",1,0.01,8,0.01);
lfoRange = hslider("lfoRange",1000,10,5000,0.01) : si.smoo;
noiseGain = hslider("noiseGain",0,0,1,0.01) <: _*_;
masterVol = hslider("masterVol",0.8,0,1,0.01) <: _*_;
panning = hslider("pan",0.5,0,1,0.01)  : si.smoo;

// Buttons:
activateNoise = button(\"activateNoise\");
killSwitch = 1-button(\"killSwitch\");

LFO = os.lf_triangle(lfoFreq) * 0.5 + 0.5;

process = os.oscrc(440) 
        * 0.25 
        * killSwitch 
        * os.sawtooth(oscFreq) 
        + no.noise * noiseGain
        * activateNoise : fi.resonlp(
            LFO * lfoRange + 50,
            5,
            1)
        * masterVol <: _ * (1-panning), _ * panning;