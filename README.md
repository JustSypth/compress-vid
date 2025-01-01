# Compress-vid
### An ffmpeg wrapper for compressing videos with more ease.

## Installation:
1. Download the latest release: [Download](https://github.com/JustSypth/compress-vid/releases/latest/download/compress-vid.tar)
2. Extract the archive
3. Run `sudo ./install.sh` inside the extracted archive.

## Usage:
### Example usage:
Without specifying any arguments (defaults will be applied):
```
compress-vid [video]
```
With arguments:
```
compress-vid --crf 28 --preset medium [video]
```


### Arguments:

**-p / --preset:**  
    Controls the speed of video encoding. (ultrafast, veryfast, medium, slow, veryslow)  
    Slower presets produce better quality and smaller file sizes. Faster presets are quicker but result in larger files with lower quality.

**-c / --crf:**  
    The CRF (Constant Rate Factor) controls video quality. <small>*(Allowed values: 0-51)*</small>  
    Lower values output higher quality and larger file sizes. Higher values output lower quality and smaller file sizes.


**--debug**  
**--version**  
**--help**
