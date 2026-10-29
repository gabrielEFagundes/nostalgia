use comrak::{Arena, Options, parse_document};

pub fn parse(document: String) {
    let arena = Arena::new();

    let root = parse_document(&arena, &document, &Options::default());
    
    for i in root.descendants(){
        println!("\n{:#?}", i);
    }
}
