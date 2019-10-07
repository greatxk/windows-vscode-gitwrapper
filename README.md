# windows-vscode-gitwrapper
**在VS Code中使用MSYS2环境中的git**

1.编译完成后，将windows-vscode-gitwrapper.exe复制到MSYS2_PATH/usr/bin文件夹中。

2.在VS Code中设置：

```JSON
// settings.json
{
    "git.path": "MSYS2_PATH/usr/bin/windows-vscode-gitwrapper.exe"
}
```
