use std::{time::Instant, sync::atomic::AtomicU64};

use walkdir::WalkDir;
use rayon::prelude::*;

fn main() {
    // rayon::ThreadPoolBuilder::new().num_threads(32).build_global().unwrap();
    println!("removing all the target/ directories");

    let time = Instant::now();
    let size = AtomicU64::new(0);
    let files = WalkDir::new(std::env::current_dir().unwrap())
        .max_open(512)
        .into_iter()
        .par_bridge()
        .filter_map(|x| x.ok())
        .filter_map(|x| {
            if let Ok(v) = x.metadata() {
                if v.is_dir() {
                    Some((x, v))
                } else { None }
            } else { None }
        })
        .filter(|x| x.0.file_name().to_string_lossy() == "target")
        .filter_map(|(i, _)| {
            let the_parent_of_the_target = i.path().parent().unwrap();
            if std::fs::metadata(the_parent_of_the_target.join("Cargo.toml")).ok().is_some() {
                size.fetch_add(fs_extra::dir::get_dir_content(i.path()).unwrap().dir_size, std::sync::atomic::Ordering::SeqCst);
                Some(i.path().to_path_buf())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    
    println!("search time {}ms", time.elapsed().as_millis());

    println!("this operation will remmove {:.2}gbs of files a total of {} directories. are you sure? (y/n): ", size.into_inner() as f64 / 1e+9, files.len());
    let mut input = String::with_capacity(10);
    std::io::stdin().read_line(&mut input).unwrap();
    
    if input.trim() != "y" {
        println!("operation cancelled");
        return
    }

    for f in &files {
        println!("{}", f.to_string_lossy());
    }
    trash::delete_all(&files).unwrap();
    
}
