# gm_fontsx
Check what fonts are installed and available on your system.

### Compiling
- Open a terminal
- Install **cargo** if you dont have it (on Windows => https://win.rustup.rs) (on Linux/Macos => curl https://sh.rustup.rs -sSf | sh)
- Get [git](https://git-scm.com/downloads) or download the archive for the repository directly
- `git clone https://github.com/Earu/gm_zip` (ignore this if you've downloaded the archive)
- Run `cd gm_zip`
- `cargo build`
- Go in `target/debug` and rename the binary according to your branch and realm (gmsv_zip_win64.dll, gmcl_zip_win64.dll, gmsv_zip_linux.dll, gmcl_zip_linux.dll, gmcl_zip_osx64.dll)
- Put the binary in your gmod `lua/bin` directory

*Note: Even on other platforms than Windows the extension of your modules **needs** to be **.dll***
