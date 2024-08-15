# Rust 快速扫盲

## 环境设置

### 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 安装 VSCode 插件

- crates: Rust 包管理
- Even Better TOML: TOML 文件支持
- Better Comments: 优化注释显示
- Error Lens: 错误提示优化
- GitLens: Git 增强
- Github Copilot: 代码提示
- indent-rainbow: 缩进显示优化
- Prettier - Code formatter: 代码格式化
- REST client: REST API 调试
- rust-analyzer: Rust 语言支持
- Rust Test lens: Rust 测试支持
- Rust Test Explorer: Rust 测试概览
- TODO Highlight: TODO 高亮
- vscode-icons: 图标优化
- YAML: YAML 文件支持



### 包管理工具 cargo

cargo 是 Rust 包管理工具，我们可以用它来安装 Rust 包。
包查找网址地址：https://crates.io/


### 安装 cargo generate

cargo generate 是一个用于生成项目模板的工具。它可以使用已有的 github repo 作为模版生成新的项目。

```bash
cargo install cargo-generate
```

在我们的课程中，新的项目会使用 `tyr-rust-bootcamp/template` 模版生成基本的代码：

```bash
cargo generate tyr-rust-bootcamp/template
```
这样会自动去Github找模版然后生成基本的代码结构。
做了什么：
1. 下载了仓库代码
2. 然后编译代码
3. 然后在本机某个特定的目录生成二进制文件

### 安装 pre-commit

pre-commit 是一个代码检查工具，可以在提交代码前进行代码检查。

```bash
pip install pre-commit
```

用`pip` 安装到全局，如果不喜欢全局使用`pipx`安装到虚拟环境

在项目创建`.pre-commit-config.yaml` 配置：
```yaml
# .pre-commit-config.yaml
fail_fast: false
repos:
  - repo: https://github.com/pre-commit/pre-commit-hooks
    rev: v4.3.0
    hooks:
      - id: check-byte-order-marker
      - id: check-case-conflict
      - id: check-merge-conflict
      - id: check-symlinks
      - id: check-yaml
      - id: end-of-file-fixer
      - id: mixed-line-ending
      - id: trailing-whitespace
  - repo: https://github.com/psf/black
    rev: 22.10.0
    hooks:
      - id: black
  - repo: local
    hooks:
      - id: cargo-fmt
        name: cargo fmt
        description: Format files with rustfmt.
        entry: bash -c 'cargo fmt -- --check'
        language: rust
        files: \.rs$
        args: []
      - id: cargo-deny
        name: cargo deny check
        description: Check cargo dependencies
        entry: bash -c 'cargo deny check -d'
        language: rust
        files: \.rs$
        args: []
      - id: typos
        name: typos
        description: check typo
        entry: bash -c 'typos'
        language: rust
        files: \.*$
        pass_filenames: false
      - id: cargo-check
        name: cargo check
        description: Check the package for errors.
        entry: bash -c 'cargo check --all'
        language: rust
        files: \.rs$
        pass_filenames: false
      - id: cargo-clippy
        name: cargo clippy
        description: Lint rust sources
        entry: bash -c 'cargo clippy --all-targets --all-features --tests --benches -- -D warnings'
        language: rust
        files: \.rs$
        pass_filenames: false
      - id: cargo-test
        name: cargo test
        description: unit test for the project
        entry: bash -c 'cargo nextest run --all-features'
        language: rust
        files: \.rs$
        pass_filenames: false

```

安装成功后运行在项目运行 `pre-commit install` 即可。


### 安装 Cargo deny

Cargo deny 是一个 Cargo 插件，可以用于检查依赖的安全性。用来检查协议是否安全，是否可商用等。

```bash
cargo install --locked cargo-deny
```


### 安装 typos

typos 是一个拼写检查工具。英文单词拼写检查，可以用来检查代码中的拼写错误。

```bash
cargo install typos-cli
```

### 安装 git cliff

git cliff 是一个生成 changelog 的工具。

```bash
cargo install git-cliff
```

### 安装 cargo nextest

cargo nextest 是一个 Rust 增强测试工具。

```bash
cargo install cargo-nextest --locked
```


## 解析项目


### 项目结构

```shell
📦rcli 
 ┣ 📂.github # Github相关配置
 ┃ ┗ 📂workflows
 ┃ ┃ ┗ 📜build.yml
 ┣ 📂assets # 资源文件
 ┃ ┣ 📜README.md
 ┃ ┗ 📜juventus.csv
 ┣ 📂src # 项目源码文件
 ┃ ┣ 📜main.rs # 可执行的项目文件
 ┣ 📜.gitignore
 ┣ 📜target # 执行 cargo build 后产生的文件夹
 ┣ 📜.pre-commit-config.yaml # pre-commit 配置文件
 ┣ 📜CHANGELOG.md # changelog 文件
 ┣ 📜Cargo.lock # cargo 依赖锁定文件
 ┣ 📜Cargo.toml # cargo 配置文件
 ┣ 📜README.md  # 项目说明文件
 ┣ 📜_typos.toml # typos 配置文件
 ┣ 📜cliff.toml # git cliff 配置文件
 ┣ 📜deny.toml # cargo deny 配置文件
 ┗ 📜output.json # cargo nextest 输出文件
```



### build 项目

运行 `cargo build` 编译项目，会在 `target` 文件夹下生成可执行文件。（debug 模式下会生成 debug 文件，release 模式下会生成 release 文件） 

- `cargo build` 默认是 debug 模式，编译速度快，适合开发阶段。
- `cargo build --release` 编译 release 模式，编译速度慢，适合生产环境。

- 可以通过配置 `~/.cargo/config` 文件来指定默认输出路径 例如 全部target放到用户目录下

    ```bat
    [build]
    target-dir = "C:/Users/ab/.cargo/target"
    ```

    
### 运行项目

运行 `cargo run` 运行项目，会在 `target` 文件夹下生成可执行文件，并运行。