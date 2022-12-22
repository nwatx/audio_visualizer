use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// input path of the mp3, webm, etc.
    #[arg(short, long)]
    input_path: String,

    /// output path of the temporary mp4 file
    #[arg(short, long)]
    temp_path: String,

    /// output path of the final mp4 file
    #[arg(short, long)]
    output_path: String,
}

impl Args {
    pub fn input_path(&self) -> &str {
        self.input_path.as_str()
    }

    pub fn temp_path(&self) -> &str {
        self.temp_path.as_str()
    }

    pub fn output_path(&self) -> &str {
        self.output_path.as_str()
    }
}