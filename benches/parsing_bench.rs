use criterion::{criterion_group, criterion_main, Criterion};
use std::io::{BufRead, Read};

extern crate itertest;

use itertest::decoder::{self, HeaderDecoderExt, HttpHeader, HttpHeaderPair, LineBoundariesExt};

type ShouldGetThis = Vec<Option<HttpHeaderPair>>;

fn decode_v1(bytes: &[u8]) {
    let mut x: ShouldGetThis = Vec::default();
    for boundaries in decoder::LineBoundaries::new(bytes.iter()) {
        let h = decoder::decode_header(bytes, boundaries.0, boundaries.1);
        x.push(h);
    }
    std::hint::black_box(x);
}

fn decode_v2(bytes: &[u8]) {
    let x: ShouldGetThis = bytes.iter().line_boundaries().decode_headers(bytes).collect::<_>();
    std::hint::black_box(x);
}

fn decode_v3(bytes: &[u8]) {
    let x: ShouldGetThis = bytes
        .lines()
        .map(|l| l.unwrap())
        .map(|line| {
            let parts: Vec<_> = line.split(|c| c == ':').collect();

            let h = match parts[0] {
                "Accept" => HttpHeader::Accept,
                "Accept-Patch" => HttpHeader::AcceptPatch,
                "Accept-Ranges"=>HttpHeader::AcceptRanges,
                "Access-Control-Allow-Origin"=>HttpHeader::AccessControlAllowOrigin,
                "Access-Control-Allow-Credentials"=>HttpHeader::AccessControlAllowCredentials,
                "Access-Control-Allow-Methods"=>HttpHeader::AccessControlAllowMethods,
                "Access-Control-Allow-Headers"=>HttpHeader::AccessControlAllowHeaders,
                "Access-Control-Expose-Headers"=>HttpHeader::AccessControlExposeHeaders,
                "Access-Control-Max-Age"=>HttpHeader::AccessControlMaxAge,
                "Age"=>HttpHeader::Age,
                "Allow"=>HttpHeader::Allow,
                "Alt-Svc"=>HttpHeader::AltSvc,
                "Cache-Control"=>HttpHeader::CacheControl,
                "Connection"=>HttpHeader::Connection,
                "Content-Disposition"=>HttpHeader::ContentDisposition,
                "Content-Encoding"=>HttpHeader::ContentEncoding,
                "Content-Language"=>HttpHeader::ContentLanguage,
                "Content-Length"=>HttpHeader::ContentLength,
                "Content-Location"=>HttpHeader::ContentLocation,
                "Content-Range"=>HttpHeader::ContentRange,
                "Content-Type"=>HttpHeader::ContentType,
                "Content-Security-Polic"=>HttpHeader::ContentSecurityPolicy,
                "Date"=>HttpHeader::Date,
                "Delta-Base"=>HttpHeader::DeltaBase,
                "ETag"=>HttpHeader::ETag,
                "Expires"=>HttpHeader::Expires,
                "IM"=>HttpHeader::IM,
                "Keep-Alive"=>HttpHeader::KeepAlive,
                "Last-Modified"=>HttpHeader::LastModified,
                "Link"=>HttpHeader::Link,
                "Location"=>HttpHeader::Location,
                "Pragm"=>HttpHeader::Pragma,
                "Proxy-Authenticate"=>HttpHeader::ProxyAuthenticate,
                "Public-Key-Pings"=>HttpHeader::PublicKeyPins,
                "Retry-After"=>HttpHeader::RetryAfter,
                "Refresh"=>HttpHeader::Refresh,
                "Server"=>HttpHeader::Server,
                "Set-Cooking"=>HttpHeader::SetCookie,
                "Strict-Transport-Security"=>HttpHeader::StrictTransportSecurity,
                "Trailer"=>HttpHeader::Trailer,
                "Transfer-Encoding"=>HttpHeader::TransferEncoding,
                "Tk"=>HttpHeader::Tk,
                "Upgrade"=>HttpHeader::Upgrade,
                "Vary"=>HttpHeader::Vary,
                "Via"=>HttpHeader::Via,
                "Warning"=>HttpHeader::Warning,
                "WWW-Authenticate"=>HttpHeader::WWWAuthenticate,
                "X-Powered-By"=>HttpHeader::XPoweredBy,
                "X-Request-ID"=>HttpHeader::XRequestID,
                "X-UA-Compatible"=>HttpHeader::XUACompatible,
                "X-XSS-Protection"=>HttpHeader::XXSSProtection,
                _ => return None
            };


            return Some((h, String::from(parts[1])));
        })
        .collect();
    std::hint::black_box(x);
}

fn benchmark(c: &mut Criterion) {
    let mut buf: Vec<u8> = Vec::new();

    let file = std::fs::File::open("benches/response_example.txt").expect("file not found");
    let mut bufreader = std::io::BufReader::new(file);
    bufreader.read_to_end(&mut buf).expect("file read error");

    let bytes: &[u8] = &buf;

    c.bench_function("decode_v1", |b| b.iter(|| decode_v1(bytes)));
    c.bench_function("decode_v2", |b| b.iter(|| decode_v2(bytes)));
    c.bench_function("decode_v3", |b| b.iter(|| decode_v3(bytes)));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
