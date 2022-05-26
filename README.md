# YXY
YXY(YSchool) platform spider library, written in rust.

[![CI](https://github.com/I-Info/yxy/actions/workflows/ci.yml/badge.svg)](https://github.com/I-Info/yxy/actions/workflows/ci.yml)

## WIP
- [x] Query electricity balance.
- [x] Simulate APP login.
- [ ] Subscribe some balance status.
  - Email
  - WeChat
- [ ] Automatic electricity bill payment.
- [ ] More query or functions...

## How to run
1. Prepare `Rust` development environment. 

2. Clone the repo
    ``` bash
    $ git clone https://github.com/I-Info/yxy.git
    ```

3. Change the working directory
    ```bash
    $ cd yxy
    ```

4. Edit the `conf.yaml` (example in `conf.example.yaml`)
    
5. Compile & Run by Cargo
    ``` bash
    $ cargo run
    ```
