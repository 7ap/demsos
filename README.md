# 🧮 demsos

[Desmos](https://www.desmos.com/) command-line image uploader.

## 🗺️ Features

* [x] Support for a "vanity hash" (10 character limit)
* [x] Upload raw image to the graph and the embed thumbnail
* [x] Automatic upload to Desmos

## 🏗️ Installation

```bash
$ cargo install demsos
```

## ⚙️ Usage

View usage with the `demsos --help` command.

### 🤔 Examples

* `demsos foo.png` ➜ Upload an image to Desmos with a randomized hash
* `demsos bar.png 10charhash` ➜ Upload an image to Desmos with a user-defined hash
