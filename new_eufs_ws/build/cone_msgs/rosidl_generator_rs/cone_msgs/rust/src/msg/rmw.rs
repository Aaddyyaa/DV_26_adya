#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "cone_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__cone_msgs__msg__Cone() -> *const std::ffi::c_void;
}

#[link(name = "cone_msgs__rosidl_generator_c")]
extern "C" {
    fn cone_msgs__msg__Cone__init(msg: *mut Cone) -> bool;
    fn cone_msgs__msg__Cone__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Cone>, size: usize) -> bool;
    fn cone_msgs__msg__Cone__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Cone>);
    fn cone_msgs__msg__Cone__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Cone>, out_seq: *mut rosidl_runtime_rs::Sequence<Cone>) -> bool;
}

// Corresponds to cone_msgs__msg__Cone
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Cone {

    // This member is not documented.
    #[allow(missing_docs)]
    pub x: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub z: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub class_id: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub confidence: f32,

}



impl Default for Cone {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !cone_msgs__msg__Cone__init(&mut msg as *mut _) {
        panic!("Call to cone_msgs__msg__Cone__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Cone {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cone_msgs__msg__Cone__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cone_msgs__msg__Cone__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cone_msgs__msg__Cone__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Cone {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Cone where Self: Sized {
  const TYPE_NAME: &'static str = "cone_msgs/msg/Cone";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__cone_msgs__msg__Cone() }
  }
}


#[link(name = "cone_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__cone_msgs__msg__ConeArray() -> *const std::ffi::c_void;
}

#[link(name = "cone_msgs__rosidl_generator_c")]
extern "C" {
    fn cone_msgs__msg__ConeArray__init(msg: *mut ConeArray) -> bool;
    fn cone_msgs__msg__ConeArray__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ConeArray>, size: usize) -> bool;
    fn cone_msgs__msg__ConeArray__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ConeArray>);
    fn cone_msgs__msg__ConeArray__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ConeArray>, out_seq: *mut rosidl_runtime_rs::Sequence<ConeArray>) -> bool;
}

// Corresponds to cone_msgs__msg__ConeArray
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConeArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub cones: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Cone>,

}



impl Default for ConeArray {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !cone_msgs__msg__ConeArray__init(&mut msg as *mut _) {
        panic!("Call to cone_msgs__msg__ConeArray__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ConeArray {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cone_msgs__msg__ConeArray__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cone_msgs__msg__ConeArray__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cone_msgs__msg__ConeArray__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ConeArray {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ConeArray where Self: Sized {
  const TYPE_NAME: &'static str = "cone_msgs/msg/ConeArray";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__cone_msgs__msg__ConeArray() }
  }
}


