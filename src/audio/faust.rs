
#![allow(non_camel_case_types)]

use std::{error::Error, ffi::{c_int, c_void}, path::Path};
use libloading::{Library, Symbol};

macro_rules! declfn {
    ($name:ident, $fn:ty) => {
        type $name = $fn;
    };
}

declfn!(dspnew, 
    fn() -> *mut c_void
);

declfn!(dspfree, 
    fn(dsp: *mut c_void)
);

declfn!(dspnuminputs, 
    fn(dsp: *mut c_void) -> c_int
);

declfn!(dspnumoutputs, 
    fn(dsp: *mut c_void) -> c_int
);

declfn!(dspinit, 
    fn(dsp: *mut c_void, sr: c_int
));

declfn!(dspproc, fn(
        dsp: *mut c_void, 
        nframes: c_int, 
        inputs: *const *const f32, 
        outputs: *mut *mut f32
    )
);

#[derive(Default)]
pub struct FaustIo {
     pub inputs: Vec<Vec<f32>>,
    pub outputs: Vec<Vec<f32>>,
}

pub struct FaustDsp {
     lib: Box<Library>,
     hdl: *mut c_void,
     new: Symbol<'static, dspnew>,
    free: Symbol<'static, dspfree>,
    init: Symbol<'static, dspinit>,
    proc: Symbol<'static, dspproc>,
     n_inputs: Symbol<'static, dspnuminputs>,
    n_outputs: Symbol<'static, dspnumoutputs>,
      io: FaustIo,
}

impl FaustDsp {
    pub fn load<P: AsRef<Path>>(path: P) 
        -> Result<Self, Box<dyn Error>> {
        unsafe {
            let lib_boxed = Box::new(
                Library::new(path.as_ref())?
            );
            let lib_static: &'static Library = Box::leak(
                Box::new(Library::new(path.as_ref())?)
            );
            let new: Symbol<dspnew> 
                = lib_static.get(b"newmydsp")?
            ;       
            let free: Symbol<dspfree> 
                = lib_static.get(b"deletemydsp")?
            ;
            let init: Symbol<dspinit> 
                = lib_static.get(b"initmydsp")?
            ;
            let proc: Symbol<dspproc> 
                = lib_static.get(b"computemydsp")?
            ;
            let i: Symbol<dspnuminputs> 
                = lib_static.get(b"getNumInputsmydsp")?
            ;
            let o: Symbol<dspnumoutputs> 
                = lib_static.get(b"getNumOutputsmydsp")?
            ;
            let hdl = new();
            Ok(FaustDsp {
                lib: lib_boxed, hdl,
                new, free, init, proc,
                n_inputs: i,
                n_outputs: o,
                io: FaustIo::default(),
            })
        }
    }
    pub fn init(mut self, sr: i32, nframes: usize) -> Self {
        (self.init)(self.hdl, sr);
        self.io.inputs  = vec![
            vec![1.0; nframes]; 
            self.num_inputs()
        ];
        self.io.outputs = vec![
            vec![0.0; nframes]; 
            self.num_outputs()
        ];
        return self;
    }
    pub fn num_inputs(&self) -> usize {
        (self.n_inputs)(self.hdl) as usize
    }
    pub fn num_outputs(&self) -> usize {
        (self.n_outputs)(self.hdl) as usize
    }
    pub fn compute(&mut self) {
        let nframes = self.io.inputs[0].len() as c_int;
        let input_ptrs: Vec<*const f32> = 
            self.io.inputs
                .iter()
                .map(|ch| ch.as_ptr())
                .collect()
        ;
        let mut output_ptrs: Vec<*mut f32> =
            self.io.outputs
                .iter_mut()
                .map(|ch| ch.as_mut_ptr())
                .collect()
        ;
        (self.proc)(
            self.hdl,
            nframes,
            input_ptrs.as_ptr(),
            output_ptrs.as_mut_ptr()
        );
    }
}

impl Drop for FaustDsp {
    fn drop(&mut self) {
        (self.free)(self.hdl);
    }
}

fn test() {
    let mut dsp = FaustDsp::load("./bypass.so")
        .unwrap()
        .init(48000, 16)
    ;
    dsp.compute();
    println!("{:?}", dsp.io.outputs);
}
