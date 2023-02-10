#[allow(clippy::all)]
pub fn testfunc(s: S1<'_,>,a: u64,t: E1,) -> i32{
  
  #[allow(unused_imports)]
  use wit_bindgen_guest_rust::rt::{{alloc, vec::Vec, string::String}};
  unsafe {
    let S1{ a:a0, b:b0, ptr:ptr0, c:c0, funcptr:funcptr0, } = s;
    let vec1 = a0;
    let ptr1 = vec1.as_ptr() as i32;
    let len1 = vec1.len() as i32;
    
    #[link(wasm_import_module = "$root")]
    extern "C" {
      #[cfg_attr(target_arch = "wasm32", link_name = "testfunc")]
      #[cfg_attr(not(target_arch = "wasm32"), link_name = "$root_testfunc")]
      fn wit_import(
      _: i32, _: i32, _: i32, _: i64, _: i64, _: i64, _: i64, _: i32, ) -> i32;
    }
    let ret = wit_import(ptr1, len1, wit_bindgen_guest_rust::rt::as_i32(b0), wit_bindgen_guest_rust::rt::as_i64(ptr0), wit_bindgen_guest_rust::rt::as_i64(c0), wit_bindgen_guest_rust::rt::as_i64(funcptr0), wit_bindgen_guest_rust::rt::as_i64(a), match t {
      E1::A => 0,
      E1::B => 1,
      E1::C => 2,
    });
    ret
  }
}
#[derive(Clone)]
pub struct S1 {
  pub a: wit_bindgen_guest_rust::rt::vec::Vec::<i32>,
  pub b: i32,
  pub ptr: u64,
  pub c: u64,
  pub funcptr: u64,
}
impl core::fmt::Debug for S1 {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("S1").field("a", &self.a).field("b", &self.b).field("ptr", &self.ptr).field("c", &self.c).field("funcptr", &self.funcptr).finish()
  }
}
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum E1 {
  A,
  B,
  C,
}
impl core::fmt::Debug for E1 {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      E1::A => {
        f.debug_tuple("E1::A").finish()
      }
      E1::B => {
        f.debug_tuple("E1::B").finish()
      }
      E1::C => {
        f.debug_tuple("E1::C").finish()
      }
    }
  }
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:host"]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 214] = [2, 0, 6, 115, 105, 109, 112, 108, 101, 6, 115, 105, 109, 112, 108, 101, 4, 104, 111, 115, 116, 0, 97, 115, 109, 11, 0, 1, 0, 7, 112, 1, 65, 2, 1, 65, 7, 1, 112, 122, 1, 114, 5, 1, 97, 0, 1, 98, 122, 3, 112, 116, 114, 119, 1, 99, 119, 7, 102, 117, 110, 99, 112, 116, 114, 119, 4, 2, 83, 49, 0, 3, 0, 1, 1, 109, 3, 1, 97, 1, 98, 1, 99, 4, 2, 69, 49, 0, 3, 0, 3, 1, 64, 3, 1, 115, 2, 1, 97, 119, 1, 116, 4, 0, 122, 3, 8, 116, 101, 115, 116, 102, 117, 110, 99, 0, 1, 5, 4, 4, 104, 111, 115, 116, 16, 112, 107, 103, 58, 47, 115, 105, 109, 112, 108, 101, 47, 104, 111, 115, 116, 4, 0, 0, 45, 9, 112, 114, 111, 100, 117, 99, 101, 114, 115, 1, 12, 112, 114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 1, 13, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 5, 48, 46, 53, 46, 49, 11, 22, 1, 6, 115, 105, 109, 112, 108, 101, 11, 112, 107, 103, 58, 47, 115, 105, 109, 112, 108, 101, 3, 0];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
