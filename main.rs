extern crate opencv;

use opencv::{
    core,
    highgui,
    imgproc,
    imgcodecs,
    prelude::*,
    videoio,
};
use indicatif::ProgressBar;
use clap::{Arg, App};


fn run() -> opencv::Result<()> {
    let matches = App::new("Colors-of-a-film")
        .version("0.1.0")
        .author("Florian Teich")
        .about("Generate color palette of a given video file.")
        .arg(Arg::with_name("input")
                 .short("i")
                 .long("input")
                 .takes_value(true)
                 .help("The input video file"))
        .arg(Arg::with_name("w")
                 .short("w")
                 .long("width")
                 .takes_value(true)
                 .default_value("1024")
                 .help("Width of output image"))
        .arg(Arg::with_name("h")
                 .short("h")
                 .long("height")
                 .takes_value(true)
                 .default_value("64")
                 .help("Height of output image"))
        .arg(Arg::with_name("output")
                 .short("o")
                 .long("output")
                 .takes_value(true)
                 .help("The output video file"))
        .get_matches();
    let inputfile = matches.value_of("input").unwrap_or("sample_video.mp4");
    let outputfile = matches.value_of("output").unwrap_or("output.png");

    #[cfg(not(feature = "opencv-32"))]
    let mut cam = videoio::VideoCapture::from_file(inputfile, 0)?;
    let mut red = core::Mat::default()?;
    let mut green = core::Mat::default()?;
    let mut blue = core::Mat::default()?;

    //let mut mat = Mat::new_rows_cols_with_default(100, 100, f32::typ(), core::Scalar::from(1.23)).unwrap();

    let mut num_frames = cam.get(7).unwrap() as i64;
    num_frames = num_frames + 1;
    let mut final_width = matches.value_of("width").unwrap_or("1024").parse::<i64>().unwrap();
    let mut final_height = matches.value_of("height").unwrap_or("32").parse::<i64>().unwrap();
    let mut curr_frame = 0;
    let mut vec_r = Vec::<f64>::new();
    let mut vec_g = Vec::<f64>::new();
    let mut vec_b = Vec::<f64>::new();
    let rs: [f64; 128] = [0.0; 128];
    let gs: [f64; 128] = [0.0; 128];
    let bs: [f64; 128] = [0.0; 128];

    println!("Frames: {}", cam.get(7).unwrap());

    let mut mat2 = Mat::new_rows_cols_with_default((num_frames as i32), 1, core::CV_8UC3, core::Scalar::from(0.0)).unwrap();
    let mut mat3 = Mat::new_rows_cols_with_default((num_frames as i32), 1, core::CV_8UC3, core::Scalar::from(0.0)).unwrap();
    let bar = ProgressBar::new(((num_frames) as u64));

    loop {
        let mut frame = core::Mat::default()?;
        cam.read(&mut frame)?;
        if frame.size()?.width > 0 {
            bar.inc(1);
            curr_frame = curr_frame + 1;
            
            core::extract_channel(&frame, &mut red, 0)?;
            core::extract_channel(&frame, &mut green, 1)?;
            core::extract_channel(&frame, &mut blue, 2)?;

            let mut red_mean = core::mean(&red, &red).unwrap()[0];
            let mut green_mean = core::mean(&green, &green).unwrap()[0];
            let mut blue_mean = core::mean(&blue, &blue).unwrap()[0];

            let t = mat2.at_row_mut::<core::Vec3b>(curr_frame)?;
            t[0] = core::Vec3b::from([(red_mean as u8), (green_mean as u8), (blue_mean as u8)]);

        }else{
            break;
        }
    }
    bar.finish();
    opencv::core::transpose(&mat2, &mut mat3);
    mat2 = Mat::new_rows_cols_with_default((final_width as i32), (final_height as i32), core::CV_8UC3, core::Scalar::from(0.0)).unwrap();
    let s: opencv::core::Size = opencv::core::Size::new((final_width as i32), (final_height as i32));
    opencv::imgproc::resize(&mat3, &mut mat2, s, 0.0, 0.0, 0);
    imgcodecs::imwrite(outputfile, &mat2, &opencv::types::VectorOfi32::new()); 
    Ok(())
}

fn main() {
    run().unwrap()
}