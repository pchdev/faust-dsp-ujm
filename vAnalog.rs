/* ------------------------------------------------------------
name: "virtualAnalog"
Code generated with Faust 2.81.8 (https://faust.grame.fr)
Compilation options: -lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */
#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
pub struct mydsp {
	iVec0: [i32;2],
	fHslider0: F32,
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	fRec0: [F32;2],
	fConst2: F32,
	fConst3: F32,
	fHslider1: F32,
	fRec1: [F32;2],
	fConst4: F32,
	iRec3: [i32;2],
	fHslider2: F32,
	fButton0: F32,
	fHslider3: F32,
	fRec4: [F32;2],
	fConst5: F32,
	fConst6: F32,
	fConst7: F32,
	fRec6: [F32;2],
	fRec7: [F32;2],
	fButton1: F32,
	fRec2: [F32;3],
	fHslider4: F32,
	fRec8: [F32;2],
	fHslider5: F32,
}

pub type FaustFloat = F32;
fn mydsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}
mod ffi {
	use std::os::raw::c_float;
	// Conditionally compile the link attribute only on non-Windows platforms
	#[cfg_attr(not(target_os = "windows"), link(name = "m"))]
	unsafe extern "C" {
		pub fn remainderf(from: c_float, to: c_float) -> c_float;
		pub fn rintf(val: c_float) -> c_float;
	}
}
fn remainder_f32(from: f32, to: f32) -> f32 {
	unsafe { ffi::remainderf(from, to) }
}
fn rint_f32(val: f32) -> f32 {
	unsafe { ffi::rintf(val) }
}

pub const FAUST_INPUTS: usize = 0;
pub const FAUST_OUTPUTS: usize = 2;
pub const FAUST_ACTIVES: usize = 8;
pub const FAUST_PASSIVES: usize = 0;


impl mydsp {
		
