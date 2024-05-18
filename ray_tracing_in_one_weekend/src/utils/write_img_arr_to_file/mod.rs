use std::{ffi::OsStr, fs, io::Write};

/// Writes image data `img_arr` to a specified file `path`
/// # Panics
/// Panics if the specified file path does not end with a ".ppm" extension
/// or the file with provided path cannot be created or written to
pub fn write_img_arr_to_file(path: &std::path::Path, img_arr: Vec<Vec<[u8; 3]>>) {
    assert_eq!(
        path.extension().unwrap(),
        "ppm",
        "Expected .ppm file, got {}",
        path.extension().and_then(OsStr::to_str).unwrap()
    );

    let img_height = img_arr.len();
    let img_width = img_arr[0].len();

    fs::create_dir_all(path.parent().unwrap()).unwrap();

    match fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
    {
        Err(e) => panic!("Couldn't open file: {e:?}"),
        Ok(mut img_file) => {
            // write default ppm headers
            img_file.write_all(b"P3\n").unwrap();
            img_file
                .write_all(format!("{img_width} {img_height}\n").as_bytes())
                .unwrap();
            img_file.write_all(("255\n").as_bytes()).unwrap();

            // write image
            for col in img_arr {
                for pixel in col {
                    img_file
                        .write_all(pixel.map(|x| format!("{x}")).join(" ").as_bytes())
                        .unwrap();
                    img_file.write_all(b" ").unwrap();
                }
            }
        }
    }
}
