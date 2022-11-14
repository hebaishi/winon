pub fn zeroes(size: usize) -> Vec<u16>  {
  let mut zero_vec: Vec<u16> = Vec::with_capacity(size as usize);
  zero_vec.resize(size, 0);
  zero_vec
}
