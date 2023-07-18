use crate::{ReadExactError, WriteAllError};
pub(crate) use embedded_io::{ErrorType as IoBase, Read, Seek, SeekFrom, Write};

pub(crate) trait ReadLeExt {
    type Error;
    async fn read_u8(&mut self) -> Result<u8, Self::Error>;
    async fn read_u16_le(&mut self) -> Result<u16, Self::Error>;
    async fn read_u32_le(&mut self) -> Result<u32, Self::Error>;
}

impl<T: Read> ReadLeExt for T
where
    <T as IoBase>::Error: From<ReadExactError<<T as IoBase>::Error>>,
{
    type Error = <Self as IoBase>::Error;

    async fn read_u8(&mut self) -> Result<u8, Self::Error> {
        let mut buf = [0_u8; 1];
        self.read_exact(&mut buf).await?;
        Ok(buf[0])
    }

    async fn read_u16_le(&mut self) -> Result<u16, Self::Error> {
        let mut buf = [0_u8; 2];
        self.read_exact(&mut buf).await?;
        Ok(u16::from_le_bytes(buf))
    }

    async fn read_u32_le(&mut self) -> Result<u32, Self::Error> {
        let mut buf = [0_u8; 4];
        self.read_exact(&mut buf).await?;
        Ok(u32::from_le_bytes(buf))
    }
}

pub(crate) trait WriteLeExt {
    type Error;
    async fn write_u8(&mut self, n: u8) -> Result<(), Self::Error>;
    async fn write_u16_le(&mut self, n: u16) -> Result<(), Self::Error>;
    async fn write_u32_le(&mut self, n: u32) -> Result<(), Self::Error>;
}

impl<T: Write> WriteLeExt for T
where
    <T as IoBase>::Error: From<WriteAllError<<T as IoBase>::Error>>,
{
    type Error = <Self as IoBase>::Error;

    async fn write_u8(&mut self, n: u8) -> Result<(), Self::Error> {
        self.write_all(&[n]).await?;
        Ok(())
    }

    async fn write_u16_le(&mut self, n: u16) -> Result<(), Self::Error> {
        self.write_all(&n.to_le_bytes()).await?;
        Ok(())
    }

    async fn write_u32_le(&mut self, n: u32) -> Result<(), Self::Error> {
        self.write_all(&n.to_le_bytes()).await?;
        Ok(())
    }
}
