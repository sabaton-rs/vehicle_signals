use core::panic;
use std::borrow::Cow;
use std::convert::TryFrom;
use std::str::FromStr;
use std::{env, time};
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{self, Command, Stdio};
use csv;
use which;

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
                min: if record[6].len() > 0 { Some(record[6].into())} else {None},
                max: if record[7].len() > 0 { Some(record[7].into())} else {None},
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

    let max_in_range = if s.max.is_some() {
        let max_with_type = format!("{}{}",s.max.as_ref().unwrap(),ty);
        let max = TokenStream::from_str(&max_with_type).unwrap();
        quote!{
            *v <= #max 
        } 
    } else {
        quote!{true}
    };

    let min_in_range = if s.min.is_some() {
        let min_with_type = format!("{}{}",s.min.as_ref().unwrap(),ty);
        let min = TokenStream::from_str(&min_with_type).unwrap();
        quote!{
            *v >= #min 
        } 
    } else {
        quote!{true}
    };

    let verify = 
    if s.max.is_some() || s.min.is_some() {
        quote!{
            ///check if the given value is within the limits defined
            ///in the specification. Return true if the value is
            ///within bounds.
            pub fn bounds_check(v : &#ty) -> bool {
                #max_in_range && #min_in_range
            }
        }

    } else {
        quote!{ 
            ///check if the given value is within the limits defined
            ///in the specification. This particular type has not
            ///specified the min or max limits so the function just
            /// returns true
            const fn bounds_check(_v : &#ty) -> bool { true}
        }
    };

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

            /// Get the value stored in this type
            pub fn value(&self) -> &#ty {
                &self.v
            }

            /// set the value. Ensure that the value is within bounds as per the
            /// specification. This function will panic in case the value is out
            /// of bounds.
            pub fn set(&mut self, value: #ty, maybe_timestamp : Option<u64>) {
                assert!(Self::bounds_check(&value));
                self.v = value;
                if let Some(ts) = maybe_timestamp {
                    self.timestamp = ts;
                }
            }

            #verify

            /// create a new instance
            pub fn new(value : #ty, timestamp: Option<u64>) -> Option<Self> {
                if Self::bounds_check(&value) {
                    Some(Self {
                        v: value,
                        timestamp : timestamp.unwrap_or(0),
                    })
                }   else {
                    None
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

    //let mut signal_found = false;
    for s in  &g[module_index].1 {
        
        signal_contents.extend(add_signal(s))
    }

    let import_cyclonedds_rs = if g[module_index].1.len() > 0 {
        quote!{use cyclonedds_rs::{*};}
    } else {
        quote!{}
    };

    quote! {
        #[allow(non_snake_case)]
        pub mod #module_name {
            #import_cyclonedds_rs
            #signal_contents
            #module_ts
        }
    }

}

fn graph_to_output(g: Graph<(String, Vec<Signal>), (), Directed, u32>, root_index : NodeIndex) {
    let outdir = env::var("OUT_DIR").expect("OUT_DIR is not set");
    let bindings_file = Path::new(&outdir).join("bindings.rs");

    let mut file = std::fs::File::create(bindings_file).expect("create failed");

    let mut generated_code = String::new();

    for module in  g.neighbors(root_index) {
        let ts = add_module(&g, module);

        generated_code.push_str(&ts.to_string());

    }

    let generated_code = rustfmt_generated_code(&generated_code).expect("Unable to run rustfmt");

    file.write_all(generated_code.as_bytes()).unwrap();
    
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


fn rustfmt_path<'a>() -> io::Result<Cow<'a, PathBuf>> {
    match which::which("rustfmt") {
        Ok(p) => Ok(Cow::Owned(p)),
        Err(e) => {
            Err(io::Error::new(io::ErrorKind::Other, format!("{}", e)))
        }
    }
}

fn rustfmt_generated_code<'a>(
        source: &'a str,
    ) -> io::Result<Cow<'a, str>> {
//        let _t = time::Timer::new("rustfmt_generated_string")
//            .with_output(self.options.time_phases);

        let rustfmt = rustfmt_path()?;
        let mut cmd = Command::new(&*rustfmt);

        cmd.stdin(Stdio::piped()).stdout(Stdio::piped());

        /* 
        if let Some(path) = self
            .options
            .rustfmt_configuration_file
            .as_ref()
            .and_then(|f| f.to_str())
        {
            cmd.args(&["--config-path", path]);
        }
        */

        let mut child = cmd.spawn()?;
        let mut child_stdin = child.stdin.take().unwrap();
        let mut child_stdout = child.stdout.take().unwrap();

        let source = source.to_owned();

        // Write to stdin in a new thread, so that we can read from stdout on this
        // thread. This keeps the child from blocking on writing to its stdout which
        // might block us from writing to its stdin.
        let stdin_handle = ::std::thread::spawn(move || {
            let _ = child_stdin.write_all(source.as_bytes());
            source
        });

        let mut output = vec![];
        io::copy(&mut child_stdout, &mut output)?;

        let status = child.wait()?;
        let source = stdin_handle.join().expect(
            "The thread writing to rustfmt's stdin doesn't do \
             anything that could panic",
        );

        match String::from_utf8(output) {
            Ok(bindings) => match status.code() {
                Some(0) => Ok(Cow::Owned(bindings)),
                Some(2) => Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Rustfmt parsing errors.".to_string(),
                )),
                Some(3) => {
                    //warn!("Rustfmt could not format some lines.");
                    Ok(Cow::Owned(bindings))
                }
                _ => Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Internal rustfmt error".to_string(),
                )),
            },
            _ => Ok(Cow::Owned(source)),
        }
    }