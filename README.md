# Smart Garden Controller

A weekend project to learn Rust and save home garden while I am on vacation

## Overview

### Hardware:
- Raspberry Pi 4
- Relay Module
- Water Pump


### Software:
- Scheduler Thread - controls periodic tasks
- Worker Thread - controls water pump and soil moisture sensors
- Telegram Bot - API to control garden remotely using a telegram bot commands


### Periodic Tasks:
- Water Plants
- Read Sensor Data

### Telegram Commands:

- /help — Help
- /waterplants — Water Plants
- /listtasks — List Periodic Tasks
- /removetask — Removes Periodic Task
- /updatetask — Update Periodic Task Schedule
- /resettasks — Reset all tasks to default config
- /shutdown — Shutdowns the system


## How to install?

- setup raspberry pi: https://projects.raspberrypi.org/en/projects/raspberry-pi-setting-up
- ssh into Pi
- install rust https://www.rust-lang.org/tools/install
- clone this project
- create .env file and set `TELOXIDE_TOKEN` variable: https://github.com/teloxide/teloxide
- cd into project folder and run `cargo build`


## How to run?
- complete installation instruction
- cd into project folder and run `cargo run build`


