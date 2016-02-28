extern crate argparse;
extern crate hyper;

use hyper::server::{Server, Request, Response};
use hyper::uri::RequestUri::{AbsolutePath, AbsoluteUri};
use argparse::{ArgumentParser, Store};
use std::path::{PathBuf};
use std::fs::{File};
use std::vec::{Vec};
use std::io::{Read};
use std::env::{current_dir};


fn main() {
    let mut port = "8000".to_string();
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("A simple document server");
        parser.refer(&mut port)
            .add_option(&["-p", "--port"], Store, "Which port to run on");
    }
    let mut fport: &str = &("0.0.0.0:".to_string() + &port);
    Server::http(fport).unwrap().handle(handler).unwrap();
}


fn get_path(req : Request) -> Option<PathBuf> {
    return match req.uri {
        AbsolutePath(str) => Some(PathBuf::from(str)),
        AbsoluteUri(uri) => uri.to_file_path().ok(),
        _ => {
            println!("{:?}", req.uri);
            return None;
        }
    }
}


fn to_system_path(path : PathBuf) -> PathBuf {
    let cd_as_str = current_dir().unwrap().to_str().unwrap();
    let as_str = path.to_str().unwrap();
    cd_as_str.push(as_str[1..]);
    return PathBuf::from( + slash_dropped);
}


fn serve_doc(path:PathBuf, res:Response) {
    let lpath = path.relative_from("/");
    let mut fp = match File::open(lpath) {
        Ok(fp) => fp,
        Err(_) => {
            println!("File not found!");
            return;
        }
    };
    let len = match fp.metadata() {
        Ok(meta) => meta.len(),
        Err(_) => {
            println!("File does not have metadata");
            return;
        }
    };
    let mut buf = Vec::with_capacity(len as usize);
    fp.read_to_end(&mut buf);
    res.send(&buf[..]);
}


fn handler(req: Request, res: Response) {
    println!("{:?}", req.uri);
    get_path(req).map(|path| {
        println!("Recieved request for {}", path.to_str().unwrap());
        return serve_doc(path, res);
    });
}
