// plugins/http-flood/src/lib.rs
pub struct HttpFlood {
    target: String,
    rate: usize,
}

impl TrafficPattern for HttpFlood {
    fn name(&self) -> &str { "http-flood" }

    fn generate(&self, ctx: &mut Context) -> Result<Vec<Packet>> {
        let request = format!(
            "GET / HTTP/1.1\r\nHost: {}\r\n\r\n",
            self.target
        );

        vec![Packet {
            protocol: Protocol::Http,
            data: request.into_bytes(),
            destination: self.target.parse()?,
        }]
    }
}