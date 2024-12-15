mod init_route;
mod optimize;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

const INIT_GREEDY: u32 = 0x00000001;
const INIT_KRUSCAL: u32 = 0x00000002;
const OPT_2OPT: u32 = 0x00010000;
const OPT_OR1OPT: u32 = 0x00040000;
const OPT_OR2OPT: u32 = 0x00080000;
const MASK_INIT: u32 = 0x0000FFFF;
const ERROR: u32 = 0xFFFFFFFF;

const MODE_DAT: u32 = 0;
const MODE_CSV: u32 = 1;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("4 args needed");
        return;
    }

    // 引数からフラグ取得
    let flag = match args[1].parse::<u32>() {
        Ok(f) => f,
        Err(_) => {
            eprintln!("flag parse error");
            ERROR
        }
    };
    if flag == ERROR {
        return;
    }

    // 引数からモード取得
    let mode = match args[2].parse::<u32>() {
        Ok(f) => {
            if f == MODE_DAT {
                FileType::Dat
            } else {
                FileType::Csv
            }
        }
        Err(_) => {
            eprintln!("mode parse error");
            return;
        }
    };

    let in_filename = args[3].as_str();
    let out_filename = args[4].as_str();

    let (x, y) = read_points_file(in_filename, mode);
    let n = x.len();

    let mut route;
    let dist;
    (route, dist) = match flag & MASK_INIT {
        INIT_GREEDY => init_route::greedy(n, x, y),
        INIT_KRUSCAL => init_route::christofides_kruscal(n, x, y),
        _ => (Vec::<usize>::new(), Vec::<Vec<usize>>::new()),
    };

    // 最適化
    if flag & OPT_2OPT == OPT_2OPT {
        route = optimize::opt2(&route, &dist);
    }
    if flag & OPT_OR2OPT == OPT_OR2OPT {
        route = optimize::or2opt(&route, &dist);
    }
    if flag & OPT_OR1OPT == OPT_OR1OPT {
        route = optimize::or1opt(&route, &dist);
    }

    write_route_file(out_filename, route);
}

fn read_points_file(filename: &str, filetype: FileType) -> (Vec<isize>, Vec<isize>) {
    let mut a = Vec::<isize>::new();
    let mut b = Vec::<isize>::new();
    let delim = match filetype {
        FileType::Dat => ' ',
        FileType::Csv => ',',
    };

    for result in BufReader::new(File::open(filename.trim()).unwrap()).lines() {
        let line = match result {
            Ok(line) => line,
            Err(_) => continue,
        };
        let v: Vec<isize> = line.split(delim).map(|k| k.parse().unwrap()).collect();
        a.push(v[0]);
        b.push(v[1]);
    }
    (a, b)
}

fn write_route_file(filename: &str, route: Vec<usize>) {
    let mut writer = BufWriter::new(File::create(filename.trim()).unwrap());
    for r in route {
        let line = format!("{}\n", r);
        match writer.write(line.as_bytes()) {
            Ok(_) => (),
            Err(e) => eprintln!("{}", e),
        };
    }
}

enum FileType {
    Dat,
    Csv,
}
