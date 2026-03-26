use super::error::*;
use std::io::Read;

pub struct FrameFlag(u16);

pub struct FrameHeader {
    frame_id: [u8; 4],
    size: [u8; 4],
    flags: [u8; 2]
}

impl FrameHeader {
    fn read_from(reader: &mut impl Read) -> Result<Self, ID3Error> {
        let mut bytes: [u8; 10] = [0; 10];
        reader.read_exact(&mut bytes).map_err(|_| ID3Error::NotEnoughBytes)?;

        Ok(Self{
            frame_id: [bytes[0], bytes[1], bytes[2], bytes[3]],
            size: [bytes[4], bytes[5], bytes[6], bytes[7]],
            flags: [bytes[8], bytes[9]]
        })
    }

    fn size(&self) -> u32 {
        u32::from_be_bytes(self.size)
    }

    fn id(&self) -> String {
        String::from_utf8(self.frame_id.to_vec()).unwrap()
    }

    fn has_flag(&self, flag: FrameFlag) -> bool {
        todo!();
    }
}

pub enum FrameData {
    Text(Vec<u8>),
    URL(Vec<u8>),
    Comment(Vec<u8>),
    People(Vec<u8>),
    Image(Vec<u8>),
    Other(Vec<u8>),
}

impl FrameData {
    fn internal_data(&self) -> &Vec<u8> {
        match self {
            Self::URL(data)
            | Self::Other(data)
            | Self::Image(data)
            | Self::Comment(data)
            | Self::People(data)
            | Self::Text(data) => {
                data
            }
        }
    }
}

pub struct Frame {
    header: FrameHeader,
    data: FrameData
}

impl Frame {
    fn read_from(reader: &mut impl Read) -> Result<Self, ID3Error>{
        let header = FrameHeader::read_from(reader)?;
        let size = header.size();

        // Read the amount of bytes specified in the header
        let mut buffer: Vec<u8> = Vec::with_capacity(size as usize);
        reader.take(size as u64).read_to_end(&mut buffer).map_err(|_| ID3Error::NotEnoughBytes)?;

        // Match the frame ID to the FrameData it should be stored in
        let data = match header.id().chars().collect::<Vec<char>>()[0..4] {
            ['T', _, _, _] => FrameData::Text(buffer),
            ['W', _, _, _] => FrameData::URL(buffer),
            ['A', 'P', 'I', 'C'] => FrameData::Image(buffer),
            ['I', 'P', 'L', 'S'] => FrameData::People(buffer),
            ['C', 'O', 'M', 'M'] => FrameData::Comment(buffer),
            _ => FrameData::Other(buffer)
        };

        // Return the frame
        Ok(Frame { header, data })
    }

    fn id(&self) -> String {
        self.header.id()
    }

