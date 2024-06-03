use serde::{Deserialize, Serialize};

use bindings::{ffdh_f, Bin, FRect, IRect};

mod bindings;

/// Definition for serializing and deserializing Bin.
#[repr(C)]
#[derive(Serialize, Deserialize)]
#[serde(remote = "Bin")]
struct BinDef {
    pub id: usize,
    pub w: i32,
    pub h: i32,
}

/// Wrapper for Bin.
#[repr(transparent)]
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct BinWrapper(#[serde(with = "BinDef")] Bin);

fn default_coordinate_i32() -> i32 {
    -1
}

/// Definition for serializing and deserializing IRect.
#[repr(C)]
#[derive(Serialize, Deserialize)]
#[serde(remote = "IRect")]
struct IRectDef {
    pub id: usize,
    #[serde(default)]
    pub bin_id: usize,
    #[serde(default = "default_coordinate_i32")]
    pub x: i32,
    #[serde(default = "default_coordinate_i32")]
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

/// Wrapper for IRect.
#[repr(transparent)]
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct IRectWrapper(#[serde(with = "IRectDef")] IRect);

fn default_coordinate_f32() -> f32 {
    -1.0
}

/// Definition for serializing and deserializing FRect.
#[repr(C)]
#[derive(Serialize, Deserialize)]
#[serde(remote = "FRect")]
struct FRectDef {
    pub id: usize,
    #[serde(default)]
    pub bin_id: usize,
    #[serde(default = "default_coordinate_f32")]
    pub x: f32,
    #[serde(default = "default_coordinate_f32")]
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

/// Wrapper for FRect.
#[repr(transparent)]
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct FRectWrapper(#[serde(with = "FRectDef")] FRect);

fn ffdh(bin: &mut BinWrapper, rects: &mut [FRectWrapper]) {
    let bin_pointer = bin as *mut BinWrapper as *mut Bin;
    let rects_pointer = rects.as_mut_ptr() as *mut FRect;
    // It's safe to call this function
    // because we know that the pointers are valid and n is the correct length.
    unsafe { ffdh_f(bin_pointer, rects_pointer, rects.len()) }
}

/// Bin packing algorithms.
#[derive(Debug, Deserialize)]
pub enum Algorithm {
    FFDH,
}

impl Algorithm {
    fn run(&self, bin: &mut BinWrapper, rects: &mut [FRectWrapper]) {
        match self {
            Algorithm::FFDH => ffdh(bin, rects),
        }
    }
}

#[tauri::command]
pub fn run_bin_packing(
    mut bin: BinWrapper,
    mut rects: Vec<FRectWrapper>,
    algorithm: Algorithm,
) -> Vec<FRectWrapper> {
    algorithm.run(&mut bin, &mut rects);
    rects
}
