
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__action__CheckForObjects_Goal() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__action__CheckForObjects_Goal__init(msg: *mut CheckForObjects_Goal) -> bool;
    fn eufs_msgs__action__CheckForObjects_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CheckForObjects_Goal>, size: usize) -> bool;
    fn eufs_msgs__action__CheckForObjects_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CheckForObjects_Goal>);
    fn eufs_msgs__action__CheckForObjects_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CheckForObjects_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<CheckForObjects_Goal>) -> bool;
}

// Corresponds to eufs_msgs__action__CheckForObjects_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CheckForObjects_Goal {
    /// Check if objects in image
    /// Goal definition
    pub id: i16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub image: sensor_msgs::msg::rmw::Image,

}



impl Default for CheckForObjects_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__action__CheckForObjects_Goal__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__action__CheckForObjects_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CheckForObjects_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__action__CheckForObjects_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__action__CheckForObjects_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__action__CheckForObjects_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CheckForObjects_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CheckForObjects_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/action/CheckForObjects_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__action__CheckForObjects_Goal() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__action__CheckForObjects_Result() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__action__CheckForObjects_Result__init(msg: *mut CheckForObjects_Result) -> bool;
    fn eufs_msgs__action__CheckForObjects_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CheckForObjects_Result>, size: usize) -> bool;
    fn eufs_msgs__action__CheckForObjects_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CheckForObjects_Result>);
    fn eufs_msgs__action__CheckForObjects_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CheckForObjects_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<CheckForObjects_Result>) -> bool;
}

// Corresponds to eufs_msgs__action__CheckForObjects_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CheckForObjects_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: i16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub bounding_boxes: super::super::msg::rmw::BoundingBoxes,

}



impl Default for CheckForObjects_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__action__CheckForObjects_Result__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__action__CheckForObjects_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CheckForObjects_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__action__CheckForObjects_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__action__CheckForObjects_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__action__CheckForObjects_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CheckForObjects_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CheckForObjects_Result where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/action/CheckForObjects_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__action__CheckForObjects_Result() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__action__CheckForObjects_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__action__CheckForObjects_Feedback__init(msg: *mut CheckForObjects_Feedback) -> bool;
    fn eufs_msgs__action__CheckForObjects_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CheckForObjects_Feedback>, size: usize) -> bool;
    fn eufs_msgs__action__CheckForObjects_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CheckForObjects_Feedback>);
    fn eufs_msgs__action__CheckForObjects_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CheckForObjects_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<CheckForObjects_Feedback>) -> bool;
}

// Corresponds to eufs_msgs__action__CheckForObjects_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CheckForObjects_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for CheckForObjects_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__action__CheckForObjects_Feedback__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__action__CheckForObjects_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CheckForObjects_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__action__CheckForObjects_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__action__CheckForObjects_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__action__CheckForObjects_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CheckForObjects_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CheckForObjects_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/action/CheckForObjects_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__action__CheckForObjects_Feedback() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__action__CheckForObjects_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__action__CheckForObjects_FeedbackMessage__init(msg: *mut CheckForObjects_FeedbackMessage) -> bool;
    fn eufs_msgs__action__CheckForObjects_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CheckForObjects_FeedbackMessage>, size: usize) -> bool;
    fn eufs_msgs__action__CheckForObjects_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CheckForObjects_FeedbackMessage>);
    fn eufs_msgs__action__CheckForObjects_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CheckForObjects_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<CheckForObjects_FeedbackMessage>) -> bool;
}

// Corresponds to eufs_msgs__action__CheckForObjects_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CheckForObjects_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::CheckForObjects_Feedback,

}



impl Default for CheckForObjects_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__action__CheckForObjects_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__action__CheckForObjects_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CheckForObjects_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__action__CheckForObjects_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__action__CheckForObjects_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__action__CheckForObjects_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CheckForObjects_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CheckForObjects_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/action/CheckForObjects_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__action__CheckForObjects_FeedbackMessage() }
  }
}




#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__action__CheckForObjects_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__action__CheckForObjects_SendGoal_Request__init(msg: *mut CheckForObjects_SendGoal_Request) -> bool;
    fn eufs_msgs__action__CheckForObjects_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CheckForObjects_SendGoal_Request>, size: usize) -> bool;
    fn eufs_msgs__action__CheckForObjects_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CheckForObjects_SendGoal_Request>);
    fn eufs_msgs__action__CheckForObjects_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CheckForObjects_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<CheckForObjects_SendGoal_Request>) -> bool;
}

