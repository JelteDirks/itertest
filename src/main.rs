
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


struct LineBoundaries<'a> {
    cursor: usize,
    buf: &'a[u8],
}

impl<'a> LineBoundaries<'a> {
    fn new(buf: &'a[u8]) -> Self {
        LineBoundaries {
            cursor: 0,
            buf,
        }
    }
}

impl<'a> Iterator for LineBoundaries<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {

        if self.cursor == self.buf.len() {
            return None;
        }

        let start = self.cursor;

        loop {
            match self.buf.get(self.cursor) {
                Some(n) => {
                    if *n == b'\n' {
                        break;
                    }
                }
                None => return Some((start, self.cursor))
            }

            self.cursor += 1;
        }

        let end = if self.cursor == 0 {
            0
        } else if self.buf[self.cursor - 1] == b'\r' {
            self.cursor - 1
        } else {
            self.cursor
        };

        self.cursor += 1;

        return Some((start, end));
    }
}

fn main() {
    let byte_slice: &[u8] = SOME_VALUE.as_bytes();

    println!("length: {}", byte_slice.len());

    let mut lb = LineBoundaries::new(byte_slice);

    lb.next();

    for boundaries in lb {
        println!("{:?}", boundaries);
        println!("{}", String::from_utf8(byte_slice[boundaries.0..boundaries.1].to_vec()).unwrap());
    }
}
