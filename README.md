# colors-of-a-film-rust

## Details

My was my first Rust project where I wanted to implement a small application to get to know the language better.
The "colors-of-a-film" application is already implemented in Python (e.g. here: https://github.com/sacert/Colors-of-Film).
Such a project is a nice way to get into practical experience in a language as it involves not only the Standard system packages but third-party packages (such as _rust-opencv_) as well.

## Dependencies

Install Rust for your OS.
Windows might be tricky as the _rust-opencv_ package may involve some fiddling around.
However, Linux' rust support seems great at the moment.

## Usage

Colors-of-a-film 0.1.0
Florian Teich
Generate color palette of a given video file.

USAGE:
    rust-colors-of-film [OPTIONS]

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -h, --height <h>         Height of output image [default: 64]
    -i, --input <input>      The input video file
    -o, --output <output>    The output video file
    -w, --width <w>          Width of output image [default: 1024]

For a quck setup of the environment:

`cargo run`

For a compilation, run:
`cargo build`

## Results

Here is an example of the output of the program for the provided video ("sample_video.mp4") only containing a single scene:

![output.png](output.png)

Especially for more complex videos (or entire movies), the images look stunning!
