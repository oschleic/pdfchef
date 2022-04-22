#![allow(unused)]
use clap::Parser;
use std::path::{Path, PathBuf};
use lopdf::{Document, Object, Stream};

#[derive(Parser)]
struct Cli{
    /// The pdf output file (replaces the input file by default)
    #[clap(parse(from_os_str))]
    #[clap(short, long)]
    output: Option<PathBuf>,
    /// The input pdf file
    #[clap(parse(from_os_str))]
    file: PathBuf,
    /// The page numbers to extract
    #[clap(short, long)]
    pages: Option<String>,

}

fn load_pdf(path: &Path) -> Document {
    let mut doc = Document::load(path).unwrap();
    doc
}


fn save_pdf(path: &Path, doc: &mut Document) {
    doc.save(path);
}

fn extract_pages(doc: &mut Document, pages: &str) -> Document {
    let mut new_doc = doc.clone();

    let pages = pages.split(',').map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let mut page_map = doc.get_pages();

    let mut to_remove = vec![];

    for(i, obj) in page_map.iter(){
        if !pages.contains(&i){
            to_remove.push(*i);
        }
    }



   new_doc.delete_pages(to_remove.as_slice());

   new_doc
}




fn main() {
    let args = Cli::parse();
    save_pdf(args.output.unwrap().as_path(), &mut extract_pages(&mut load_pdf(args.file.as_path()), args.pages.unwrap().as_str()));
}
