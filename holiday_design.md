
假期数据放在github上，采用如下的方式存放，程序启动会首先使用本地缓存，然后去更新meta.json 文件，对比本地的，看看是否需要更新。

假期的数据放在resources的hlidays目录下:

``` json
holidays/
├── meta.json
├── 2025/
│   ├── cn_zh.json
│   ├── cn_en.json
│   └── fr_fr.json
└── 2026/
    └── ...
```

meta.json 的格式如下：

``` json
{
  "last_updated": "2025-01-20T12:00:00Z",
  "files": {
    "2025_cn_zh": {
      "last_modified": "2025-01-18T08:00:00Z",
      "sha256": "abc123..."
    },
    "2025_cn_en": {
      "last_modified": "2025-01-17T09:00:00Z",
      "sha256": "def456..."
    }
  }
}

```

程序启动的时候，会在 xdg的cache_dir 目录的holidays 目录寻找 meta_cache.json 文件，文件的格式如下:

``` json
{
  "data"": "github 上的 meta json 的内容",
  "cache_time": "2025-01-18T08:00:00Z"
}
```

缓存的有效时间为1天.

meta_cache 更新的逻辑如下：

``` Mermaid
flowchart TD
    A[开始] --> B{判断 meta_cache.json 是否存在}
    B -- 是 --> C{判断时间是否过期}
    B -- 否 --> D[去下载 meta.josn 文件]

    D --> E{是否下载成功}
   E -- 是 --> H[保存meta_cache.json文件]
   H --> I
    E -- 否 --> G{meta_cache.json 是否存在}
    C -- 是 --> D
    C -- 否 --> I[更新日历数据]
    G -- 是 --> I
    G -- 否 --> F[结束]
```
