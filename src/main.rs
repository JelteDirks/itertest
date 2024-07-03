
const SOME_VALUE: &str = r#"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
Dui ut ornare lectus sit amet. Diam maecenas ultricies mi eget mauris pharetra.
Ut porttitor leo a diam sollicitudin tempor id eu nisl. Tristique sollicitudin nibh sit amet commodo nulla facilisi nullam.
Massa tincidunt dui ut ornare lectus sit amet est placerat. Metus vulputate eu scelerisque felis imperdiet.
Orci ac auctor augue mauris augue neque gravida. A condimentum vitae sapien pellentesque habitant morbi tristique senectus et.
Aliquet risus feugiat in ante metus dictum at. Scelerisque in dictum non consectetur a erat nam at.
Quam adipiscing vitae proin sagittis nisl rhoncus. Sit amet dictum sit amet.

Tempus imperdiet nulla malesuada pellentesque elit. Ornare massa eget egestas purus viverra accumsan in nisl nisi.
Mauris in aliquam sem fringilla ut morbi. Faucibus a pellentesque sit amet porttitor eget dolor.
Semper viverra nam libero justo laoreet sit amet cursus. Diam maecenas ultricies mi eget mauris.
Nibh mauris cursus mattis molestie a iaculis at erat. Ullamcorper velit sed ullamcorper morbi tincidunt.
Lorem dolor sed viverra ipsum nunc aliquet. Gravida rutrum quisque non tellus orci.
Tincidunt dui ut ornare lectus sit amet est. Eget est lorem ipsum dolor sit amet."#;


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

        let end = if self.buf[self.cursor - 1] == b'\r' {
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

    let lb = LineBoundaries::new(byte_slice);

    for boundaries in lb {
        println!("{:?}", boundaries);

        println!("{}", String::from_utf8(byte_slice[boundaries.0..boundaries.1].to_vec()).unwrap());
    }
}
