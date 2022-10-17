# 🧮 demsos

[Desmos](https://www.desmos.com/) command-line image uploader.

## 🗺️ Features

* [x] Support for a "vanity hash" (10 character limit)
* [x] Convert PNG to LaTeX on the fly
* [x] Automatic upload to Desmos
* [ ] Compression (5 megabyte limit on requests)

## 🏗️ Installation

```bash
$ cargo install demsos
```

## ⚙️ Usage

View usage with the `demsos --help` command.

### 🤔 Examples

* `demsos foo.png` ➜ Upload an image to Desmos with a randomized hash
* `demsos bar.png 10charhash` ➜ Upload an image to Desmos with a user-defined hash
