use std::io;

use tower_lsp_server::Client;
use tracing_subscriber::fmt::MakeWriter;

pub struct LspLogger {
    client: Client,
}

impl LspLogger {
    fn new(client: Client) -> Self {
        Self { client }
    }
}

impl io::Write for LspLogger {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let client = self.client.clone();
        let message = buf.to_vec();
        tokio::spawn(async move {
            client
                .log_message(
                    tower_lsp_server::lsp_types::MessageType::LOG,
                    String::from_utf8(message).unwrap(),
                )
                .await;
        });
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub struct LogWriter {
    client: Client,
}

impl<'a> MakeWriter<'a> for LogWriter {
    type Writer = LspLogger;
    fn make_writer(&'a self) -> Self::Writer {
        LspLogger::new(self.client.clone())
    }
}
