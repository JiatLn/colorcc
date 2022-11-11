use crate::color::{base_color::BaseColor, namer::ColorNamer};
use crate::core::opt::Opt;
use structopt::StructOpt;

mod color;
mod core;

fn main() {
    let opt = Opt::from_args();

    let namer = ColorNamer;
    println!("{:?}", namer);

    let color = BaseColor::from(opt.color_name);
    println!("{:?}", color);

    println!("Hello, colorcc!");
}