    fn data(&self) -> &Vec<u8> {
        self.data.internal_data()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_header_from_valid_bytes() {
        let bytes: [u8; 10] = [0x54, 0x49, 0x54, 0x32, 0x00, 0x00, 0x00, 0x25, 0x00, 0x00];
        FrameHeader::read_from(&mut bytes.as_slice()).unwrap();
    }

    #[test]
    fn frame_header_from_too_many_bytes() {
        let bytes: [u8; 11] = [0x54, 0x49, 0x54, 0x32, 0x00, 0x00, 0x00, 0x25, 0x00, 0x00, 0x00];
        FrameHeader::read_from(&mut bytes.as_slice()).unwrap();
    }

    #[test]
    fn frame_header_error_from_not_enough_bytes() {
        let bytes: [u8; 9] = [0x54, 0x49, 0x54, 0x32, 0x00, 0x00, 0x00, 0x25, 0x00];
        let frame_header = FrameHeader::read_from(&mut bytes.as_slice());
        match frame_header {
            Err(ID3Error::NotEnoughBytes) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn frame_header_size() {
        let bytes: [u8; 10] = [0x54, 0x49, 0x54, 0x32, 0x00, 0x00, 0x00, 0x25, 0x00, 0x00];
        let head = FrameHeader::read_from(&mut bytes.as_slice()).unwrap();
        assert_eq!(head.size(), 37)
    }

    #[test]
    fn frame_header_id() {
        let bytes: [u8; 10] = [0x54, 0x49, 0x54, 0x32, 0x00, 0x00, 0x00, 0x25, 0x00, 0x00];
        let head = FrameHeader::read_from(&mut bytes.as_slice()).unwrap();
        assert_eq!(head.id(), "TIT2".to_string())
    }

    #[test]
    fn text_frame_internal_data() {
        let bytes = vec![1, 2, 3, 4];
        let frame = FrameData::Text(bytes.clone());
        assert_eq!(&bytes, frame.internal_data())
    }

    #[test]
    fn url_frame_internal_data() {
        let bytes = vec![1, 2, 3, 4];
        let frame = FrameData::URL(bytes.clone());
        assert_eq!(&bytes, frame.internal_data())
    }

    #[test]
    fn people_frame_internal_data() {
        let bytes = vec![1, 2, 3, 4];
        let frame = FrameData::People(bytes.clone());
        assert_eq!(&bytes, frame.internal_data())
    }

    #[test]
    fn image_frame_internal_data() {
        let bytes = vec![1, 2, 3, 4];
        let frame = FrameData::Image(bytes.clone());
        assert_eq!(&bytes, frame.internal_data())
    }

    #[test]
    fn comment_frame_internal_data() {
        let bytes = vec![1, 2, 3, 4];
        let frame = FrameData::Comment(bytes.clone());
        assert_eq!(&bytes, frame.internal_data())
    }

    #[test]
    fn other_frame_internal_data() {
        let bytes = vec![1, 2, 3, 4];
        let frame = FrameData::Other(bytes.clone());
        assert_eq!(&bytes, frame.internal_data())
    }

    #[test]
    fn frame_from_valid_bytes() {
        let bytes = vec![0x54, 0x49, 0x54, 0x32, 0x00, 0x00, 0x00, 0x25, 0x00, 0x00, 0x01, 0xFF, 0xFE, 0x50, 0x00, 0x6F, 0x00, 0x6C, 0x00, 0x79, 0x00, 0x67, 0x00, 0x6F, 0x00, 0x6E, 0x00, 0x64, 0x00, 0x77, 0x00, 0x61, 0x00, 0x6E, 0x00, 0x61, 0x00, 0x6C, 0x00, 0x61, 0x00, 0x6E, 0x00, 0x64, 0x00, 0x00, 0x00];
        Frame::read_from(&mut bytes.as_slice()).unwrap();
    }

    #[test]
    fn frame_id_is_correct_from_valid_bytes() {
        let bytes = vec![0x54, 0x49, 0x54, 0x32, 0x00, 0x00, 0x00, 0x25, 0x00, 0x00, 0x01, 0xFF, 0xFE, 0x50, 0x00, 0x6F, 0x00, 0x6C, 0x00, 0x79, 0x00, 0x67, 0x00, 0x6F, 0x00, 0x6E, 0x00, 0x64, 0x00, 0x77, 0x00, 0x61, 0x00, 0x6E, 0x00, 0x61, 0x00, 0x6C, 0x00, 0x61, 0x00, 0x6E, 0x00, 0x64, 0x00, 0x00, 0x00];
        let frame = Frame::read_from(&mut bytes.as_slice()).unwrap();
        assert_eq!(frame.id(), "TIT2".to_string());
    }

    #[test]
    fn frame_data_is_correct_from_valid_bytes() {
        let bytes = vec![0x54, 0x49, 0x54, 0x32, 0x00, 0x00, 0x00, 0x25, 0x00, 0x00, 0x01, 0xFF, 0xFE, 0x50, 0x00, 0x6F, 0x00, 0x6C, 0x00, 0x79, 0x00, 0x67, 0x00, 0x6F, 0x00, 0x6E, 0x00, 0x64, 0x00, 0x77, 0x00, 0x61, 0x00, 0x6E, 0x00, 0x61, 0x00, 0x6C, 0x00, 0x61, 0x00, 0x6E, 0x00, 0x64, 0x00, 0x00, 0x00];
        let frame = Frame::read_from(&mut bytes.as_slice()).unwrap();
        assert_eq!(frame.data(), &vec![0x01, 0xFF, 0xFE, 0x50, 0x00, 0x6F, 0x00, 0x6C, 0x00, 0x79, 0x00, 0x67, 0x00, 0x6F, 0x00, 0x6E, 0x00, 0x64, 0x00, 0x77, 0x00, 0x61, 0x00, 0x6E, 0x00, 0x61, 0x00, 0x6C, 0x00, 0x61, 0x00, 0x6E, 0x00, 0x64, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn frame_header_has_flag_from_frame_flag() {
        let bytes: [u8; 10] = [0x54, 0x49, 0x54, 0x32, 0x00, 0x00, 0x00, 0x25, 0xE0, 0xE0];
        let head = FrameHeader::read_from(&mut bytes.as_slice()).unwrap();
        assert!(head.has_flag(FrameFlag(0xE0E0)))
    }
}
