# excel2flatbuffers-rs
convert excel to flatbuffers table and pack excel data to binary file, for game development.

1. 根据Excel表头生成Flatbuffers的fbs文件(目前支持int,float,long,string)
2. 将Excel中的数据使用flatbuffers的结构，打包成二进制文件
3. 生成目标语言代码

> excel 的结构的解析，可以根据自己的需要去修改解析代码。flatc.exe 是必须的，放到工程根目录。在最终编译出 exe 后，放到和 exe 放一起。

### 使用

`excel2flatbuffers-rs.exe -l csharp -e ./excels/ -b ./generated/bytes/ -c ./generated/code/ -f ./generated/fbs/ `

> 注意路径的后面要加上 `/`

#### Rust 真的好快！哈哈哈哈