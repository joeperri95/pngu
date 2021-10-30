use std::path::PathBuf;
use structopt::StructOpt;

mod imgur;
mod decode;
mod encode;
mod utils;
mod chunk;

#[derive(StructOpt, Debug)]
#[structopt(name="PNGu")]
enum Opt{

    Encode {
        #[structopt(short="w", long)]
        use_web: bool,

        #[structopt(short, long, default_value="processed.png", parse(from_os_str))]
        output: PathBuf,

        #[structopt(short, long, parse(from_os_str))]
        input: Option<PathBuf>,
        
        #[structopt(short, long)]
        message: String,
    },

    Decode {
        #[structopt(short, long, parse(from_os_str))]
        input: PathBuf,

    },
}

fn main() {
    
    let opt= Opt::from_args();
    println!("{:#?}", opt);
    
    match opt
    {
        Opt::Encode {use_web, output, input, message} =>
        {
           
           let filetodo: String;
            if use_web
            {
                filetodo = imgur::get_random_png();
            }
            else
            {
                match input{
                    Some(p) => {
                        filetodo = p.into_os_string().into_string().unwrap();
                        },
                    None => {println!("Please enter either an input file or specify the use-web
                    flag");return;}
                }
            }             
            
            let output_chosen = output.into_os_string().into_string().unwrap();
            let output_available = utils::get_available_filename(&output_chosen); 

            println!("input {:?} output {:?}", filetodo, output_available);
            encode::process_png(filetodo, output_available, &message);
            println!("done");
        },
        Opt::Decode {input} => {
            decode::process_png(input.to_str().unwrap().to_string());
        },
    };
}


