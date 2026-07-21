#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__srv__RecordStart_Request() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__srv__RecordStart_Request__init(msg: *mut RecordStart_Request) -> bool;
    fn eufs_msgs__srv__RecordStart_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RecordStart_Request>, size: usize) -> bool;
    fn eufs_msgs__srv__RecordStart_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RecordStart_Request>);
    fn eufs_msgs__srv__RecordStart_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RecordStart_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<RecordStart_Request>) -> bool;
}

// Corresponds to eufs_msgs__srv__RecordStart_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RecordStart_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pack_number: i64,

}



impl Default for RecordStart_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__srv__RecordStart_Request__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__srv__RecordStart_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RecordStart_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__RecordStart_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__RecordStart_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__RecordStart_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RecordStart_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RecordStart_Request where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/srv/RecordStart_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__srv__RecordStart_Request() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__srv__RecordStart_Response() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__srv__RecordStart_Response__init(msg: *mut RecordStart_Response) -> bool;
    fn eufs_msgs__srv__RecordStart_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RecordStart_Response>, size: usize) -> bool;
    fn eufs_msgs__srv__RecordStart_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RecordStart_Response>);
    fn eufs_msgs__srv__RecordStart_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RecordStart_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<RecordStart_Response>) -> bool;
}

// Corresponds to eufs_msgs__srv__RecordStart_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RecordStart_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub start_recording: bool,

}



impl Default for RecordStart_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__srv__RecordStart_Response__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__srv__RecordStart_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RecordStart_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__RecordStart_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__RecordStart_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__RecordStart_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RecordStart_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RecordStart_Response where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/srv/RecordStart_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__srv__RecordStart_Response() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__srv__RecordStop_Request() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__srv__RecordStop_Request__init(msg: *mut RecordStop_Request) -> bool;
    fn eufs_msgs__srv__RecordStop_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RecordStop_Request>, size: usize) -> bool;
    fn eufs_msgs__srv__RecordStop_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RecordStop_Request>);
    fn eufs_msgs__srv__RecordStop_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RecordStop_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<RecordStop_Request>) -> bool;
}

// Corresponds to eufs_msgs__srv__RecordStop_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RecordStop_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for RecordStop_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__srv__RecordStop_Request__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__srv__RecordStop_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RecordStop_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__RecordStop_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__RecordStop_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__RecordStop_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RecordStop_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RecordStop_Request where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/srv/RecordStop_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__srv__RecordStop_Request() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__srv__RecordStop_Response() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__srv__RecordStop_Response__init(msg: *mut RecordStop_Response) -> bool;
    fn eufs_msgs__srv__RecordStop_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RecordStop_Response>, size: usize) -> bool;
    fn eufs_msgs__srv__RecordStop_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RecordStop_Response>);
    fn eufs_msgs__srv__RecordStop_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RecordStop_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<RecordStop_Response>) -> bool;
}

// Corresponds to eufs_msgs__srv__RecordStop_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RecordStop_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub stop_recording: bool,

}



impl Default for RecordStop_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__srv__RecordStop_Response__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__srv__RecordStop_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RecordStop_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__RecordStop_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__RecordStop_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__RecordStop_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RecordStop_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RecordStop_Response where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/srv/RecordStop_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__srv__RecordStop_Response() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__srv__Register_Request() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__srv__Register_Request__init(msg: *mut Register_Request) -> bool;
    fn eufs_msgs__srv__Register_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Register_Request>, size: usize) -> bool;
    fn eufs_msgs__srv__Register_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Register_Request>);
    fn eufs_msgs__srv__Register_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Register_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Register_Request>) -> bool;
}

// Corresponds to eufs_msgs__srv__Register_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Register_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub node_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub severity: u8,

}



