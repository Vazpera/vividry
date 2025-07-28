extern crate clap;
extern crate colored;
extern crate vividry;
use clap::*;
use colored::Colorize;
use vividry::Color;



#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Clone, Debug)]
enum Command {
    Gradient { 
        colors: Vec<Color>,
        #[arg(short, long, default_value="5")]
        number: u32
    },
    Convert {
        colors: Vec<Color>,
    }
}

fn gradient(colors: Vec<Color>, number: u32) {
    for i in 0..number {
        let bias = i as f64 / (number - 1) as f64 * (colors.len() as f64 - 1.0);
        let index_0 = bias.floor() as usize;
        let index_1 = usize::clamp(bias.floor() as usize + 1, 0, colors.len() -1);
        let color_0 = colors[index_0].to_rgb();
        let color_1 = colors[index_1].to_rgb();
        let interpol = bias.fract();
        let color = Color::from_rgbf(
            color_0[0]*(1.0-interpol) + color_1[0]*interpol, 
            color_0[1]*(1.0-interpol) + color_1[1]*interpol, 
            color_0[2]*(1.0-interpol) + color_1[2]*interpol, 
        ).unwrap();
        let colorgb = color.to_rgb();
        println!("{}", color.to_hex().black().on_truecolor(colorgb[0] as u8, colorgb[1] as u8, colorgb[2] as u8));
    }
}
fn convert(colors: Vec<Color>) {
    for color in colors {
        let rgb = color.to_rgb();
        let formatted_string = format!("rgb({}, {}, {})", rgb[0], rgb[1], rgb[2]).black().on_truecolor(rgb[0] as u8, rgb[1] as u8, rgb[2] as u8);
        
        println!("{formatted_string}")

    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    match args.command {
        Command::Gradient { colors, number } => gradient(colors, number),
        Command::Convert { colors } => convert(colors),
    }
    Ok(())
}
