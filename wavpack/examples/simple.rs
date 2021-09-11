use anyhow::Result;
use cue::{cd::CD, cd_text::PTI};
use std::path::Path;
use wavpack::*;

fn main() -> Result<()> {
    let version = get_library_version_string();
    println!("WavPack version = {}", version);
    let mut context = ContextBuilder::new(Path::new("a.wv")).tags().build()?;
    let mode = context.get_mode()?;
    dbg!(mode);
    let tags = context.get_all_tag_items()?;
    dbg!(&tags);
    if let TagData::Text(x) = &tags["Cuesheet"] {
        println!("{}", x);
        let cd = CD::parse(x)?;
        let tracks = cd.tracks();
        for t in tracks {
            let c = t.get_cdtext();
            let s = t.get_start();
            let l = t.get_length();
            println!(
                "titile = {:?}, start = {}, length = {}",
                c.read(PTI::Title),
                s,
                l
            );
        }
    }
    let context = ContextBuilder::new(Path::new("not_exist.wv")).build();
    assert_eq!(context.err().unwrap().to_string(), "can't open file");
    Ok(())
}
