import("stdfaust.lib");

osc = (os.oscrc(440), os.oscrc(880)) :> _;

process =  osc * 0.25 <: _,_;