impl Default for Register_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__srv__Register_Request__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__srv__Register_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Register_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__Register_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__Register_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__Register_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Register_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Register_Request where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/srv/Register_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__srv__Register_Request() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__srv__Register_Response() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__srv__Register_Response__init(msg: *mut Register_Response) -> bool;
    fn eufs_msgs__srv__Register_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Register_Response>, size: usize) -> bool;
    fn eufs_msgs__srv__Register_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Register_Response>);
    fn eufs_msgs__srv__Register_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Register_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Register_Response>) -> bool;
}

// Corresponds to eufs_msgs__srv__Register_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Register_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: u8,

}



impl Default for Register_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__srv__Register_Response__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__srv__Register_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Register_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__Register_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__Register_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__Register_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Register_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Register_Response where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/srv/Register_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__srv__Register_Response() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__srv__SetCanState_Request() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__srv__SetCanState_Request__init(msg: *mut SetCanState_Request) -> bool;
    fn eufs_msgs__srv__SetCanState_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetCanState_Request>, size: usize) -> bool;
    fn eufs_msgs__srv__SetCanState_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetCanState_Request>);
    fn eufs_msgs__srv__SetCanState_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetCanState_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetCanState_Request>) -> bool;
}

// Corresponds to eufs_msgs__srv__SetCanState_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetCanState_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub ami_state: u16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub as_state: u16,

}



impl Default for SetCanState_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__srv__SetCanState_Request__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__srv__SetCanState_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetCanState_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__SetCanState_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__SetCanState_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__SetCanState_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetCanState_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetCanState_Request where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/srv/SetCanState_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__srv__SetCanState_Request() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__srv__SetCanState_Response() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__srv__SetCanState_Response__init(msg: *mut SetCanState_Response) -> bool;
    fn eufs_msgs__srv__SetCanState_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetCanState_Response>, size: usize) -> bool;
    fn eufs_msgs__srv__SetCanState_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetCanState_Response>);
    fn eufs_msgs__srv__SetCanState_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetCanState_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetCanState_Response>) -> bool;
}

// Corresponds to eufs_msgs__srv__SetCanState_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetCanState_Response {
    /// indicate successful run of triggered service
    pub success: bool,

    /// informational, e.g. for error messages
    pub message: rosidl_runtime_rs::String,

}



impl Default for SetCanState_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__srv__SetCanState_Response__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__srv__SetCanState_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetCanState_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__SetCanState_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__SetCanState_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__SetCanState_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetCanState_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetCanState_Response where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/srv/SetCanState_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__srv__SetCanState_Response() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__srv__SetString_Request() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__srv__SetString_Request__init(msg: *mut SetString_Request) -> bool;
    fn eufs_msgs__srv__SetString_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetString_Request>, size: usize) -> bool;
    fn eufs_msgs__srv__SetString_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetString_Request>);
    fn eufs_msgs__srv__SetString_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetString_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetString_Request>) -> bool;
}

// Corresponds to eufs_msgs__srv__SetString_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetString_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub data: rosidl_runtime_rs::String,

}



impl Default for SetString_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__srv__SetString_Request__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__srv__SetString_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetString_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__SetString_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__SetString_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__SetString_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetString_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetString_Request where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/srv/SetString_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__srv__SetString_Request() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__srv__SetString_Response() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__srv__SetString_Response__init(msg: *mut SetString_Response) -> bool;
    fn eufs_msgs__srv__SetString_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetString_Response>, size: usize) -> bool;
    fn eufs_msgs__srv__SetString_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetString_Response>);
    fn eufs_msgs__srv__SetString_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetString_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetString_Response>) -> bool;
}

// Corresponds to eufs_msgs__srv__SetString_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetString_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for SetString_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__srv__SetString_Response__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__srv__SetString_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetString_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__SetString_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__SetString_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__SetString_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetString_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetString_Response where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/srv/SetString_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__srv__SetString_Response() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__srv__SetTrack_Request() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__srv__SetTrack_Request__init(msg: *mut SetTrack_Request) -> bool;
    fn eufs_msgs__srv__SetTrack_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetTrack_Request>, size: usize) -> bool;
    fn eufs_msgs__srv__SetTrack_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetTrack_Request>);
    fn eufs_msgs__srv__SetTrack_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetTrack_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetTrack_Request>) -> bool;
}

