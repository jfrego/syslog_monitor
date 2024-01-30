mod db;
mod schema;
mod models;
mod timer;
mod send_mail;

use std::fs::File;
use std::io::{self, BufReader, SeekFrom};
use std::io::prelude::*;
use std::{thread, time::Duration};
use db::get_conn_pool;
use regex::Regex;
use timer::{init_time, timer_watch_dog};
use diesel::r2d2::{PooledConnection, ConnectionManager};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;


//keeps track of new lines added to syslog file
fn syslog_watcher(conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>) {
    dotenv().ok();
    let file_path = env::var("FILE_PATH").expect("File path env var not set!!");

    let mut f = File::open(file_path).unwrap();
    let mut x = f.seek(SeekFrom::End(0)).unwrap();

    loop {      
        let y = f.seek(SeekFrom::End(0)).unwrap();

        if x == y {
            thread::sleep(Duration::from_millis(100));
            continue;
        } else if x < y {
            f.seek(SeekFrom::Start(x)).unwrap();
            let reader = BufReader::new(&mut f);
            
            for l in reader.lines() {
                line_parser(l, conn);
            }
            x = y;
        }
    }
}

//match each line for a specific re pattern and retrieve a mac address
fn line_parser(l: Result<String, io::Error>, conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>) {
    let patrn = Regex::new(r"_STAT.*CPU:").unwrap();
    let patrn_grp = Regex::new(r"GRP2604P.*Memory").unwrap();
    let patrn_gxp = Regex::new(r"GXP2160.*Memory").unwrap();
    let l = l.unwrap().to_string();
    let mac_ptrn_l = Regex::new(r"..:...:..:..:..:..").unwrap();

    if patrn.is_match(&l) {
        let mac_ptrn = Regex::new(r"..:..:..:..:..:..").unwrap();
        let mac = mac_ptrn.find(&l).unwrap();
        timer::timer_reset(mac.as_str(), conn); 

    } else if patrn_grp.is_match(&l) {        
        let mac = mac_ptrn_l.find(&l).unwrap();
        timer::timer_reset(&mac.as_str().replace(" ", ""), conn);

    } else if patrn_gxp.is_match(&l) {
        let mac = mac_ptrn_l.find(&l).unwrap();
        timer::timer_reset(&mac.as_str().replace(" ", ""), conn);
    }         
}



fn main() {
    let pool = get_conn_pool();    
    let pool1 = pool.clone();

    init_time(&mut pool.get().unwrap());
    let handle = thread::spawn(move || syslog_watcher(&mut pool.get().unwrap()));
    thread::spawn(move || timer_watch_dog(&mut pool1.get().unwrap()));
    
    handle.join().unwrap(); 
}
