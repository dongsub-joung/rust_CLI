use chrono::{Datelike, NaiveDate};
use rusqlite::{Connection, Result};
use std::io::{self, stdin, BufRead, Write};

mod day;

fn main() {
    fn day_inputing() -> Vec<u32> {
        let std = stdin();
        let mut input = std.lock().lines();
        let v: Vec<u32> = input
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|f| f.parse().unwrap())
            .collect();

        v
    }

    fn say_difference(difference: &i64) {
        if difference > &0 {
            println!("Days remaining until D-Day: {}", difference);
        } else if difference == &0 {
            println!("Today is D-Day!");
        } else {
            println!("D-Day was {} days ago.", difference.abs());
        }
    }

    loop {
        let v = day_inputing();

        let today = chrono::offset::Local::today().naive_local();

        let d_day = NaiveDate::from_ymd(v[0] as i32, v[1], v[2]);

        let difference = d_day.signed_duration_since(today).num_days();

        say_difference(&difference);

        // DB
        let conn = Connection::open("dday.db").expect("err");

        conn.execute(
            "CREATE TABLE IF NOT EXISTS dday (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                yyyy INTEGER,
                mm INTEGER,
                dd INTEGER,
                difference INTEGER
            )",
            [],
        )
        .expect("DB_Creating_Err");

        let d_day = day::Day::new(v[0], v[1], v[2], difference as u32);

        conn.execute(
            "INSERT INTO dday (yyyy, mm, dd, difference) VALUES (?, ?, ?, ?)",
            &[&d_day.yyyy, &d_day.mm, &d_day.dd, &d_day.difference],
        )
        .expect("DB_Insert_Err");

        let mut stmt = conn.prepare("SELECT id, yyyy, mm, dd, difference FROM dday").expect("DB_Get_Err");
        let dday_iter = stmt.query_map([], |row| {
            Ok(day::DayOutPut {
                id: row.get(0)?,
                yyyy: row.get(1)?,
                mm: row.get(2)?,
                dd: row.get(3)?,
                difference: row.get(4)?,
            })
        }).expect("Parsing_Err");


        for dday in dday_iter {
            let dday_info = dday.expect("err");
            println!("ID: {}: {} {} {}, "
                , dday_info.id
                ,dday_info.yyyy
                ,dday_info.mm
                ,dday_info.dd
            );
            println!("Difference in days until D-Day: {}", dday_info.difference);
        }
    }
}