// Corresponds to eufs_msgs__srv__SetTrack_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetTrack_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub data: super::super::msg::rmw::ConeArrayWithCovariance,

}



impl Default for SetTrack_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__srv__SetTrack_Request__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__srv__SetTrack_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetTrack_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__SetTrack_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__SetTrack_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__SetTrack_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetTrack_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetTrack_Request where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/srv/SetTrack_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__srv__SetTrack_Request() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__srv__SetTrack_Response() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__srv__SetTrack_Response__init(msg: *mut SetTrack_Response) -> bool;
    fn eufs_msgs__srv__SetTrack_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetTrack_Response>, size: usize) -> bool;
    fn eufs_msgs__srv__SetTrack_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetTrack_Response>);
    fn eufs_msgs__srv__SetTrack_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetTrack_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetTrack_Response>) -> bool;
}

// Corresponds to eufs_msgs__srv__SetTrack_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetTrack_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for SetTrack_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__srv__SetTrack_Response__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__srv__SetTrack_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetTrack_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__SetTrack_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__SetTrack_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__SetTrack_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetTrack_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetTrack_Response where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/srv/SetTrack_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__srv__SetTrack_Response() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__srv__SetMission_Request() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__srv__SetMission_Request__init(msg: *mut SetMission_Request) -> bool;
    fn eufs_msgs__srv__SetMission_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetMission_Request>, size: usize) -> bool;
    fn eufs_msgs__srv__SetMission_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetMission_Request>);
    fn eufs_msgs__srv__SetMission_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetMission_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetMission_Request>) -> bool;
}

// Corresponds to eufs_msgs__srv__SetMission_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetMission_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mission: i16,

}

impl SetMission_Request {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const NOT_SELECTED: i16 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const ACCELERATION: i16 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SKIDPAD: i16 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const AUTOCROSS: i16 = 3;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TRACK_DRIVE: i16 = 4;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const MANUAL_DRIVING: i16 = 5;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const ADS_EBS_TEST: i16 = 6;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const ADS_INSPECTION: i16 = 7;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const DDT_INSPECTION_A: i16 = 8;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const DDT_INSPECTION_B: i16 = 9;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const DDT_AUTONOMOUS_DEMO: i16 = 10;

}


impl Default for SetMission_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__srv__SetMission_Request__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__srv__SetMission_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetMission_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__SetMission_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__SetMission_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__SetMission_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetMission_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetMission_Request where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/srv/SetMission_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__srv__SetMission_Request() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__srv__SetMission_Response() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__srv__SetMission_Response__init(msg: *mut SetMission_Response) -> bool;
    fn eufs_msgs__srv__SetMission_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetMission_Response>, size: usize) -> bool;
    fn eufs_msgs__srv__SetMission_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetMission_Response>);
    fn eufs_msgs__srv__SetMission_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetMission_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetMission_Response>) -> bool;
}

// Corresponds to eufs_msgs__srv__SetMission_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetMission_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for SetMission_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__srv__SetMission_Response__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__srv__SetMission_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetMission_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__SetMission_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__SetMission_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__SetMission_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetMission_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetMission_Response where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/srv/SetMission_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__srv__SetMission_Response() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__srv__GetMap_Request() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__srv__GetMap_Request__init(msg: *mut GetMap_Request) -> bool;
    fn eufs_msgs__srv__GetMap_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetMap_Request>, size: usize) -> bool;
    fn eufs_msgs__srv__GetMap_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetMap_Request>);
    fn eufs_msgs__srv__GetMap_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetMap_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetMap_Request>) -> bool;
}

// Corresponds to eufs_msgs__srv__GetMap_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetMap_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetMap_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__srv__GetMap_Request__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__srv__GetMap_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetMap_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__GetMap_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__GetMap_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__GetMap_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetMap_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetMap_Request where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/srv/GetMap_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__srv__GetMap_Request() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__srv__GetMap_Response() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__srv__GetMap_Response__init(msg: *mut GetMap_Response) -> bool;
    fn eufs_msgs__srv__GetMap_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetMap_Response>, size: usize) -> bool;
    fn eufs_msgs__srv__GetMap_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetMap_Response>);
    fn eufs_msgs__srv__GetMap_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetMap_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetMap_Response>) -> bool;
}

