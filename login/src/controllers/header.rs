// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT
use base64::encode;
use base64::decode;
use redis::Client;
use redis::Commands;

use crate::ui::*;
use slint::*;
use chrono::Local;
use chrono::NaiveDateTime;
use std::str;
use shared_string::SharedString;


pub fn setup(window: &MyLoginWindow) -> Timer {
    let update_timer = Timer::default();
    update_timer.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(300), {
        let weak_window = window.as_weak();

        move || {
            update(&weak_window.unwrap().global::<HeaderAdapter>());
        }
    });

    update_timer
}

fn update(header_adapter: &HeaderAdapter) {
    // Connect to Redis server
    let client = Client::open("redis://:redis-stack@10.8.8.2/").unwrap();
    let mut connection = client.get_connection().unwrap();

    //let rlic: String = decode(connection.get("license").as_bytes());
    let rlic: String = connection.get("license").unwrap_or_default();
    let decode_rlic = decode(rlic.as_bytes()).unwrap();
    //println!("Base64解码：{:?}", decode_rlic);
    let decode_rlic_str = String::from_utf8(decode_rlic).unwrap();
    //println!("许可证: {}", decode_rlic_str);
    let split_date: Vec<&str> = decode_rlic_str.split("-").collect();
    //println!("{:?}", split_date);
    let last_item = split_date.last().unwrap();
    //println!("{}", last_item);
    let (head, tail) = last_item.split_at(last_item.len() - 1);
    let last_item_without_last_char = head.to_string();
    //println!("去掉回车换行：{}",last_item_without_last_char);
    let i: i64 = last_item_without_last_char.parse().unwrap();
    //println!("i: {}", i);
    let date = NaiveDateTime::from_timestamp(i, 0);
    //println!("Date: {}", date);

    //let now = Local::now();
    //let timestamp = now.timestamp();
    //println!("Current timestamp: {}", timestamp);

    header_adapter.set_ldays(date.to_string().into());
}
