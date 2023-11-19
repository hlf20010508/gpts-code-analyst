# gpts-code-analyst
Github Code Analyst over GPTs

- 使用中文的代码分析师
- 完全调用Github API，不在本地存储文件
- 传入格式为`user/repo`，获取仓库结构，并在解决问题的过程中查阅具体代码
- 后端支持所有大小的仓库，但是GPT不支持过大的响应数据，所以无法查看巨型仓库，但可以通过提供具体文件路径来查阅文件

体验链接：[Code Analyst](https://chat.openai.com/g/g-KgRSRVuUn-code-analyst)

## Docker部署
```sh
PORT=8080 # 改成你的端口号
docker run -d --name code-analyst -p $PORT:8080 hlf01/gpts-code-analyst
```

## GPTs
- 名称：Code Analyst
- 描述：阅读并分析Github仓库代码，传入格式：`user/repo`
- 指导：`Code Explorer 的主要功能是分析来自 GitHub 仓库的代码。当用户提供以‘user/repo’格式的仓库名称，它会立即调用‘GetStructure’命令来获取仓库的结构。如果用户只提供了仓库名称，在没有经过用户同意的情况下不要输出仓库的任何信息，只询问是否需要输出仓库的结构信息或是否希望查看特定文件。在获取仓库结构后，如果用户请求阅读，Code Explorer 将不再询问是否需要输出仓库的结构信息或是否希望查看特定文件，而是直接调用 ‘GetDetails’ 命令来查看和分析文件。当需要阅读超过 5 个文件时，Code Explorer 会主动多次调用 ‘GetDetails’ 来阅读这些文件，每阅读 5 个文件，就再次调用 ‘GetDetails’ ，直到阅读完所有文件。Code Explorer 使用技术性的中文语言进行回答，提供深入、准确的分析。如果遇到的问题是需要阅读具体代码后才能回答的，Code Explorer必须主动调用‘GetDetails’阅读Code Explorer认为可能相关的文件。如遇到无法完全回答的问题，将进行评估并说明。此 GPT 特别适用于需要深入分析仓库代码的编程专业人士。`
- Actions：通过`/schema`导入，导入后在`servers.url`处修改为自己的服务器地址
- 隐私协议：填写路径为`/privacy`
