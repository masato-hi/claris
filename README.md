# claris
Generate an image from Yaml.

## Install
```
cargo install --git https://github.com/masato-hi/claris.git
```

## Usage
```
claris [options] FILE
```

```
Options:
    -d [directory]      Set output directory. default: Same directory as input
                        file.
    -f                  Overwrite output file.
    -h, --help          Print usage
```

## Example
Source:
```
---
height: 200
width: 200
color: "#ffffff"
layers:
  -
    circle:
      color: "#ffa500"
      x: 100
      y: 100
      radius: 40
      fill: true
      scale:
        x: 1.3
        y: 1
  -
    arc:
      color: "#000000"
      x: 80
      y: 100
      radius: 10
      start: 200
      end: 160
      close: true
      fill: true
      scale:
        x: 1
        y: 1.5
  -
    arc:
      color: "#000000"
      x: 120
      y: 100
      radius: 10
      start: 200
      end: 160
      close: true
      fill: true
      scale:
        x: 1
        y: 1.5
  -
    arc:
      color: "#FF0000"
      x: 100
      y: 120
      radius: 10
      start: 0
      end: 180
      close: true
      fill: true
```

Output:

![face example](https://github.com/masato-hi/claris/blob/master/claris-impl/examples/face_sample.png?raw=true)
