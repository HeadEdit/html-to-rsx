# HTML to RSX

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
