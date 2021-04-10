# svg-to-bmp

This is my first cargo program. It reads the `svg` file and gets the info (cx, cy, fill) and it creates a bitmap.

#### big-svg.svg: 

```
<?xml version="1.0" standalone="no"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 20010904//EN"
  "http://www.w3.org/TR/2001/REC-SVG-20010904/DTD/svg10.dtd">
<svg width="512" height="414">
  <circle cx="0" cy="0" r="1" fill="white"/>
  <circle cx="1" cy="0" r="1" fill="white"/>
  <circle cx="2" cy="0" r="1" fill="white"/>
  <circle cx="3" cy="0" r="1" fill="white"/>
  <circle cx="4" cy="0" r="1" fill="white"/>
  <circle cx="5" cy="0" r="1" fill="white"/>
  ...
  <circle cx="503" cy="413" r="1" fill="white"/>
  <circle cx="504" cy="413" r="1" fill="white"/>
  <circle cx="505" cy="413" r="1" fill="white"/>
  <circle cx="506" cy="413" r="1" fill="white"/>
  <circle cx="507" cy="413" r="1" fill="white"/>
  <circle cx="508" cy="413" r="1" fill="white"/>
  <circle cx="509" cy="413" r="1" fill="white"/>
  <circle cx="510" cy="413" r="1" fill="white"/>
  <circle cx="511" cy="413" r="1" fill="white"/>
</svg>  
```

#### big-svg.bmp:

![alt svg-to-bmp](big-svg.bmp) 