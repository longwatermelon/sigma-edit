# sigma-edit
Automatically generated sigma edits. These are not meant to be serious.

Requires [ffmpeg](https://ffmpeg.org) to combine audio and video together.

<details>
<summary>Sigma edit example (audio on)</summary>
  
https://github.com/longwatermelon/sigma-edit/assets/73869536/dc671bb2-bdb9-4c11-a93b-5f0870bf27ab
</details>

<details>
<summary>Character vs character example (audio on)</summary>

https://github.com/longwatermelon/sigma-edit/assets/73869536/a535aa70-1ee6-4d5d-b5ba-730f23f7e367
</details>

# Usage
```
mkdir output
cargo r [number of output videos, default is 1]
```

Configuration options
* `rig-ties`: `true` to guarantee ties in character vs character videos, `false` for otherwise
