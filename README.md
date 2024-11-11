# HTML to RSX
基于https://github.com/swanandx/html-to-rsx的二次开发

添加了格式化和html-to-rsx关键词转换

RSX is a meta language used by [dioxus](https://github.com/DioxusLabs/dioxus).
This is a simple utility to convert HTML to RSX to save you some time!

You can clone the repo and execute using:
```sh
cargo r -- <PATH TO HTML FILE>
```

Or you can pass HTML from stdin as well
```sh
cat <HTML FILE> | cargo r
```

For example, 
HTML:
```html
<div id="hero" class="container">
  <p>This is awesome!</p>
  <br />
</div>
```
RSX:
```
div { 
    id: "hero",
    class: "container",
    p { "This is awesome!"}
    br { }
}
```