	pub fn new() -> mydsp { 
		mydsp {
			iVec0: [0;2],
			fHslider0: 0.0,
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fRec0: [0.0;2],
			fConst2: 0.0,
			fConst3: 0.0,
			fHslider1: 0.0,
			fRec1: [0.0;2],
			fConst4: 0.0,
			iRec3: [0;2],
			fHslider2: 0.0,
			fButton0: 0.0,
			fHslider3: 0.0,
			fRec4: [0.0;2],
			fConst5: 0.0,
			fConst6: 0.0,
			fConst7: 0.0,
			fRec6: [0.0;2],
			fRec7: [0.0;2],
			fButton1: 0.0,
			fRec2: [0.0;3],
			fHslider4: 0.0,
			fRec8: [0.0;2],
			fHslider5: 0.0,
		}
	}
	pub fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("compile_options", r"-lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m.declare("filename", r"virtualAnalog.dsp");
		m.declare("filters.lib/fir:author", r"Julius O. Smith III");
		m.declare("filters.lib/fir:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/fir:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/iir:author", r"Julius O. Smith III");
		m.declare("filters.lib/iir:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/iir:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/lowpass0_highpass1", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/name", r"Faust Filters Library");
		m.declare("filters.lib/nlf2:author", r"Julius O. Smith III");
		m.declare("filters.lib/nlf2:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/nlf2:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/resonlp:author", r"Julius O. Smith III");
		m.declare("filters.lib/resonlp:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/resonlp:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/tf2:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf2:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf2:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/tf2s:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf2s:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf2s:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/version", r"1.7.1");
		m.declare("maths.lib/author", r"GRAME");
		m.declare("maths.lib/copyright", r"GRAME");
		m.declare("maths.lib/license", r"LGPL with exception");
		m.declare("maths.lib/name", r"Faust Math Library");
		m.declare("maths.lib/version", r"2.9.0");
		m.declare("name", r"virtualAnalog");
		m.declare("noises.lib/name", r"Faust Noise Generator Library");
		m.declare("noises.lib/version", r"1.5.0");
		m.declare("oscillators.lib/lf_sawpos:author", r"Bart Brouns, revised by Stéphane Letz");
		m.declare("oscillators.lib/lf_sawpos:licence", r"STK-4.3");
		m.declare("oscillators.lib/lf_triangle:author", r"Bart Brouns");
		m.declare("oscillators.lib/lf_triangle:licence", r"STK-4.3");
		m.declare("oscillators.lib/name", r"Faust Oscillator Library");
		m.declare("oscillators.lib/saw1:author", r"Bart Brouns");
		m.declare("oscillators.lib/saw1:licence", r"STK-4.3");
		m.declare("oscillators.lib/saw2ptr:author", r"Julius O. Smith III");
		m.declare("oscillators.lib/saw2ptr:license", r"STK-4.3");
		m.declare("oscillators.lib/version", r"1.6.0");
		m.declare("platform.lib/name", r"Generic Platform Library");
		m.declare("platform.lib/version", r"1.3.0");
		m.declare("signals.lib/name", r"Faust Signal Routing Library");
		m.declare("signals.lib/version", r"1.6.0");
	}

	pub fn get_sample_rate(&self) -> i32 { self.fSampleRate as i32}
	
	pub fn class_init(sample_rate: i32) {
		// Obtaining locks on 0 static var(s)
	}
	pub fn instance_reset_params(&mut self) {
		self.fHslider0 = 1.0;
		self.fHslider1 = 1e+03;
		self.fHslider2 = 0.0;
		self.fButton0 = 0.0;
		self.fHslider3 = 8e+01;
		self.fButton1 = 0.0;
		self.fHslider4 = 0.5;
		self.fHslider5 = 0.8;
	}
	pub fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.iVec0[l0 as usize] = 0;
		}
		for l1 in 0..2 {
			self.fRec0[l1 as usize] = 0.0;
		}
		for l2 in 0..2 {
			self.fRec1[l2 as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.iRec3[l3 as usize] = 0;
		}
		for l4 in 0..2 {
			self.fRec4[l4 as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fRec6[l5 as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec7[l6 as usize] = 0.0;
		}
		for l7 in 0..3 {
			self.fRec2[l7 as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fRec8[l8 as usize] = 0.0;
		}
	}
	pub fn instance_constants(&mut self, sample_rate: i32) {
		// Obtaining locks on 0 static var(s)
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = 1.0 / self.fConst0;
		self.fConst2 = 44.1 / self.fConst0;
		self.fConst3 = 1.0 - self.fConst2;
		self.fConst4 = 3.1415927 / self.fConst0;
		self.fConst5 = 2764.6016 / self.fConst0;
		self.fConst6 = F32::cos(self.fConst5);
		self.fConst7 = F32::sin(self.fConst5);
	}
	pub fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	pub fn init(&mut self, sample_rate: i32) {
		mydsp::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	pub fn build_user_interface(&self, ui_interface: &mut dyn UI<FaustFloat>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	pub fn build_user_interface_static(ui_interface: &mut dyn UI<FaustFloat>) {
		ui_interface.open_vertical_box("virtualAnalog");
		ui_interface.add_button("activateNoise", ParamIndex(0));
		ui_interface.add_button("killSwitch", ParamIndex(1));
		ui_interface.add_horizontal_slider("lfoFreq", ParamIndex(2), 1.0, 0.01, 8.0, 0.01);
		ui_interface.add_horizontal_slider("lfoRange", ParamIndex(3), 1e+03, 1e+01, 5e+03, 0.01);
		ui_interface.add_horizontal_slider("masterVol", ParamIndex(4), 0.8, 0.0, 1.0, 0.01);
		ui_interface.add_horizontal_slider("noiseGain", ParamIndex(5), 0.0, 0.0, 1.0, 0.01);
		ui_interface.add_horizontal_slider("oscFreq", ParamIndex(6), 8e+01, 5e+01, 5e+02, 0.01);
		ui_interface.add_horizontal_slider("pan", ParamIndex(7), 0.5, 0.0, 1.0, 0.01);
		ui_interface.close_box();
	}
	
	pub fn get_param(&self, param: ParamIndex) -> Option<FaustFloat> {
		match param.0 {
			0 => Some(self.fButton0),
			1 => Some(self.fButton1),
			2 => Some(self.fHslider0),
			3 => Some(self.fHslider1),
			5 => Some(self.fHslider2),
			6 => Some(self.fHslider3),
			7 => Some(self.fHslider4),
			4 => Some(self.fHslider5),
			_ => None,
		}
	}
	
	pub fn set_param(&mut self, param: ParamIndex, value: FaustFloat) {
		match param.0 {
			0 => { self.fButton0 = value }
			1 => { self.fButton1 = value }
			2 => { self.fHslider0 = value }
			3 => { self.fHslider1 = value }
			5 => { self.fHslider2 = value }
			6 => { self.fHslider3 = value }
			7 => { self.fHslider4 = value }
			4 => { self.fHslider5 = value }
			_ => {}
		}
	}
	
	pub fn compute(
		&mut self,
		count: usize,
		inputs: &[impl AsRef<[FaustFloat]>],
		outputs: &mut[impl AsMut<[FaustFloat]>],
	) {
		
		// Obtaining locks on 0 static var(s)
		let [outputs0, outputs1, .. ] = outputs.as_mut() else { panic!("wrong number of output buffers"); };
		let outputs0 = outputs0.as_mut()[..count].iter_mut();
		let outputs1 = outputs1.as_mut()[..count].iter_mut();
		let mut fSlow0: F32 = self.fConst1 * self.fHslider0;
		let mut fSlow1: F32 = self.fConst2 * self.fHslider1;
		let mut fSlow2: F32 = 4.656613e-10 * self.fButton0 * mydsp_faustpower2_f(self.fHslider2);
		let mut fSlow3: F32 = F32::max(1.1920929e-07, F32::abs(self.fHslider3));
		let mut fSlow4: F32 = self.fConst1 * fSlow3;
		let mut fSlow5: F32 = 1.0 - self.fConst0 / fSlow3;
		let mut fSlow6: F32 = 0.25 * (1.0 - self.fButton1);
		let mut fSlow7: F32 = self.fConst2 * self.fHslider4;
		let mut fSlow8: F32 = mydsp_faustpower2_f(self.fHslider5);
		let zipped_iterators = outputs0.zip(outputs1);
		for (output0, output1) in zipped_iterators {
			self.iVec0[0] = 1;
			let mut iTemp0: i32 = i32::wrapping_sub(1, self.iVec0[1]);
			let mut fTemp1: F32 = (if iTemp0 != 0 {0.0} else {fSlow0 + self.fRec0[1]});
			self.fRec0[0] = fTemp1 - F32::floor(fTemp1);
			self.fRec1[0] = fSlow1 + self.fConst3 * self.fRec1[1];
			let mut fTemp2: F32 = F32::tan(self.fConst4 * (0.5 * self.fRec1[0] * (2.0 * (1.0 - F32::abs(2.0 * self.fRec0[0] + -1.0)) + -1.0 + 1.0) + 5e+01));
			let mut fTemp3: F32 = 1.0 / fTemp2;
			let mut fTemp4: F32 = (fTemp3 + 0.2) / fTemp2 + 1.0;
			self.iRec3[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec3[1]), 12345);
			let mut fTemp5: F32 = fSlow4 + self.fRec4[1] + -1.0;
			let mut iTemp6: i32 = (fTemp5 < 0.0) as i32;
			let mut fTemp7: F32 = fSlow4 + self.fRec4[1];
			self.fRec4[0] = (if iTemp6 != 0 {fTemp7} else {fTemp5});
			let mut fRec5: F32 = (if iTemp6 != 0 {fTemp7} else {fSlow4 + self.fRec4[1] + fSlow5 * fTemp5});
			self.fRec6[0] = self.fConst7 * self.fRec7[1] + self.fConst6 * self.fRec6[1];
			self.fRec7[0] = (iTemp0) as F32 + self.fConst6 * self.fRec7[1] - self.fConst7 * self.fRec6[1];
			self.fRec2[0] = fSlow6 * self.fRec7[0] * (2.0 * fRec5 + -1.0) + fSlow2 * (self.iRec3[0]) as F32 - (self.fRec2[2] * ((fTemp3 + -0.2) / fTemp2 + 1.0) + 2.0 * self.fRec2[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp2))) / fTemp4;
			let mut fTemp8: F32 = self.fRec2[2] + self.fRec2[0] + 2.0 * self.fRec2[1];
			self.fRec8[0] = fSlow7 + self.fConst3 * self.fRec8[1];
			*output0 = fSlow8 * ((1.0 - self.fRec8[0]) * fTemp8 / fTemp4);
			*output1 = fSlow8 * (self.fRec8[0] * fTemp8 / fTemp4);
			self.iVec0[1] = self.iVec0[0];
			self.fRec0[1] = self.fRec0[0];
			self.fRec1[1] = self.fRec1[0];
			self.iRec3[1] = self.iRec3[0];
			self.fRec4[1] = self.fRec4[0];
			self.fRec6[1] = self.fRec6[0];
			self.fRec7[1] = self.fRec7[0];
			self.fRec2[2] = self.fRec2[1];
			self.fRec2[1] = self.fRec2[0];
			self.fRec8[1] = self.fRec8[0];
		}
		
	}

}

impl FaustDsp for mydsp {
	type T = FaustFloat;
	fn new() -> Self where Self: Sized {
		Self::new()
	}
	fn metadata(&self, m: &mut dyn Meta) {
		self.metadata(m)
	}
	fn get_sample_rate(&self) -> i32 {
		self.get_sample_rate()
	}
	fn get_num_inputs(&self) -> i32 {
		FAUST_INPUTS as i32
	}
	fn get_num_outputs(&self) -> i32 {
		FAUST_OUTPUTS as i32
	}
	fn class_init(sample_rate: i32) where Self: Sized {
		Self::class_init(sample_rate);
	}
	fn instance_reset_params(&mut self) {
		self.instance_reset_params()
	}
	fn instance_clear(&mut self) {
		self.instance_clear()
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate)
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_init(sample_rate)
	}
	fn init(&mut self, sample_rate: i32) {
		self.init(sample_rate)
	}
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		self.build_user_interface(ui_interface)
	}
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) where Self: Sized {
		Self::build_user_interface_static(ui_interface);
	}
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		self.get_param(param)
	}
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		self.set_param(param, value)
	}
	fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut [&mut [Self::T]]) {
		self.compute(count as usize, inputs, outputs)
	}
}
