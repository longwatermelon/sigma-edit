# sigma-edit
Automatically generated sigma edits. These are not meant to be serious.

Requires [ffmpeg](https://ffmpeg.org) to combine audio and video together.

# Usage
```
mkdir out
cargo r [playlist|number of output shorts]
```

Configuration options
* `rig-ties`: `true` to guarantee ties in character vs character videos, `false` for otherwise. Defaults to `false`.
* `probability`: A value of > 0.5 will give the first character a higher chance of winning, and a value of < 0.5 gives the second character a higher chance of winning. Defaults to 0.5.