// Corresponds to eufs_msgs__srv__GetMap_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetMap_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub map_path: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for GetMap_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__srv__GetMap_Response__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__srv__GetMap_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetMap_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__GetMap_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__GetMap_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__srv__GetMap_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetMap_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetMap_Response where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/srv/GetMap_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__srv__GetMap_Response() }
  }
}






#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__eufs_msgs__srv__RecordStart() -> *const std::ffi::c_void;
}

// Corresponds to eufs_msgs__srv__RecordStart
#[allow(missing_docs, non_camel_case_types)]
pub struct RecordStart;

impl rosidl_runtime_rs::Service for RecordStart {
    type Request = RecordStart_Request;
    type Response = RecordStart_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__eufs_msgs__srv__RecordStart() }
    }
}




#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__eufs_msgs__srv__RecordStop() -> *const std::ffi::c_void;
}

// Corresponds to eufs_msgs__srv__RecordStop
#[allow(missing_docs, non_camel_case_types)]
pub struct RecordStop;

impl rosidl_runtime_rs::Service for RecordStop {
    type Request = RecordStop_Request;
    type Response = RecordStop_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__eufs_msgs__srv__RecordStop() }
    }
}




#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__eufs_msgs__srv__Register() -> *const std::ffi::c_void;
}

// Corresponds to eufs_msgs__srv__Register
#[allow(missing_docs, non_camel_case_types)]
pub struct Register;

impl rosidl_runtime_rs::Service for Register {
    type Request = Register_Request;
    type Response = Register_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__eufs_msgs__srv__Register() }
    }
}




#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__eufs_msgs__srv__SetCanState() -> *const std::ffi::c_void;
}

// Corresponds to eufs_msgs__srv__SetCanState
#[allow(missing_docs, non_camel_case_types)]
pub struct SetCanState;

impl rosidl_runtime_rs::Service for SetCanState {
    type Request = SetCanState_Request;
    type Response = SetCanState_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__eufs_msgs__srv__SetCanState() }
    }
}




#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__eufs_msgs__srv__SetString() -> *const std::ffi::c_void;
}

// Corresponds to eufs_msgs__srv__SetString
#[allow(missing_docs, non_camel_case_types)]
pub struct SetString;

impl rosidl_runtime_rs::Service for SetString {
    type Request = SetString_Request;
    type Response = SetString_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__eufs_msgs__srv__SetString() }
    }
}




#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__eufs_msgs__srv__SetTrack() -> *const std::ffi::c_void;
}

// Corresponds to eufs_msgs__srv__SetTrack
#[allow(missing_docs, non_camel_case_types)]
pub struct SetTrack;

impl rosidl_runtime_rs::Service for SetTrack {
    type Request = SetTrack_Request;
    type Response = SetTrack_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__eufs_msgs__srv__SetTrack() }
    }
}




#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__eufs_msgs__srv__SetMission() -> *const std::ffi::c_void;
}

// Corresponds to eufs_msgs__srv__SetMission
#[allow(missing_docs, non_camel_case_types)]
pub struct SetMission;

impl rosidl_runtime_rs::Service for SetMission {
    type Request = SetMission_Request;
    type Response = SetMission_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__eufs_msgs__srv__SetMission() }
    }
}




#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__eufs_msgs__srv__GetMap() -> *const std::ffi::c_void;
}

// Corresponds to eufs_msgs__srv__GetMap
#[allow(missing_docs, non_camel_case_types)]
pub struct GetMap;

impl rosidl_runtime_rs::Service for GetMap {
    type Request = GetMap_Request;
    type Response = GetMap_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__eufs_msgs__srv__GetMap() }
    }
}


