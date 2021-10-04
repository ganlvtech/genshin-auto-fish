# 原神半自动钓鱼

需要手动更换鱼饵和选择位置抛竿，鱼上钩时，程序会自动收杆和调整力度。

目前代码里写死的图形匹配逻辑，仅支持 1920x1080 全屏。

您可以可以随意使用代码和程序，您使用此程序对账号造成的一切后果请自行承担。

由于原神是使用管理员运行的，本程序**必须使用管理员权限**才能模拟点击。

## 原理说明

使用 BitBlt 和 GetDIBits 获取屏幕颜色信息，使用纯 CPU 进行特征识别。

识别 `<` `|` `>` 和收杆图标，前三个使用 `#ffffc0` 纯色进行查找，最后一个使用几个手动选择的特征点的灰度值比较差值进行匹配。

有力度条时，控制力度，在区间左侧 1/3 处徘徊。没有找到力度条时，会查找收杆图标，如果发现收杆图标，就点击左键收杆。

使用 SendInput 模拟鼠标点击

## 构建

```batch
cd bindings
cargo build
cp .\target\debug\bindings-eeb6656161a2ef74\out\windows.rs .\src\lib.rs
cd ..

cargo build --release

mt.exe -nologo -manifest "D:\Rust\genshin-auto-fish\genshin-auto-fish.exe.manifest" -outputresource:"D:\Rust\genshin-auto-fish\target\release\genshin-auto-fish.exe;#1"
```
