# Downloader
## Running
```bash
./downloader <Link> <Filename>
```
## Info
#### Debug build size: `56.2`MiB (`57,508`KiB & `58,887,592`Bytes)
#### Release build size: `7.8`MiB (`8,034`KiB & `8,226,264`Bytes) 
#### opt-level = "z" : release build size: `8.2`MiB (`8,414`KiB & `8,615,680`Bytes)
#### opt-level = "s" : release build size: `7.9`MiB (`8,106`KiB & `8,300,200`Bytes)
#### opt-level = "z" lto = true : release build size: `5.0`MiB (`5,075`KiB & `5,195,952`Bytes)
#### opt-level = "s" lto = true : release build size: `4.8`MiB (`4,965`KiB & `5,083,424`Bytes)
#### opt-level = "z" lto = true codegen-units = 1 : release build size: `4.3`MiB (`4,420`KiB & `4,525,872`Bytes)
#### opt-level = "s" lto = true codegen-units = 1 : release build size: `4.2`MiB (`4,330`KiB & `4,433,000`Bytes)
#### opt-level = "z" lto = true codegen-units = 1 panic = "abort" : release build size: `3.9`MiB (`3,989`KiB & `4,083,896`Bytes)
#### opt-level = "s" lto = true codegen-units = 1 panic = "abort" : release build size: `3.8`MiB (`3,874`KiB & `3,966,560`Bytes)
#### opt-level = "z" lto = true codegen-units = 1 panic = "abort" stripped : release build size: `2.0`MiB (`2,032`KiB & `2,080,312`Bytes)
#### opt-level = "s" lto = true codegen-units = 1 panic = "abort" stripped : release build size: `2.0`MiB (`2,044`KiB & `2,092,600`Bytes)
#### opt-level = "z" lto = true codegen-units = 1 panic = "abort" stripped upx : release build size: `753.8`KiB (`771,852`Bytes)
#### opt-level = "s" lto = true codegen-units = 1 panic = "abort" stripped upx : release build size: `765.6` KiB (`783,940`Bytes)
