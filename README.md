# gm_fontsx
Since apparently we're never getting a proper way to check for installed fonts on Garry's Mod, this has to exist 😒

### Usage
```lua
require("fontsx")

-- creates a gmod virtual font depending on whether the system font "Consolas" is available
surface.CreateFont("my_virtual_font", {
	font = fonts.Exists("Consolas") and "Consolas" or "Roboto",
	size = 15,
})
```

```lua
require("fontsx")

-- prints all system fonts currently available
PrintTable(fonts.GetAll())
```

### Compiling
- Open a terminal
- Install **cargo** if you dont have it (on Windows => https://win.rustup.rs) (on Linux/Macos => curl https://sh.rustup.rs -sSf | sh)
- Get [git](https://git-scm.com/downloads) or download the archive for the repository directly
- `git clone https://github.com/Earu/gm_fontsx` (ignore this if you've downloaded the archive)
- Run `cd gm_fontsx`
- `cargo build`
- Go in `target/debug` and rename the binary according to your branch and realm (gmsv_fontsx_win64.dll, gmcl_fontsx_win64.dll, gmsv_fontsx_linux.dll, gmcl_fontsx_linux.dll, gmcl_fontsx_osx64.dll)
- Put the binary in your gmod `lua/bin` directory

*Note: Even on other platforms than Windows the extension of your modules **needs** to be **.dll***