// Corresponds to eufs_msgs__action__CheckForObjects_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CheckForObjects_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::CheckForObjects_Goal,

}



impl Default for CheckForObjects_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__action__CheckForObjects_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__action__CheckForObjects_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CheckForObjects_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__action__CheckForObjects_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__action__CheckForObjects_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__action__CheckForObjects_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CheckForObjects_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CheckForObjects_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/action/CheckForObjects_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__action__CheckForObjects_SendGoal_Request() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__action__CheckForObjects_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__action__CheckForObjects_SendGoal_Response__init(msg: *mut CheckForObjects_SendGoal_Response) -> bool;
    fn eufs_msgs__action__CheckForObjects_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CheckForObjects_SendGoal_Response>, size: usize) -> bool;
    fn eufs_msgs__action__CheckForObjects_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CheckForObjects_SendGoal_Response>);
    fn eufs_msgs__action__CheckForObjects_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CheckForObjects_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<CheckForObjects_SendGoal_Response>) -> bool;
}

// Corresponds to eufs_msgs__action__CheckForObjects_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CheckForObjects_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for CheckForObjects_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__action__CheckForObjects_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__action__CheckForObjects_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CheckForObjects_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__action__CheckForObjects_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__action__CheckForObjects_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__action__CheckForObjects_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CheckForObjects_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CheckForObjects_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/action/CheckForObjects_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__action__CheckForObjects_SendGoal_Response() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__action__CheckForObjects_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__action__CheckForObjects_GetResult_Request__init(msg: *mut CheckForObjects_GetResult_Request) -> bool;
    fn eufs_msgs__action__CheckForObjects_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CheckForObjects_GetResult_Request>, size: usize) -> bool;
    fn eufs_msgs__action__CheckForObjects_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CheckForObjects_GetResult_Request>);
    fn eufs_msgs__action__CheckForObjects_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CheckForObjects_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<CheckForObjects_GetResult_Request>) -> bool;
}

// Corresponds to eufs_msgs__action__CheckForObjects_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CheckForObjects_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for CheckForObjects_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__action__CheckForObjects_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__action__CheckForObjects_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CheckForObjects_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__action__CheckForObjects_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__action__CheckForObjects_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__action__CheckForObjects_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CheckForObjects_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CheckForObjects_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/action/CheckForObjects_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__action__CheckForObjects_GetResult_Request() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__action__CheckForObjects_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__action__CheckForObjects_GetResult_Response__init(msg: *mut CheckForObjects_GetResult_Response) -> bool;
    fn eufs_msgs__action__CheckForObjects_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CheckForObjects_GetResult_Response>, size: usize) -> bool;
    fn eufs_msgs__action__CheckForObjects_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CheckForObjects_GetResult_Response>);
    fn eufs_msgs__action__CheckForObjects_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CheckForObjects_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<CheckForObjects_GetResult_Response>) -> bool;
}

// Corresponds to eufs_msgs__action__CheckForObjects_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CheckForObjects_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::CheckForObjects_Result,

}



impl Default for CheckForObjects_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__action__CheckForObjects_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__action__CheckForObjects_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CheckForObjects_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__action__CheckForObjects_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__action__CheckForObjects_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__action__CheckForObjects_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CheckForObjects_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CheckForObjects_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/action/CheckForObjects_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__action__CheckForObjects_GetResult_Response() }
  }
}






#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__eufs_msgs__action__CheckForObjects_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to eufs_msgs__action__CheckForObjects_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct CheckForObjects_SendGoal;

impl rosidl_runtime_rs::Service for CheckForObjects_SendGoal {
    type Request = CheckForObjects_SendGoal_Request;
    type Response = CheckForObjects_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__eufs_msgs__action__CheckForObjects_SendGoal() }
    }
}




#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__eufs_msgs__action__CheckForObjects_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to eufs_msgs__action__CheckForObjects_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct CheckForObjects_GetResult;

impl rosidl_runtime_rs::Service for CheckForObjects_GetResult {
    type Request = CheckForObjects_GetResult_Request;
    type Response = CheckForObjects_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__eufs_msgs__action__CheckForObjects_GetResult() }
    }
}


