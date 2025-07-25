use std::io::{Cursor, Write};

use bytes::{Buf, BytesMut};
use mini_redis::Frame;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
};

pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

pub struct ConnectionReadHalf {
    stream: OwnedReadHalf,
    buffer: BytesMut,
}

pub struct ConnectionWriteHalf {
    stream: BufWriter<OwnedWriteHalf>,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Connection {
            // stream,
            stream: BufWriter::new(stream),
            buffer: BytesMut::with_capacity(4096),
        }
    }

    pub async fn read_frame(&mut self) -> mini_redis::Result<Option<Frame>> {
        loop {
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            }
        }
    }

    pub async fn write_frame(&mut self, frame: &Frame) -> mini_redis::Result<()> {
        match frame {
            Frame::Array(val) => {
                self.stream.write_u8(b'$').await?;
                self.write_decimal(val.len() as u64).await?;
                for frame in val {
                    self.write_value(frame).await?;
                }
            }
            _ => self.write_value(frame).await?,
        }

        self.stream.flush().await?;
        Ok(())
    }

    async fn write_value(&mut self, frame: &Frame) -> mini_redis::Result<()> {
        match frame {
            Frame::Simple(val) => {
                self.stream.write_u8(b'+').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Error(val) => {
                self.stream.write_u8(b'-').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Integer(val) => {
                self.stream.write_u8(b':').await?;
                self.write_decimal(*val).await?;
            }
            Frame::Null => self.stream.write_all(b"$-1\r\n").await?,
            Frame::Bulk(val) => {
                self.stream.write_u8(b'$').await?;
                self.write_decimal(val.len() as u64).await?;
                self.stream.write_all(val).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Array(_) => unreachable!(),
        }

        Ok(())
    }

    fn parse_frame(&mut self) -> mini_redis::Result<Option<Frame>> {
        let mut buf = Cursor::new(&self.buffer[..]);
        match Frame::check(&mut buf) {
            Ok(_) => {
                let len = buf.position() as usize;
                buf.set_position(0);
                let frame = Frame::parse(&mut buf)?;
                self.buffer.advance(len);
                Ok(Some(frame))
            }
            Err(mini_redis::frame::Error::Incomplete) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    async fn write_decimal(&mut self, val: u64) -> mini_redis::Result<()> {
        let mut buf = [0u8; 20];
        let mut buf = Cursor::new(&mut buf[..]);
        write!(&mut buf, "{}", val)?;

        let pos = buf.position() as usize;
        self.stream.write_all(&buf.get_ref()[..pos]).await?;
        self.stream.write_all(b"\r\n").await?;

        Ok(())
    }

    pub fn split(self) -> (ConnectionReadHalf, ConnectionWriteHalf) {
        let (read_half, write_half) = self.stream.into_inner().into_split();
        let read_half = ConnectionReadHalf {
            stream: read_half,
            buffer: self.buffer,
        };
        let write_half = ConnectionWriteHalf {
            stream: BufWriter::new(write_half),
        };
        (read_half, write_half)
    }
}

impl ConnectionReadHalf {
    pub async fn read_frame(&mut self) -> mini_redis::Result<Option<Frame>> {
        loop {
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            }
        }
    }

    fn parse_frame(&mut self) -> mini_redis::Result<Option<Frame>> {
        let mut buf = Cursor::new(&self.buffer[..]);
        match Frame::check(&mut buf) {
            Ok(_) => {
                let len = buf.position() as usize;
                buf.set_position(0);
                let frame = Frame::parse(&mut buf)?;
                self.buffer.advance(len);
                Ok(Some(frame))
            }
            Err(mini_redis::frame::Error::Incomplete) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}

impl ConnectionWriteHalf {
    pub async fn write_frame(&mut self, frame: &Frame) -> mini_redis::Result<()> {
        match frame {
            Frame::Array(val) => {
                self.stream.write_u8(b'$').await?;
                self.write_decimal(val.len() as u64).await?;
                for frame in val {
                    self.write_value(frame).await?;
                }
            }
            _ => self.write_value(frame).await?,
        }

        self.stream.flush().await?;
        Ok(())
    }

    async fn write_value(&mut self, frame: &Frame) -> mini_redis::Result<()> {
        match frame {
            Frame::Simple(val) => {
                self.stream.write_u8(b'+').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Error(val) => {
                self.stream.write_u8(b'-').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Integer(val) => {
                self.stream.write_u8(b':').await?;
                self.write_decimal(*val).await?;
            }
            Frame::Null => self.stream.write_all(b"$-1\r\n").await?,
            Frame::Bulk(val) => {
                self.stream.write_u8(b'$').await?;
                self.write_decimal(val.len() as u64).await?;
                self.stream.write_all(val).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Array(_) => unreachable!(),
        }

        Ok(())
    }

    async fn write_decimal(&mut self, val: u64) -> mini_redis::Result<()> {
        let mut buf = [0u8; 20];
        let mut buf = Cursor::new(&mut buf[..]);
        write!(&mut buf, "{}", val)?;

        let pos = buf.position() as usize;
        self.stream.write_all(&buf.get_ref()[..pos]).await?;
        self.stream.write_all(b"\r\n").await?;

        Ok(())
    }
}
