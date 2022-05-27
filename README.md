# YXY
YXY(YSchool) platform spider library, written in rust.

[![crates.io](https://img.shields.io/crates/v/yxy.svg)](https://crates.io/crates/yxy)
[![CI](https://github.com/I-Info/yxy/actions/workflows/ci.yml/badge.svg)](https://github.com/I-Info/yxy/actions/workflows/ci.yml)

[中文说明文档](https://github.com/I-Info/yxy/blob/main/README-zh.md)
## WIP
- [x] Query electricity balance.
- [x] Simulate APP login.
- [x] Subscribe some balance status.
  - Email ❌
  - WeChat(ServerChan) ✅
- [ ] Automatic electricity bill payment.
- [ ] More query or features...

## How to run
1. Prepare `Rust` development environment. 

2. Clone the repo
    ``` bash
    git clone https://github.com/I-Info/yxy.git
    ```

3. Change the working directory
    ```bash
    cd yxy
    ```

4. Edit the `conf.yaml` (example in `conf.example.yaml`)
    
5. Compile & Run by Cargo
    ``` bash
    cargo run
    ```

## Features
> The following uses `yxy` to represent the main program

1. Query electricity by conf
   - `conf.yaml` file is located in the current working directory
        ``` bash
        ./yxy
        ```

    - Or in other place
        ``` bash
        ./yxy -c <PATH>
        ```

2. Other Queries
    1. UID
        > Get UID by simulating app login, so you need to register yxy app account first.
        ``` bash
        ./yxy query uid <phone number>
        ```

    2. Electricity 
        > (Simply query by UID without config file)
        ``` bash
        ./yxy query ele <UID>
        ```

## Disclaimer
For learning only, do not use for commercial purposes.