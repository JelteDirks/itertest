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
        let mut previous: &u8 = &0;

        match self.iter.next() {
            Some(n) => {
                if *n == b'\n' {
                    self.cursor += 1;
                    return Some((self.cursor-1, self.cursor-1));
                }
                previous = n;
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
                    previous = n;
                    self.cursor += 1;
                }
                None => {
                    return Some((start, self.cursor))
                }
            }
        }

        let end = if *previous == b'\r' {
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

    let lb = LineBoundaries::new(byte_slice.iter());

    for boundaries in lb {
        println!("{:?}", boundaries);
        println!("{}", String::from_utf8(byte_slice[boundaries.0..boundaries.1].to_vec()).unwrap());
    }

    let iterme: &[u8] = SOME_VALUE.as_bytes();

    for boundaries in iterme.iter().line_boundaries() {
        println!("{:?}", boundaries);
        println!("{}", String::from_utf8(iterme[boundaries.0..boundaries.1].to_vec()).unwrap());
    }

}
