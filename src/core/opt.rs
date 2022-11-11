use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "colorcc",
    about = "🎨 A cli tool to give you some color see see."
)]
pub struct Opt {
    #[structopt()]
    pub color_name: String,
}
