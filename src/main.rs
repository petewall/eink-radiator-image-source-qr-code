use clap::{Parser, Subcommand};
use image::Luma;
use qrcode::QrCode;
use std::cmp;
// use std::path::PathBuf;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Prints a blank configuration
    Config {},

    /// Generates a QR code image
    Generate {
        #[clap(long)]
        width: u32,
        #[clap(long)]
        height: u32
    },

    /// Prints the version of this utility
    Version {},
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Config {} => {
            println!("myapp config")
        }
        Commands::Generate { height, width } => {
            println!("'myapp generate' {:?}x{:?}", width, height);

            // Encode some data into bits.
            let code = QrCode::new(b"http://petewall.net").unwrap();

            // Render the bits into an image.
            let dimension = cmp::min(width, height);
            let image = code.render::<Luma<u8>>()
                .quiet_zone(true)
                .min_dimensions(*dimension, *dimension)
                .build();

            Luma<u8>::
            image.save("qrcode.png").unwrap();

        }
        Commands::Version {} => {
            println!("myapp version")
        }
    }
}
