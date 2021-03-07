/*
 * i3blocks-cpu - A CPU block for i3blocks
 * Copyright (c) 2020  Michael Sasser <Michael@MichaelSasser.org>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
#![crate_name = "i3blocks_cpu"]

use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::Sub;
use std::{thread, time};

#[derive(Clone)]
struct CpuData {
    ctime: f32,
    cidle: f32,
}

impl Sub for CpuData {
    type Output = CpuData;

    fn sub(self, other: CpuData) -> CpuData {
        CpuData {
            ctime: self.ctime - other.ctime,
            cidle: self.cidle - other.cidle,
        }
    }
}

impl Default for CpuData {
    fn default() -> CpuData {
        CpuData {
            cidle: 0.0,
            ctime: 0.0,
        }
    }
}

/// Get the CPU temperature.
fn get_cpu_temp() -> f32 {
    let mut temp: f32 = 0.0;

    let directory = "/sys/devices/platform/coretemp.0/hwmon/";
    if let Ok(dirs) = fs::read_dir(directory) {
        for dir in dirs {
            if let Ok(dir) = dir {
                if let Ok(entries) = fs::read_dir(dir.path()) {
                    for entry in entries {
                        // `entry` is a `DirEntry`.
                        if let Ok(entry) = entry {
                            // println!("{:?}", entry.file_name());
                            let filename = match entry.file_name().into_string() {
                                Ok(f) => f,
                                Err(_) => String::from(""),
                            };
                            let filepath = match dir.file_name().into_string() {
                                Ok(f) => f,
                                Err(_) => String::from(""),
                            };
                            // println!("{:?}, {:?}, {:?}", directory, filepath, filename);
                            if filename.ends_with("input") {
                                let data = fs::read_to_string(
                                    format!("{}{}/{}", directory, filepath, filename).as_str(),
                                )
                                .expect("Unable to read from /sys/devices/platform/...");
                                let value =
                                    data.as_str().trim().parse::<f32>().expect("0.0") / 1000.0;
                                if temp < value {
                                    temp = value;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    temp
}

/// Get the CPU load in percent.
///
/// The load is calculated like:
/// https://github.com/Leo-G/DevopsWiki/wiki/How-Linux-CPU-Usage-Time-and-Percentage-is-calculated
fn get_cpu_load() -> CpuData {
    let file = match fs::File::open("/proc/stat") {
        Ok(file) => file,
        Err(_) => panic!("Unable to read from /proc/stat"),
    };
    let mut buffer = BufReader::new(file);
    let mut cpu_line = String::new();

    buffer
        .read_line(&mut cpu_line)
        .expect("Unable to read line.");

    /*
     * user nice system idle iowait  irq  softirq steal guest guest_nice
     * 2    3    4      5    6       7    8       9     10    11
     */
    let tokens: Vec<&str> = cpu_line.split(" ").collect();
    let user = tokens[2].parse::<i32>().expect("0");
    let nice = tokens[3].parse::<i32>().expect("0");
    let system = tokens[4].parse::<i32>().expect("0");
    let idle = tokens[5].parse::<i32>().expect("0");
    let iowait = tokens[6].parse::<i32>().expect("0");
    let irq = tokens[7].parse::<i32>().expect("0");
    let softirq = tokens[8].parse::<i32>().expect("0");
    let steal = tokens[9].parse::<i32>().expect("0");
    let cpu = CpuData {
        ctime: (user + nice + system + idle + iowait + irq + softirq + steal) as f32,
        cidle: (idle + iowait) as f32,
    };
    return cpu;
}

fn main() -> std::io::Result<()> {
    let mut old_load: CpuData = CpuData::default();
    loop {
        let temp = get_cpu_temp();
        let load = get_cpu_load();
        let cpu = load.clone() - old_load;
        let cpu_persentage = (cpu.ctime - cpu.cidle) / cpu.ctime * 100.0;

        // println!("'{:?}'", tokens);
        println!("{: >6.2}% {}°C", cpu_persentage, temp);
        old_load = load;
        thread::sleep(time::Duration::from_secs(1));
    }
}
