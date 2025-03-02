use std::{fs::File, io::{BufReader, Error, ErrorKind, Read, Seek, SeekFrom}, usize};

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug)]
struct BmpHeader {
    offset: u32,
    width: u32,
    height: u32,
    bit_count: u16
}

pub fn load_bitmap(filename: &str) -> Result<Vec<u32>, Error> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);

    let header = match read_bitmap_header(&mut buf_reader) {
        Ok(header) => header,
        Err(error) => return Err(error)
    };

    assert!(header.bit_count == 24, "Only 24 bits bitmaps are supported");
    let bytes_per_pixel: u32 = (header.bit_count / 8) as u32;

    let stride = (header.width * bytes_per_pixel + bytes_per_pixel) & (!bytes_per_pixel);
    let size: usize = (header.height * stride) as usize;
    let _ = buf_reader.seek(SeekFrom::Start(header.offset.into()));
    let mut bitmap_buffer: Vec<u8> = vec![0; size];
    buf_reader.read_exact(&mut bitmap_buffer)?;

    let mut xrgb_array: Vec<u32> = vec![0; (header.width * header.height) as usize];
    let mut xrgb_index = 0;
    for y in (0..header.height).rev() {
        for x in 0..header.width {
            let index = (stride * y) + (x * bytes_per_pixel);
            let r: u8 = bitmap_buffer[(index + 2) as usize];
            let g: u8 = bitmap_buffer[(index + 1) as usize];
            let b: u8 = bitmap_buffer[index as usize];

            xrgb_array[xrgb_index] = (r as u32) << 16 | (g as u32) << 8 | (b as u32);
            xrgb_index += 1;
        }
    }

    Ok(xrgb_array)
}

fn read_bitmap_header<T: Read + Seek>(reader: &mut T) -> Result<BmpHeader, Error> {
    let bm_type = reader.read_u16::<LittleEndian>()?;
    if bm_type != 0x4D42 {
        return Err(Error::new(ErrorKind::Other, format!("bm_type({}) not valid for bitmap header", bm_type)));
    }

    let _ = reader.seek(std::io::SeekFrom::Current(8));
    let bm_offset = reader.read_u32::<LittleEndian>()?;

    // skip struct size
    let _ = reader.seek(std::io::SeekFrom::Current(4));
    let bm_width = reader.read_u32::<LittleEndian>()?;
    let bm_height = reader.read_u32::<LittleEndian>()?;
    // skip planes
    let _ = reader.seek(std::io::SeekFrom::Current(2));
    let bm_bit_count = reader.read_u16::<LittleEndian>()?;

    Ok(BmpHeader {
        offset: bm_offset,
        width: bm_width,
        height: bm_height,
        bit_count: bm_bit_count
    })
}
