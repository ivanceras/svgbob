extern crate handlebars;
use std::io::{Write, Read};
use std::fs::File;

use std::collections::BTreeMap;

use handlebars::Handlebars;
use handlebars::Context;

fn main() {
    let mut handlebars = Handlebars::new();

    let mut m: BTreeMap<String, String> = BTreeMap::new();
    m.insert("bob".to_string(),
             include_str!("long.bob").to_owned());
    let context = Context::wraps(&m);


    let mut source_template = File::open(&"web/index.hbs").unwrap();
    let mut output_file = File::create("index.html").unwrap();
    if let Ok(_) = handlebars.template_renderw2(&mut source_template, &context, &mut output_file) {
        println!("Rendered to {:?}", output_file);
    } else {
       println!("Error"); 
    };
}
