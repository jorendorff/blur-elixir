#[macro_use] extern crate rustler;
#[macro_use] extern crate lazy_static;
extern crate image;

use std::error::Error;
use rustler::{NifEnv, NifTerm, NifResult, NifEncoder};
use rustler::resource::ResourceCell;
use image::{DynamicImage, GenericImage};

mod atoms {
    rustler_atoms! {
        atom ok;
        atom error;
    }
}

rustler_export_nifs! {
    "Elixir.Blur",
    [
        ("open", 1, open),
        ("save", 2, save),
        ("blur", 1, blur)
    ],
    Some(load)
}

struct Image(DynamicImage);

fn load<'a>(env: NifEnv<'a>, _load_data: NifTerm<'a>) -> bool {
    resource_struct_init!(Image, env);
    true
}

fn fail<'a>(env: NifEnv<'a>, error: &Error) -> NifResult<NifTerm<'a>> {
    Ok((atoms::error(), error.description().encode(env)).encode(env))
}

// open/1 (filename) -> {:ok, image}
fn open<'a>(env: NifEnv<'a>, args: &Vec<NifTerm<'a>>) -> NifResult<NifTerm<'a>> {
    let filename: String = args[0].decode()?;
    match image::open(filename) {
        Ok(dynamic_image) =>
            Ok(ResourceCell::new(Image(dynamic_image)).encode(env)),
        Err(image_err) =>
            fail(env, &image_err)
    }
}

// save/2 (image, filename) -> :ok
fn save<'a>(env: NifEnv<'a>, args: &Vec<NifTerm<'a>>) -> NifResult<NifTerm<'a>> {
    let image_arg: ResourceCell<Image> = args[0].decode()?;
    let filename: String = args[1].decode()?;

    let img = &image_arg.0;
    match image::save_buffer(filename, &img.raw_pixels(), img.width(), img.height(), img.color()) {
        Ok(()) =>
            Ok(atoms::ok().encode(env)),
        Err(image_err) =>
            fail(env, &image_err)
    }
}

// blur/1 (image) -> image
fn blur<'a>(env: NifEnv<'a>, args: &Vec<NifTerm<'a>>) -> NifResult<NifTerm<'a>> {
    let image: ResourceCell<Image> = args[0].decode()?;
    let blurred = image.0.blur(7.0);
    Ok(ResourceCell::new(Image(blurred)).encode(env))
}

