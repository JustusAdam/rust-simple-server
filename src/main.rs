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
    let cd = current_dir().unwrap();
    let cd_as_str : &str = cd.to_str().unwrap();
    let as_str : &str = &path.to_str().unwrap();

    return PathBuf::from(
        format!("{}{}",
            cd_as_str,
            if path.has_root() && cd_as_str.ends_with("/") {
                &as_str[1..]
            } else {
                as_str
            }
        ));
}


fn serve_path(path:PathBuf, res:Response) {
    let lpath = to_system_path(path);
    println!("Serving {}", lpath.to_str().unwrap());
    if !lpath.exists() {
        println!("Path does not exist");
        res.send(b"Could not find path");
    } else if lpath.is_dir() {
        server_dir(lpath, res);
    } else if lpath.is_file() {
        serve_doc(lpath, res);
    } else {
        println!("Unexpected error, path exists but is neither file nor directory.");
    }
}


fn server_dir(path:PathBuf, res:Response) {
    let message = path.read_dir().unwrap().fold(String::new(), |acc, item| {
        let p = item.unwrap().path();
        let itemstr : &str = p.to_str().unwrap();
        if acc.is_empty() {
            return acc + itemstr;
        } else {
            return acc + "\n" + itemstr;
        }
    });
    res.send(message.as_bytes());
}


fn serve_doc(path:PathBuf, res:Response) {
    let mut fp = match File::open(path) {
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
        return serve_path(path, res);
    });
}
