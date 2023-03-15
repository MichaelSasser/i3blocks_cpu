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

use anyhow::Result;
use std::ops::Sub;
use std::time::Duration;
use tokio::fs::{read_dir, File};
use tokio::io::{AsyncReadExt, AsyncSeekExt};

const CPU_TEMPERATURE_DIRECTORY: &str = "/sys/devices/platform/coretemp.0/hwmon/";
const CPU_LOAD_FILE: &str = "/proc/stat";

#[derive(Clone, Copy, Default)]
struct CpuLoadData {
    ctime: f32,
    cidle: f32,
}

impl Sub for CpuLoadData {
    type Output = CpuLoadData;

    fn sub(self, other: CpuLoadData) -> CpuLoadData {
        CpuLoadData {
            ctime: self.ctime - other.ctime,
            cidle: self.cidle - other.cidle,
        }
    }
}

impl CpuLoadData {
    fn replace(&mut self, other: Self) -> Self {
        let buf = *self;
        *self = other;
        buf
    }
}

struct CpuLoad {
    file: File,
    buf: String,
    data: CpuLoadData,
    prev_data: CpuLoadData,
}

impl CpuLoad {
    async fn new(path: &str) -> Self {
        let file = File::open(path)
            .await
            .unwrap_or_else(|_| panic!("Unable to read from {}", &path));

        Self {
            file,
            buf: String::new(),
            data: CpuLoadData {
                ctime: 0_f32,
                cidle: 0_f32,
            },
            prev_data: CpuLoadData {
                ctime: 0_f32,
                cidle: 0_f32,
            },
        }
    }

    async fn read_data(&mut self) -> Result<()> {
        // let mut buffer = BufReader::new(file);

        self.file.read_to_string(&mut self.buf).await?;

        /*
         *       user nice system idle iowait  irq  softirq steal guest guest_nice
         * file  2    3    4      5    6       7    8       9     10    11
         * here  0    1    2      3    4       5    6       7     8     9
         */
        let tokens = self
            .buf
            .split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .take(10)
            .skip(2)
            .map(|s| s.parse::<i32>().expect("0"))
            .collect::<Vec<i32>>();

        self.prev_data = self.data.replace(CpuLoadData {
            ctime: (tokens.iter().sum::<i32>()) as f32,
            cidle: (tokens[3] + tokens[4]) as f32,
        });

        // Cleanup
        self.buf.clear();
        self.file.rewind().await?;

        Ok(())
    }

    fn in_percent(&self) -> f32 {
        let data = self.data - self.prev_data;

        (data.ctime - data.cidle) / data.ctime * 100_f32
    }
}

struct CpuTemperature {
    files: Vec<File>,
    buf: String,
    data: f32,
}

impl CpuTemperature {
    /// Get the CPU temperature.
    async fn find_files(path: &str) -> Self {
        let mut files: Vec<File> = Vec::new();
        if let Ok(mut dirs) = read_dir(path).await {
            while let Ok(dir) = dirs.next_entry().await {
                if let Some(dir) = dir {
                    if let Ok(mut entries) = read_dir(dir.path()).await {
                        while let Ok(entry) = entries.next_entry().await {
                            if let Some(entry) = entry {
                                let filename =
                                    entry.file_name().into_string().unwrap_or_else(|_| {
                                        panic!("Unable to read filename from entry {:?}", entry)
                                    });
                                let filepath = dir.file_name().into_string().unwrap_or_else(|_| {
                                    panic!("Unable to read file_path from dir {:?}", dir)
                                });
                                if filename.ends_with("input") {
                                    let path = format!(
                                        "{}{}/{}",
                                        CPU_TEMPERATURE_DIRECTORY, filepath, filename
                                    );
                                    files.push(File::open(&path).await.unwrap_or_else(|_| {
                                        panic!("Unable to read from {}", &path)
                                    }));
                                }
                            } else {
                                break;
                            }
                        }
                    }
                } else {
                    break;
                }
            }
        }
        Self {
            data: 0_f32,
            files,
            buf: String::new(),
        }
    }

    async fn get_cpu_temp(&mut self) -> Result<()> {
        for file in self.files.iter_mut() {
            self.data = 0_f32;
            file.read_to_string(&mut self.buf).await?;

            let value = self.buf.as_str().trim().parse::<f32>()? / 1000.0;

            // Choose max
            if self.data < value {
                self.data = value;
            }

            // Cleanup
            self.buf.clear();
            file.rewind().await?;
        }
        Ok(())
    }

    fn in_degree_c(&self) -> f32 {
        self.data
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let mut cpu = CpuLoad::new(CPU_LOAD_FILE).await;
    let mut temperature = CpuTemperature::find_files(CPU_TEMPERATURE_DIRECTORY).await;
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    loop {
        interval.tick().await;
        (_, _) = tokio::join!(temperature.get_cpu_temp(), cpu.read_data());

        println!(
            "{: >6.2}% {}°C",
            cpu.in_percent(),
            temperature.in_degree_c()
        );
    }
}
