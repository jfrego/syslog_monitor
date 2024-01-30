use crate::models;
use crate::schema::extensions::dsl::*;
use crate::send_mail;

use chrono::NaiveTime;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use models::Extension;
use diesel::prelude::*;
use std::{thread::sleep, time::Duration};
use chrono::Local;


//reset time stamp for each given mac address
pub fn timer_reset(mac_str: &str, conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>) {
    let t = Local::now().time().to_string();
    diesel::update(extensions.find(mac_str)).set((timer.eq(t), mail.eq(false)))
        .execute(conn).expect("Error reseting timer and email!!");
}


//continuously match time stamp on records with the actual time
pub fn timer_watch_dog(conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>) {
    loop {
        sleep(Duration::from_secs(3));
        let exts = extensions.load::<Extension>(conn)
            .expect("Error query extensions timer wadog!");
        let t = Local::now().time();
        for ex in exts {
            let t_ex = NaiveTime::parse_from_str(&ex.timer, "%H:%M:%S%.f").unwrap();

            if (t - t_ex).num_seconds() >= 300 && ex.mail == false {
                diesel::update(extensions.find(ex.mac)).set(mail.eq(true))
                    .execute(conn).expect("Error setting mail to true!!");

                send_mail::mail_sender(ex.extension, ex.domain);
            }
        }
    } 
}


//set the initial time stamp on each record
pub fn init_time(conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>) {
    let t = Local::now().time().to_string();
    diesel::update(extensions).set(timer.eq(t)).execute(conn)
        .expect("Error seting init timer!!!");
}

