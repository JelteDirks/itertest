use std::io::Read;

struct LineBoundaries<'a, Iter>
where Iter: Iterator<Item = &'a u8>
{
    cursor: usize,
    iter: Iter
}

impl<'a, Iter> LineBoundaries<'a, Iter> 
where Iter: Iterator<Item = &'a u8>,
{
    fn new(i: Iter) -> Self {
        LineBoundaries {
            cursor: 0,
            iter: i
        }
    }
}

trait LineBoundariesExt<'a> : Sized
where Self: Iterator<Item = &'a u8>,
{
    fn line_boundaries(self) -> LineBoundaries<'a, Self>; 
}

impl<'a, Iter> LineBoundariesExt<'a> for Iter 
where Iter: Iterator<Item = &'a u8>
{
    fn line_boundaries(self) -> LineBoundaries<'a, Self> {
        LineBoundaries::new(self)
    }
}


impl<'a, Iter> Iterator for LineBoundaries<'a, Iter> 
where Iter: Iterator<Item = &'a u8>
{
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.cursor;
        let mut _previous: &u8 = &0;

        match self.iter.next() {
            Some(n) => {
                if *n == b'\n' {
                    self.cursor += 1;
                    return Some((self.cursor-1, self.cursor-1));
                }
                _previous = n;
                self.cursor += 1;
            }
            None => return None
        }

        loop {
            match self.iter.next() {
                Some(n) => {
                    if *n == b'\n' {
                        break;
                    }
                    _previous = n;
                    self.cursor += 1;
                }
                None => {
                    return Some((start, self.cursor))
                }
            }
        }

        let end = if *_previous == b'\r' {
            self.cursor - 1
        } else {
            self.cursor
        };

        self.cursor += 1;

        return Some((start, end));
    }
}

struct HeaderDecoder<'a, Iter>
where Iter: Iterator<Item = (usize, usize)>
{
    iter: Iter,
    buf: &'a [u8],
}

impl<'a, Iter> HeaderDecoder<'a, Iter>
where Iter: Iterator<Item = (usize, usize)>
{
    fn new(i: Iter, buf: &'a [u8]) -> Self {
        HeaderDecoder { iter: i, buf }
    }
}

impl<'a, Iter> Iterator for HeaderDecoder<'a, Iter>
    where Iter: Iterator<Item = (usize, usize)>
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(pair) => {
                // WARNING: buf can be different than source of pairs!
                return Some(String::from_utf8(self.buf[pair.0..pair.1].to_vec()).unwrap());
            }
            None => None
        }
    }
}

trait HeaderDecoderExt<'a>: Sized
where Self: Iterator<Item = (usize, usize)>
{
    fn decode_headers(self, buf: &'a [u8]) -> HeaderDecoder<'a, Self>;
}

impl <'a, Iter> HeaderDecoderExt<'a> for Iter
where Iter: Iterator<Item = (usize, usize)>,
{
    fn decode_headers(self, buf: &'a [u8]) -> HeaderDecoder<'a, Self> {
        HeaderDecoder::new(self, buf)
    }
}

fn main() {

    let mut buf: Vec<u8> = Vec::new();

    let file = std::fs::File::open("small_file.txt").expect("file not found");
    let mut bufreader = std::io::BufReader::new(file);
    bufreader.read_to_end(&mut buf).expect("file read error");

    let bytes: &[u8] = &buf;

    // === TIME START ===
    let t1_start = std::time::Instant::now();
    for boundaries in LineBoundaries::new(bytes.iter()) {
        let s = String::from_utf8(bytes[boundaries.0..boundaries.1].to_vec()).unwrap();
        std::hint::black_box(s);
    }
    let t1_end = std::time::Instant::now();
    // === TIME END ===

    // === TIME START ===
    let t2_start = std::time::Instant::now();
    bytes.iter().line_boundaries().decode_headers(bytes).for_each(|s| {
        std::hint::black_box(s);
    });
    let t2_end = std::time::Instant::now();
    // === TIME END ===

    let t1 = t1_end - t1_start;
    let t2 = t2_end - t2_start;

    println!("{},{}", t1.as_nanos(), t2.as_nanos());
}

// Based on 100K runs in release.
// file is 1401B
//
// loop1
// mean: 2562.6554
// std: 641.872
// median: 2500
// min: 2291
// max: 20583
// loop2
//
// mean: 1941.8528
// std: 577.8965
// median: 1875
// min: 1667
// max: 20458
