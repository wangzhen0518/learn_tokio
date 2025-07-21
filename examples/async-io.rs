use tokio::{
    fs::File,
    io::{self, AsyncReadExt, AsyncWriteExt},
};

async fn async_read() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = [0; 10];
    let n = f.read(&mut buffer).await?;
    println!("The bytes: {:?}", &buffer[..n]);
    Ok(())
}

async fn async_read_to_end() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).await?;
    println!("The bytes: {:?}", buffer);
    Ok(())
}

async fn async_write() -> io::Result<()> {
    let mut file = File::create("foo.txt").await?;
    let n = file.write(b"some bytes").await?;
    println!("Wrote the first {} bytes of 'some bytes'.", n);
    Ok(())
}

async fn async_write_all() -> io::Result<()> {
    let mut file = File::create("foo.txt").await?;
    file.write_all(b"some bytes").await?;
    Ok(())
}

async fn async_copy() -> io::Result<()> {
    let mut reader: &[u8] = b"hello";
    let mut file = File::create("foo.txt").await?;
    io::copy(&mut reader, &mut file).await?;
    println!("{:?}", reader);
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    async_read().await?;
    async_read_to_end().await?;
    async_write().await?;
    async_write_all().await?;
    async_copy().await?;

    Ok(())
}
