const BIAS: i32 = -332;


lazy_static! {
  static ref B1: char = unsafe { char::from_u32_unchecked(0x110001) };
  static ref B2: char = unsafe { char::from_u32_unchecked(0x110002) };
  static ref B3: char = unsafe { char::from_u32_unchecked(0x110003) };
  static ref E1: char = unsafe { char::from_u32_unchecked(0x110004) };
  static ref E2: char = unsafe { char::from_u32_unchecked(0x110005) };
  static ref E3: char = unsafe { char::from_u32_unchecked(0x110006) };
}