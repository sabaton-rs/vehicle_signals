use core::panic;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::process;
use csv;

use petgraph::Directed;
use petgraph::graph::{NodeIndex, Graph};
use quote::{quote,format_ident,TokenStreamExt};
use proc_macro2::TokenStream;

//use petgraph::algo::{dijkstra, min_spanning_tree};
//use petgraph::data::FromElements;

const VSS_CSV_FILE : &str= "vss_rel_2.2-develop.csv";

#[derive(Debug)]
struct Signal {
    module : Vec<String>,
    name : String,
    kind : String,
    datatype : TokenStream,
    complex: String,
    unit : Option<String>,
    min : Option<String>,
    max : Option<String>,
    description : String,
    enumeration : String,
    id : String,
    default: Option<String>,
}

fn parse_csv() -> Result<Vec<Signal>, Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(File::open(VSS_CSV_FILE).unwrap());
    let mut signals = Vec::new();
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        if record[1].contains("branch") {
            //println!("Skipping branch: {}", &record[0]);
        } else {

            let (mod_path, name) = name_to_module_and_typename(record[0].into());

            let sig = Signal {
                module : mod_path,
                name,
                kind: record[1].into(),
                datatype: vss_type_to_rust_type(record[2].into()),
                complex: record[4].into(),
                unit: Some(record[5].into()),
                min: Some(record[6].into()),
                max: Some(record[7].into()),
                description: record[8].into(),
                enumeration: record[9].into(),
                id: record[1].into(),
                default: None,
            };
            signals.push(sig);
        }

        //println!("{:?}", record);
    }
    Ok(signals)
}

fn main() {
    if let Ok(s) = parse_csv() {
        let mut g = Graph::<(String,Vec<Signal>), ()>::new();
        let root_node = ("ROOT".to_owned(),vec![]);
        
        let root_index = g.add_node(root_node);
        for sample in s {
            let mut cur_index = root_index;
            for module in &sample.module {
                match g.neighbors(cur_index).find(|n| { 
                    if g[*n].0 == *module {
                        true
                    } else { false}
                }) {
                    Some(n) => {
                        cur_index = n;
                    },
                    None => {
                        // create the node
                    let new_node = g.add_node((module.into(),vec![]));
                    g.add_edge(cur_index,new_node, ());
                    cur_index = new_node;
                    },
                }
        
            }
            let node = &mut g[cur_index];
            node.1.push(sample);

        }

        graph_to_output(g, root_index);
        
        process::exit(0);
    }
}

fn add_signal(s : &Signal) -> TokenStream {
    let signal_name =  quote::format_ident!("{}",&s.name);
    let documentation =  format!("{}",&s.description);
    let ty = &s.datatype;
     quote!{
        #[doc=#documentation]
        #[allow(non_camel_case_types)]
        #[derive(Default, Deserialize, Serialize, Topic)]
        pub struct #signal_name {
            v : #ty,
            timestamp : u64,
        }

        impl #signal_name {
            pub fn timestamp(&self) -> u64 {
                self.timestamp
            }

            pub fn value(&self) -> &#ty {
                &self.v
            }

            pub fn new(value : #ty, timestamp: Option<u64>) -> Self {
                Self {
                    v: value,
                    timestamp : timestamp.unwrap_or(0),
                }
            }


        }

    }
}

fn add_module(g: &Graph<(String, Vec<Signal>), (), Directed, u32>, module_index : NodeIndex) -> TokenStream {
    
    let module_name =  quote::format_ident!("{}",  &g[module_index].0.to_lowercase());
    let mut module_ts = TokenStream::new();

    for c in g.neighbors(module_index) {
        module_ts.extend(add_module(g,c));
    }   

    let mut signal_contents = TokenStream::new();

    for s in  &g[module_index].1 {
        
        signal_contents.extend(add_signal(s))
    }

    quote! {
        #[allow(non_snake_case)]
        pub mod #module_name {
            use cyclonedds_rs::{*};
            #signal_contents
            #module_ts
        }
    }

}

fn graph_to_output(g: Graph<(String, Vec<Signal>), (), Directed, u32>, root_index : NodeIndex) {
    let outdir = env::var("OUT_DIR").expect("OUT_DIR is not set");
    let bindings_file = Path::new(&outdir).join("bindings.rs");

    let mut file = std::fs::File::create(bindings_file).expect("create failed");

    for module in  g.neighbors(root_index) {
        let ts = add_module(&g, module);
        file.write_all(ts.to_string().as_bytes()).unwrap();

    }
    
}

fn vss_type_to_rust_type(vss_type: &str) -> TokenStream {

    let vss =  vss_type.trim().trim_end_matches("[]");

    let rust_type = match vss {
        "uint8" => quote!{u8},
        "int8" => quote!{i8},
        "uint16" => quote!{u16},
        "int16" => quote!{i16},
        "uint32" => quote!{u32},
        "int32" => quote!{i32},
        "uint64" => quote!{u64},
        "int64" => quote!{i64},
        "boolean" => quote!{bool},
        "float" => quote!{f32},
        "double" => quote!{f64},
        "string" => quote!{String},
        "byteBuffer" => quote!{Vec<u8>},
        _ => panic!("Unknown type"),
    };

    if vss_type.trim().ends_with("[]") {
        quote!{Vec<#rust_type>}    
    } else {
        rust_type.into()
    }
    
}

fn name_to_module_and_typename(name: &str) -> (Vec<String>, String) {
    
    let typename = name.split(".").last().unwrap();
    let module_path= name.split(".");

    let mut module = Vec::new();

    for p in module_path {
        module.push(p.to_owned());
    }

    let name = module.pop().unwrap();

    (module, typename.into())
}