// Copyright 2019 Kodebox, Inc.
// This file is part of CodeChain.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#[macro_use]
extern crate clap;
extern crate futures;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate serde;
extern crate serde_json;

mod config;
mod error;
mod filter;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::{Ipv4Addr, SocketAddrV4};

use futures::Future;
use hyper::Server;

use self::config::Config;
use self::error::Error;
use self::filter::Filter;

fn main() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    pretty_env_logger::init();

    let yaml = load_yaml!("clap.yml");
    let args = clap::App::from_yaml(yaml).version(VERSION).get_matches();
    let bind = value_t_or_exit!(args.value_of("bind"), Ipv4Addr);
    let port = value_t_or_exit!(args, "port", u16);
    let forward = value_t_or_exit!(args, "forward", String).parse().unwrap();
    let allowed_list = args.value_of("allowed_list").unwrap();
    let mut allowed_rpcs = BufReader::new(File::open(allowed_list).unwrap())
        .lines()
        .map(|line| line.map(|line| line.trim().to_string()))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    allowed_rpcs.sort_unstable();

    let bind_addr = SocketAddrV4::new(bind, port).into();

    let config = Config::new(forward, allowed_rpcs);
    let server = Server::bind(&bind_addr)
        .serve(config)
        .map_err(|e| println!("{:?}", e));

    hyper::rt::run(server);
}
