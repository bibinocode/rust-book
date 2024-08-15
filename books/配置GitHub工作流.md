# 配置GitHub工作流


项目根目录创建 `.github/workflows/build.yml` 文件，并定义jobs

```yaml
name: build # 工作流名称

on: # 监听器
  push: # 监听push事件
    branches: # 监听分支
      - master
    tags: # 监听tag
      - v*
  pull_request: # 监听pull_request事件
    branches: # 监听分支
      - master

permissions: # 权限
  contents: write

jobs: # 任务
  build-rust: # 编译rust代码
    strategy: # 策略
      matrix: # 矩阵
        platform: [ubuntu-latest] # 平台检查是否 运行在ubuntu上
    runs-on: ${{ matrix.platform }} # 运行在平台上
    steps: # 步骤
      - uses: actions/checkout@v4 # 克隆代码
        with: # 克隆参数
          fetch-depth: 0 # 克隆全部历史记录
          submodules: recursive # 克隆子模块
      - name: Install Rust # 步骤名
        run: rustup toolchain install stable --component llvm-tools-preview # 执行命令
      - name: Install cargo-llvm-cov 
        uses: taiki-e/install-action@cargo-llvm-cov # 安装 cargo-llvm-cov 检查
      - name: install nextest
        uses: taiki-e/install-action@nextest # 安装 nextest 运行测试
      - uses: Swatinem/rust-cache@v2 # 上一次成功构建的缓存rust依赖 加速构建
      - name: Check code format 
        run: cargo fmt -- --check # 检查代码格式
      - name: Check the package for errors
        run: cargo check --all # 检查包是否有错误
      - name: Lint rust sources
        run: cargo clippy --all-targets --all-features --tests --benches -- -D warnings  # 运行 clippy 检查代码
      - name: Execute rust tests  
        run: cargo nextest run --all-features  # 运行测试
      - name: Generate a changelog
        uses: orhun/git-cliff-action@v2 #  如果tag  生成 changelog
        id: git-cliff # 步骤id
        if: startsWith(github.ref, 'refs/tags/')  # 条件
        with:
          config: cliff.toml # 配置文件
          args: -vv --latest --strip header # 参数
        env:
          OUTPUT: CHANGES.md # 输出文件名
      - name: Release
        uses: softprops/action-gh-release@v1  # 发布
        if: startsWith(github.ref, 'refs/tags/') # 条件
        with:
          body: ${{ steps.git-cliff.outputs.content }}  # 发布内容

```
