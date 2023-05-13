use std::{fmt::Display, fs, io::Write, process::ExitCode};

use clap::{Parser, Subcommand};
use fmf::{document::Document, format::DefaultFormatEngine};
use serde::Serialize;

/// FMF (field, millet, fox) format compiler and project manager
#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    sub: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Query document metadata
    #[command(alias = "q")]
    Query {
        /// Document
        filename: String,

        /// Print query data in json format
        #[arg(long)]
        json: bool,
    },
    /// Build document into HTML
    Cc {
        /// Document
        filename: String,

        /// Template to use
        #[arg(long)]
        template: Option<String>,

        /// Output filename (default to stdout)
        #[arg(short, long)]
        output: Option<String>,
    },
}

#[derive(Serialize)]
struct QueryData<'a> {
    title: &'a str,
    author: Option<&'a str>,
    date: Option<&'a str>,
    category: Option<&'a str>,
}

impl<'a> Display for QueryData<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Title: {}", self.title)?;
        if let Some(author) = self.author {
            writeln!(f, "\n\nAuthor: {author}")?;
        }
        if let Some(date) = self.date {
            writeln!(f, "Date: {date}")?;
        }
        if let Some(category) = self.category {
            write!(f, "Category: {category}")?;
        }

        Ok(())
    }
}

fn main() -> ExitCode {
    let Args { sub } = Args::parse();

    match sub {
        Commands::Cc {
            filename,
            template,
            output,
        } => {
            let s = match fs::read_to_string(filename) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Failed to read file: {e}");
                    return ExitCode::FAILURE;
                }
            };
            let doc = Document::<DefaultFormatEngine>::new(&s);
            let out = if let Some(template) = template {
                let out = format!("{doc}");
                let title = doc.get_title();
                let (author, date, category) = doc.get_author_card().unwrap_or_default();
                match fs::read_to_string(template) {
                    Ok(v) => v
                        .replace("{contents}", &out)
                        .replace("{title}", title)
                        .replace("{author}", author)
                        .replace("{date}", date)
                        .replace("{category}", category),
                    Err(e) => {
                        eprintln!("Failed to read template file: {e}");
                        return ExitCode::FAILURE;
                    }
                }
            } else {
                format!("{doc}")
            };
            if let Some(output) = output {
                if let Err(e) | Ok(Err(e)) =
                    fs::File::create(output).map(|mut f| f.write(out.as_bytes()))
                {
                    eprintln!("Failed to write to file: {e}");
                    return ExitCode::FAILURE;
                }
            } else {
                print!("{out}");
            }
        }

        Commands::Query { filename, json } => {
            let s = match fs::read_to_string(filename) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Failed to read file: {e}");
                    return ExitCode::FAILURE;
                }
            };
            let doc = Document::<DefaultFormatEngine>::new(&s);
            let query = {
                let title = doc.get_title();
                let (author, date, category) = match doc.get_author_card() {
                    Some((a, b, c)) => (Some(a), Some(b), Some(c)),
                    None => (None, None, None),
                };
                QueryData {
                    title,
                    author,
                    date,
                    category,
                }
            };
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&query).expect("json serialization")
                );
            } else {
                println!("{query}");
            }
        }
    }

    ExitCode::SUCCESS
}
