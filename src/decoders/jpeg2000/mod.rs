use openjpeg_sys as opj;
use std::cell::RefMut;
use std::convert::TryInto;

use crate::context::{SectionBody, SectionInfo};
use crate::decoders::common::*;
use crate::decoders::simple::*;
use crate::error::*;
use crate::reader::Grib2Read;
use crate::utils::{read_as, GribInt};

mod ext;
use ext::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Jpeg2000CodeStreamDecodeError {
    NotSupported,
    DecoderSetupError,
    MainHeaderReadError,
    BodyReadError,
    LengthMismatch,
}

pub(crate) struct Jpeg2000CodeStreamDecoder {}

impl<R: Grib2Read> Grib2DataDecode<R> for Jpeg2000CodeStreamDecoder {
    fn decode(
        sect5: &SectionInfo,
        sect6: &SectionInfo,
        sect7: &SectionInfo,
        mut reader: RefMut<R>,
    ) -> Result<Box<[f32]>, GribError> {
        let (sect5_body, sect6_body) = match (sect5.body.as_ref(), sect6.body.as_ref()) {
            (Some(SectionBody::Section5(b5)), Some(SectionBody::Section6(b6))) => (b5, b6),
            _ => return Err(GribError::InternalDataError),
        };

        if sect6_body.bitmap_indicator != 255 {
            return Err(GribError::DecodeError(
                DecodeError::BitMapIndicatorUnsupported,
            ));
        }

        let sect5_data = reader.read_sect_payload_as_slice(sect5)?;
        let ref_val = read_as!(f32, sect5_data, 6);
        let exp = read_as!(u16, sect5_data, 10).as_grib_int();
        let dig = read_as!(u16, sect5_data, 12).as_grib_int();
        //let nbit = read_as!(u8, sect5_data, 14);
        let value_type = read_as!(u8, sect5_data, 15);

        if value_type != 0 {
            return Err(GribError::DecodeError(
                DecodeError::SimplePackingDecodeError(
                    SimplePackingDecodeError::OriginalFieldValueTypeNotSupported,
                ),
            ));
        }

        let sect7_data = reader.read_sect_payload_as_slice(sect7)?;

        let stream = Stream::from_bytes(&sect7_data)
            .map_err(|e| GribError::DecodeError(DecodeError::Jpeg2000CodeStreamDecodeError(e)))?;
        let jp2_unpacked = decode_jp2(stream)
            .map_err(|e| GribError::DecodeError(DecodeError::Jpeg2000CodeStreamDecodeError(e)))?;
        let decoded =
            SimplePackingDecodeIterator::new(jp2_unpacked, ref_val, exp, dig).collect::<Vec<_>>();
        if decoded.len() != sect5_body.num_points() as usize {
            return Err(GribError::DecodeError(
                DecodeError::SimplePackingDecodeError(SimplePackingDecodeError::LengthMismatch),
            ));
        }
        Ok(decoded.into_boxed_slice())
    }
}

fn decode_jp2(stream: Stream) -> Result<impl Iterator<Item = i32>, Jpeg2000CodeStreamDecodeError> {
    let codec = Codec::j2k()?;

    let mut decode_params = unsafe { std::mem::zeroed::<opj::opj_dparameters>() };
    unsafe { opj::opj_set_default_decoder_parameters(&mut decode_params as *mut _) };

    if unsafe { openjpeg_sys::opj_setup_decoder(codec.0.as_ptr(), &mut decode_params) } != 1 {
        return Err(Jpeg2000CodeStreamDecodeError::DecoderSetupError);
    }

    let mut image = Image::new();

    if unsafe { opj::opj_read_header(stream.0, codec.0.as_ptr(), &mut image.0) } != 1 {
        return Err(Jpeg2000CodeStreamDecodeError::MainHeaderReadError);
    }

    if unsafe { opj::opj_decode(codec.0.as_ptr(), stream.0, image.0) } != 1 {
        return Err(Jpeg2000CodeStreamDecodeError::BodyReadError);
    }

    drop(codec);
    drop(stream);

    let width = image.width();
    let height = image.height();
    let factor = image.factor();

    let width = value_for_discard_level(width, factor);
    let height = value_for_discard_level(height, factor);

    if let [comp_gray] = image.components() {
        let vec = unsafe {
            std::slice::from_raw_parts(comp_gray.data, (width * height) as usize).to_vec()
        };
        Ok(vec.into_iter())
    } else {
        Err(Jpeg2000CodeStreamDecodeError::NotSupported)
    }
}
