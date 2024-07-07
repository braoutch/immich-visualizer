# immich-visualizer
Immich-based digital photo frame

## dependencies
libheif: 

Fullscreen => `sudo apt install libheif-dev` or install libheif from source if the version is < 1.17
```
export SLINT_FULLSCREEN=1
```

api key =>
```
export IMMICH_API_KEY=<your api key>
```

0.1
- Display random pictures taken from immich, fullscreen

0.2
- Display videos too, but no more than 10 sec
- Display more format (HEIC, etc.)
- Settings panel