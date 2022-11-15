use anyhow::bail;

pub(crate) fn get_varint_size(value: i32) -> anyhow::Result<i32> {
    if value < 0 {
        bail!("Value must be positive");
    } else if value < 0x80 {
        Ok(1)
    } else if value < 0x4000 {
        Ok(2)
    } else if value < 0x200000 {
        Ok(3)
    } else if value < 0x10000000 {
        Ok(4)
    } else {
        Ok(5)
    }
}
