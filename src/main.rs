const SOME_VALUE: &str = r#"HTTP/1.1 200 OK
X-Powered-By: Express
Content-Type: application/octet-stream
Content-Length: 25
ETag: W/"19-B8zw7OXzcTA2cl4FfElEEwnpEvE"
Date: Wed, 03 Jul 2024 18:58:33 GMT
Connection: keep-alive
Keep-Alive: timeout=5

This is some binary data.
"#;


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
    let bytes = SOME_VALUE.as_bytes();

    // === TIME START ===
    let t_start = std::time::Instant::now();
    for boundaries in LineBoundaries::new(SOME_VALUE.as_bytes().iter()) {
        let s = String::from_utf8(bytes[boundaries.0..boundaries.1].to_vec()).unwrap();
        std::hint::black_box(s);
    }
    let t1 = std::time::Instant::now() - t_start;
    // === TIME END ===

    // === TIME START ===
    let t_start = std::time::Instant::now();
    SOME_VALUE.as_bytes().iter().line_boundaries().decode_headers(bytes).for_each(|s| {
        std::hint::black_box(s);
    });
    let t2 = std::time::Instant::now() - t_start;
    // === TIME END ===

    println!("{},{}", t1.as_nanos(), t2.as_nanos());
}
