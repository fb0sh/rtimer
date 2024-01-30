# rtimer
```bash
Usage: rtimer.exe [OPTIONS] [PATTERN]

Arguments:
  [PATTERN]  !1h:1m:30s  !12h:1m:30s  !1h:1m  !1m:30s  !1h:30s  !1h  !2m  !3s  @12:33

Options:
  -c, --cmd <CMD>  till the end, execute a system script or command
  -h, --help       Print help
  -V, --version    Print version
```


```rust api
pub use util::exec_shell;
pub use util::play_sound;
pub use util::select_pattern;
pub use util::Cli;

use rtimer::exec_shell;
use rtimer::play_sound;
use rtimer::select_pattern;
use rtimer::Cli;

```