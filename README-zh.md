# YXY
YXY(易校园) 平台爬虫库，使用 rust 开发。
> 👏🏻欢迎对本仓库提供宝贵的贡献，比如通过 Issue 和 PR。

[![crates.io](https://img.shields.io/crates/v/yxy.svg)](https://crates.io/crates/yxy)
[![CI](https://github.com/I-Info/yxy/actions/workflows/ci.yml/badge.svg)](https://github.com/I-Info/yxy/actions/workflows/ci.yml)

## 如何运行
1. 准备好 `Rust` 开发环境

2. 克隆当前仓库
    ``` bash
    git clone https://github.com/I-Info/yxy.git
    ```

3. 更改当前工作目录
    ```bash
    cd yxy
    ```

4. 创建和修改 `conf.yaml` 配置文件 (样例在 `conf.example.yaml`)
    
5. 通过 Cargo 编译运行
    ``` bash
    cargo run
    ```


## 功能
> tips: 务必先注册好YXY APP再使用此程序进行相关查询

> 下文使用 `yxy` 代表主程序

1. 使用配置文件进行查询
   - `conf.yaml` 位于当前工作目录下
        ``` bash
        ./yxy
        ```

    - 位于其他位置
        ``` bash
        ./yxy -c <PATH>
        ```

2. 其他查询
    1. UID
        ``` bash
        ./yxy query uid <phone number>
        ```

    2. Electricity 
        > 直接通过参数中 UID 查询，不使用配置文件
        ``` bash
        ./yxy query ele <UID>
        ```

## 声明
仅供学习交流，严禁用于商业用途