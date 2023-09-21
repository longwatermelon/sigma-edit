# sigma-edit
Automatically generated sigma edits. These are not meant to be serious.

Requires [ffmpeg](https://ffmpeg.org) to combine audio and video together.

# Usage
```
mkdir out
cargo r # Generate 1 random short
cargo r 5 # Generate 5 random shorts
cargo r type edit # Generate edit short. Options: edit, comparison, month, wallpaper
cargo r playlist # Generate playlist of random songs.
```

Configuration options
* `rig-ties`: `true` to guarantee ties in character vs character videos, `false` for otherwise. Defaults to `false`.
* `probability`: A value of > 0.5 will give the first character a higher chance of winning, and a value of < 0.5 gives the second character a higher chance of winning. Defaults to 0.5.
* `slow`: Slow clips down in edit shorts. Usually doesn't work very well, so it's not recommended to use it. Defaults to `false`.
