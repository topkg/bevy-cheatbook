# 翻译

## 本地运行

```shell
# 先安装markdown渲染工具: mdbook, 也是rust官方推出的,适合写书写博客.
cargo install mdbook

# 将~/.cargo/bin添加到Path环境变量, 自行百度

# 在仓库根目录,运行mdbook,使用本地浏览器打开 http://localhost:3000
mdbook serve
```

## 翻译规则

1. 为了避免原始仓库同步到当前仓库导致的大量冲突, 翻译时尽量大块大块地翻译.
2. mdbook类似的工具都倾向组合策略,可复用的部分都会被提取为单独的物料,翻译时需要找到具体的物料.
