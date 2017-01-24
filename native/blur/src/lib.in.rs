#[macro_use] extern crate rustler;
#[macro_use] extern crate lazy_static;
extern crate image;

use rustler::{NifEnv, NifTerm, NifResult, NifEncoder};
use rustler::resource::ResourceCell;
use image::DynamicImage;

mod atoms {
    rustler_atoms! {
        atom ok;
        //atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
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

// open/1 (filename) -> {:ok, image}
fn open<'a>(env: NifEnv<'a>, args: &Vec<NifTerm<'a>>) -> NifResult<NifTerm<'a>> {
    let filename: String = args[0].decode()?;
    Ok(atoms::ok().encode(env))
}

// save/2 (image, filename) -> :ok
fn save<'a>(env: NifEnv<'a>, args: &Vec<NifTerm<'a>>) -> NifResult<NifTerm<'a>> {
    let image: ResourceCell<Image> = args[0].decode()?;
    let filename: String = args[1].decode()?;
    Ok(atoms::ok().encode(env))
}

// blur/1 (image) -> image
fn blur<'a>(env: NifEnv<'a>, args: &Vec<NifTerm<'a>>) -> NifResult<NifTerm<'a>> {
    let image: ResourceCell<Image> = args[0].decode()?;
    Ok(atoms::ok().encode(env))
}

