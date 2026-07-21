#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__BoundingBox() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__BoundingBox__init(msg: *mut BoundingBox) -> bool;
    fn eufs_msgs__msg__BoundingBox__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BoundingBox>, size: usize) -> bool;
    fn eufs_msgs__msg__BoundingBox__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BoundingBox>);
    fn eufs_msgs__msg__BoundingBox__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BoundingBox>, out_seq: *mut rosidl_runtime_rs::Sequence<BoundingBox>) -> bool;
}

// Corresponds to eufs_msgs__msg__BoundingBox
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Enum of types for the message.
/// PIXEL = use the pixel coordinates for xmin, xmax....
/// PERCENTAGE = use the percentage into the frame for xmin, xmax....

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BoundingBox {

    // This member is not documented.
    #[allow(missing_docs)]
    pub color: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub probability: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub type_: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub xmin: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ymin: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub xmax: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ymax: f64,

}

impl BoundingBox {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PIXEL: i32 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PERCENTAGE: i32 = 1;

}


impl Default for BoundingBox {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__BoundingBox__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__BoundingBox__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BoundingBox {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__BoundingBox__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__BoundingBox__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__BoundingBox__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BoundingBox {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BoundingBox where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/BoundingBox";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__BoundingBox() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__BoundingBoxes() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__BoundingBoxes__init(msg: *mut BoundingBoxes) -> bool;
    fn eufs_msgs__msg__BoundingBoxes__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BoundingBoxes>, size: usize) -> bool;
    fn eufs_msgs__msg__BoundingBoxes__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BoundingBoxes>);
    fn eufs_msgs__msg__BoundingBoxes__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BoundingBoxes>, out_seq: *mut rosidl_runtime_rs::Sequence<BoundingBoxes>) -> bool;
}

// Corresponds to eufs_msgs__msg__BoundingBoxes
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BoundingBoxes {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub image_header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub bounding_boxes: rosidl_runtime_rs::Sequence<super::super::msg::rmw::BoundingBox>,

}



impl Default for BoundingBoxes {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__BoundingBoxes__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__BoundingBoxes__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BoundingBoxes {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__BoundingBoxes__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__BoundingBoxes__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__BoundingBoxes__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BoundingBoxes {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BoundingBoxes where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/BoundingBoxes";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__BoundingBoxes() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__CanState() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__CanState__init(msg: *mut CanState) -> bool;
    fn eufs_msgs__msg__CanState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CanState>, size: usize) -> bool;
    fn eufs_msgs__msg__CanState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CanState>);
    fn eufs_msgs__msg__CanState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CanState>, out_seq: *mut rosidl_runtime_rs::Sequence<CanState>) -> bool;
}

// Corresponds to eufs_msgs__msg__CanState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// State of the Autonomous System

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CanState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub as_state: u16,

    /// Mission indicator
    pub ami_state: u16,

}

impl CanState {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const AS_OFF: u16 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const AS_READY: u16 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const AS_DRIVING: u16 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const AS_EMERGENCY_BRAKE: u16 = 3;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const AS_FINISHED: u16 = 4;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const AMI_NOT_SELECTED: u16 = 10;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const AMI_ACCELERATION: u16 = 11;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const AMI_SKIDPAD: u16 = 12;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const AMI_AUTOCROSS: u16 = 13;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const AMI_TRACK_DRIVE: u16 = 14;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const AMI_AUTONOMOUS_DEMO: u16 = 15;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const AMI_ADS_INSPECTION: u16 = 16;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const AMI_ADS_EBS: u16 = 17;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const AMI_DDT_INSPECTION_A: u16 = 18;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const AMI_DDT_INSPECTION_B: u16 = 19;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const AMI_JOYSTICK: u16 = 20;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const AMI_MANUAL: u16 = 21;

}


impl Default for CanState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__CanState__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__CanState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CanState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__CanState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__CanState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__CanState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CanState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CanState where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/CanState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__CanState() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__CarForces() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__CarForces__init(msg: *mut CarForces) -> bool;
    fn eufs_msgs__msg__CarForces__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CarForces>, size: usize) -> bool;
    fn eufs_msgs__msg__CarForces__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CarForces>);
    fn eufs_msgs__msg__CarForces__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CarForces>, out_seq: *mut rosidl_runtime_rs::Sequence<CarForces>) -> bool;
}

// Corresponds to eufs_msgs__msg__CarForces
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// A message containing various forces on the car

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarForces {
    /// Car tyre forces
    pub front_left_lateral: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub front_left_longitudinal: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub front_left_vertical: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub front_right_lateral: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub front_right_longitudinal: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub front_right_vertical: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rear_left_lateral: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rear_left_longitudinal: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rear_left_vertical: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rear_right_lateral: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rear_right_longitudinal: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rear_right_vertical: f64,

}



impl Default for CarForces {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__CarForces__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__CarForces__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CarForces {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__CarForces__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__CarForces__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__CarForces__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CarForces {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CarForces where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/CarForces";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__CarForces() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__CarState() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__CarState__init(msg: *mut CarState) -> bool;
    fn eufs_msgs__msg__CarState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CarState>, size: usize) -> bool;
    fn eufs_msgs__msg__CarState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CarState>);
    fn eufs_msgs__msg__CarState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CarState>, out_seq: *mut rosidl_runtime_rs::Sequence<CarState>) -> bool;
}

// Corresponds to eufs_msgs__msg__CarState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message includes both the kinematic and the dynamic state of the vehicle
/// All values are in SI units

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// aka frame of the car
    pub child_frame_id: rosidl_runtime_rs::String,

    /// Kinematic State (x_k) in header frame
    pub pose: geometry_msgs::msg::rmw::PoseWithCovariance,

    /// Dynamic State (x_d) in child_frame_id
    /// m/s
    pub twist: geometry_msgs::msg::rmw::TwistWithCovariance,

    /// m/s^2
    pub linear_acceleration: geometry_msgs::msg::rmw::Vector3,

    /// Row major x, y z in (m/s^2)
    pub linear_acceleration_covariance: [f64; 9],


    // This member is not documented.
    #[allow(missing_docs)]
    pub slip_angle: f64,

    /// Metadata
    /// in Volts (V)
    pub state_of_charge: f64,

}



impl Default for CarState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__CarState__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__CarState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CarState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__CarState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__CarState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__CarState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CarState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CarState where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/CarState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__CarState() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__ChassisCommand() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__ChassisCommand__init(msg: *mut ChassisCommand) -> bool;
    fn eufs_msgs__msg__ChassisCommand__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ChassisCommand>, size: usize) -> bool;
    fn eufs_msgs__msg__ChassisCommand__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ChassisCommand>);
    fn eufs_msgs__msg__ChassisCommand__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ChassisCommand>, out_seq: *mut rosidl_runtime_rs::Sequence<ChassisCommand>) -> bool;
}

// Corresponds to eufs_msgs__msg__ChassisCommand
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ChassisCommand {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sender: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub throttle: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub steering: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub front_brake: f64,

}



impl Default for ChassisCommand {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__ChassisCommand__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__ChassisCommand__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ChassisCommand {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ChassisCommand__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ChassisCommand__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ChassisCommand__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ChassisCommand {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ChassisCommand where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/ChassisCommand";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__ChassisCommand() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__ChassisState() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__ChassisState__init(msg: *mut ChassisState) -> bool;
    fn eufs_msgs__msg__ChassisState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ChassisState>, size: usize) -> bool;
    fn eufs_msgs__msg__ChassisState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ChassisState>);
    fn eufs_msgs__msg__ChassisState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ChassisState>, out_seq: *mut rosidl_runtime_rs::Sequence<ChassisState>) -> bool;
}

// Corresponds to eufs_msgs__msg__ChassisState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ChassisState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub throttle_relay_enabled: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub autonomous_enabled: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub runstop_motion_enabled: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub steering_commander: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub steering: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub throttle_commander: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub throttle: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub front_brake_commander: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub front_brake: f64,

}



impl Default for ChassisState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__ChassisState__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__ChassisState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ChassisState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ChassisState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ChassisState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ChassisState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ChassisState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ChassisState where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/ChassisState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__ChassisState() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__ConeArray() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__ConeArray__init(msg: *mut ConeArray) -> bool;
    fn eufs_msgs__msg__ConeArray__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ConeArray>, size: usize) -> bool;
    fn eufs_msgs__msg__ConeArray__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ConeArray>);
    fn eufs_msgs__msg__ConeArray__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ConeArray>, out_seq: *mut rosidl_runtime_rs::Sequence<ConeArray>) -> bool;
}

// Corresponds to eufs_msgs__msg__ConeArray
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Array of 2D cone locations (z = 0)

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConeArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub blue_cones: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::Point>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yellow_cones: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::Point>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub orange_cones: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::Point>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub big_orange_cones: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::Point>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub unknown_color_cones: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::Point>,

}



impl Default for ConeArray {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__ConeArray__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__ConeArray__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ConeArray {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ConeArray__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ConeArray__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ConeArray__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ConeArray {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ConeArray where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/ConeArray";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__ConeArray() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__ConeArrayWithCovariance() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__ConeArrayWithCovariance__init(msg: *mut ConeArrayWithCovariance) -> bool;
    fn eufs_msgs__msg__ConeArrayWithCovariance__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ConeArrayWithCovariance>, size: usize) -> bool;
    fn eufs_msgs__msg__ConeArrayWithCovariance__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ConeArrayWithCovariance>);
    fn eufs_msgs__msg__ConeArrayWithCovariance__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ConeArrayWithCovariance>, out_seq: *mut rosidl_runtime_rs::Sequence<ConeArrayWithCovariance>) -> bool;
}

// Corresponds to eufs_msgs__msg__ConeArrayWithCovariance
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Array of 2D cone locations (z = 0) with covariances

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConeArrayWithCovariance {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub blue_cones: rosidl_runtime_rs::Sequence<super::super::msg::rmw::ConeWithCovariance>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yellow_cones: rosidl_runtime_rs::Sequence<super::super::msg::rmw::ConeWithCovariance>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub orange_cones: rosidl_runtime_rs::Sequence<super::super::msg::rmw::ConeWithCovariance>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub big_orange_cones: rosidl_runtime_rs::Sequence<super::super::msg::rmw::ConeWithCovariance>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub unknown_color_cones: rosidl_runtime_rs::Sequence<super::super::msg::rmw::ConeWithCovariance>,

}



impl Default for ConeArrayWithCovariance {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__ConeArrayWithCovariance__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__ConeArrayWithCovariance__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ConeArrayWithCovariance {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ConeArrayWithCovariance__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ConeArrayWithCovariance__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ConeArrayWithCovariance__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ConeArrayWithCovariance {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ConeArrayWithCovariance where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/ConeArrayWithCovariance";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__ConeArrayWithCovariance() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__ConeWithColorProbability() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__ConeWithColorProbability__init(msg: *mut ConeWithColorProbability) -> bool;
    fn eufs_msgs__msg__ConeWithColorProbability__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ConeWithColorProbability>, size: usize) -> bool;
    fn eufs_msgs__msg__ConeWithColorProbability__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ConeWithColorProbability>);
    fn eufs_msgs__msg__ConeWithColorProbability__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ConeWithColorProbability>, out_seq: *mut rosidl_runtime_rs::Sequence<ConeWithColorProbability>) -> bool;
}

// Corresponds to eufs_msgs__msg__ConeWithColorProbability
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// 2D cone locations (z = 0) with id, covariance and probabilities of each colour

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConeWithColorProbability {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub blue_prob: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yellow_prob: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub orange_prob: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub big_orange_prob: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub unknown_prob: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point: geometry_msgs::msg::rmw::Point,


    // This member is not documented.
    #[allow(missing_docs)]
    pub covariance: [f64; 4],

}



impl Default for ConeWithColorProbability {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__ConeWithColorProbability__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__ConeWithColorProbability__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ConeWithColorProbability {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ConeWithColorProbability__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ConeWithColorProbability__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ConeWithColorProbability__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ConeWithColorProbability {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ConeWithColorProbability where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/ConeWithColorProbability";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__ConeWithColorProbability() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__ConeWithColorProbabilityArray() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__ConeWithColorProbabilityArray__init(msg: *mut ConeWithColorProbabilityArray) -> bool;
    fn eufs_msgs__msg__ConeWithColorProbabilityArray__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ConeWithColorProbabilityArray>, size: usize) -> bool;
    fn eufs_msgs__msg__ConeWithColorProbabilityArray__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ConeWithColorProbabilityArray>);
    fn eufs_msgs__msg__ConeWithColorProbabilityArray__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ConeWithColorProbabilityArray>, out_seq: *mut rosidl_runtime_rs::Sequence<ConeWithColorProbabilityArray>) -> bool;
}

// Corresponds to eufs_msgs__msg__ConeWithColorProbabilityArray
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Array of ConeWithColourProbability.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConeWithColorProbabilityArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cones: rosidl_runtime_rs::Sequence<super::super::msg::rmw::ConeWithColorProbability>,

}



impl Default for ConeWithColorProbabilityArray {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__ConeWithColorProbabilityArray__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__ConeWithColorProbabilityArray__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ConeWithColorProbabilityArray {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ConeWithColorProbabilityArray__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ConeWithColorProbabilityArray__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ConeWithColorProbabilityArray__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ConeWithColorProbabilityArray {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ConeWithColorProbabilityArray where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/ConeWithColorProbabilityArray";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__ConeWithColorProbabilityArray() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__ConeWithCovariance() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__ConeWithCovariance__init(msg: *mut ConeWithCovariance) -> bool;
    fn eufs_msgs__msg__ConeWithCovariance__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ConeWithCovariance>, size: usize) -> bool;
    fn eufs_msgs__msg__ConeWithCovariance__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ConeWithCovariance>);
    fn eufs_msgs__msg__ConeWithCovariance__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ConeWithCovariance>, out_seq: *mut rosidl_runtime_rs::Sequence<ConeWithCovariance>) -> bool;
}

// Corresponds to eufs_msgs__msg__ConeWithCovariance
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Cone information

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConeWithCovariance {

    // This member is not documented.
    #[allow(missing_docs)]
    pub point: geometry_msgs::msg::rmw::Point,


    // This member is not documented.
    #[allow(missing_docs)]
    pub covariance: [f64; 4],

}



impl Default for ConeWithCovariance {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__ConeWithCovariance__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__ConeWithCovariance__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ConeWithCovariance {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ConeWithCovariance__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ConeWithCovariance__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ConeWithCovariance__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ConeWithCovariance {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ConeWithCovariance where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/ConeWithCovariance";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__ConeWithCovariance() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__ConeAssociation() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__ConeAssociation__init(msg: *mut ConeAssociation) -> bool;
    fn eufs_msgs__msg__ConeAssociation__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ConeAssociation>, size: usize) -> bool;
    fn eufs_msgs__msg__ConeAssociation__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ConeAssociation>);
    fn eufs_msgs__msg__ConeAssociation__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ConeAssociation>, out_seq: *mut rosidl_runtime_rs::Sequence<ConeAssociation>) -> bool;
}

// Corresponds to eufs_msgs__msg__ConeAssociation
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Represents an link between two cones in the same frame

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConeAssociation {
    /// First cone in pair
    /// Typically a cone observed by perception
    pub first: super::super::msg::rmw::ConeWithCovariance,

    /// Second cone in pair
    /// Typically a cone in a map
    pub second: super::super::msg::rmw::ConeWithCovariance,

}



impl Default for ConeAssociation {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__ConeAssociation__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__ConeAssociation__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ConeAssociation {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ConeAssociation__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ConeAssociation__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ConeAssociation__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ConeAssociation {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ConeAssociation where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/ConeAssociation";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__ConeAssociation() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__ConeAssociationArray() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__ConeAssociationArray__init(msg: *mut ConeAssociationArray) -> bool;
    fn eufs_msgs__msg__ConeAssociationArray__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ConeAssociationArray>, size: usize) -> bool;
    fn eufs_msgs__msg__ConeAssociationArray__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ConeAssociationArray>);
    fn eufs_msgs__msg__ConeAssociationArray__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ConeAssociationArray>, out_seq: *mut rosidl_runtime_rs::Sequence<ConeAssociationArray>) -> bool;
}

// Corresponds to eufs_msgs__msg__ConeAssociationArray
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Represents a set of cone associations

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConeAssociationArray {
    /// Metadata relating to association type
    /// Association type
    pub type_: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub threshold: f64,

    /// Information relating to matched cones
    pub matched: rosidl_runtime_rs::Sequence<super::super::msg::rmw::ConeAssociation>,

    /// Information relating to unmatched cones
    pub unmatched: rosidl_runtime_rs::Sequence<super::super::msg::rmw::ConeWithCovariance>,

}



impl Default for ConeAssociationArray {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__ConeAssociationArray__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__ConeAssociationArray__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ConeAssociationArray {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ConeAssociationArray__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ConeAssociationArray__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ConeAssociationArray__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ConeAssociationArray {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ConeAssociationArray where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/ConeAssociationArray";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__ConeAssociationArray() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__ConeAssociationArrayArrayStamped() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__ConeAssociationArrayArrayStamped__init(msg: *mut ConeAssociationArrayArrayStamped) -> bool;
    fn eufs_msgs__msg__ConeAssociationArrayArrayStamped__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ConeAssociationArrayArrayStamped>, size: usize) -> bool;
    fn eufs_msgs__msg__ConeAssociationArrayArrayStamped__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ConeAssociationArrayArrayStamped>);
    fn eufs_msgs__msg__ConeAssociationArrayArrayStamped__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ConeAssociationArrayArrayStamped>, out_seq: *mut rosidl_runtime_rs::Sequence<ConeAssociationArrayArrayStamped>) -> bool;
}

// Corresponds to eufs_msgs__msg__ConeAssociationArrayArrayStamped
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConeAssociationArrayArrayStamped {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub associations: rosidl_runtime_rs::Sequence<super::super::msg::rmw::ConeAssociationArray>,

}



impl Default for ConeAssociationArrayArrayStamped {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__ConeAssociationArrayArrayStamped__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__ConeAssociationArrayArrayStamped__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ConeAssociationArrayArrayStamped {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ConeAssociationArrayArrayStamped__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ConeAssociationArrayArrayStamped__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ConeAssociationArrayArrayStamped__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ConeAssociationArrayArrayStamped {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ConeAssociationArrayArrayStamped where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/ConeAssociationArrayArrayStamped";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__ConeAssociationArrayArrayStamped() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__ConeAssociationArrayStamped() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__ConeAssociationArrayStamped__init(msg: *mut ConeAssociationArrayStamped) -> bool;
    fn eufs_msgs__msg__ConeAssociationArrayStamped__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ConeAssociationArrayStamped>, size: usize) -> bool;
    fn eufs_msgs__msg__ConeAssociationArrayStamped__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ConeAssociationArrayStamped>);
    fn eufs_msgs__msg__ConeAssociationArrayStamped__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ConeAssociationArrayStamped>, out_seq: *mut rosidl_runtime_rs::Sequence<ConeAssociationArrayStamped>) -> bool;
}

// Corresponds to eufs_msgs__msg__ConeAssociationArrayStamped
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Represents a set of cone associations

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConeAssociationArrayStamped {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub associations: super::super::msg::rmw::ConeAssociationArray,

}



impl Default for ConeAssociationArrayStamped {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__ConeAssociationArrayStamped__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__ConeAssociationArrayStamped__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ConeAssociationArrayStamped {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ConeAssociationArrayStamped__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ConeAssociationArrayStamped__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ConeAssociationArrayStamped__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ConeAssociationArrayStamped {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ConeAssociationArrayStamped where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/ConeAssociationArrayStamped";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__ConeAssociationArrayStamped() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__Costmap() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__Costmap__init(msg: *mut Costmap) -> bool;
    fn eufs_msgs__msg__Costmap__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Costmap>, size: usize) -> bool;
    fn eufs_msgs__msg__Costmap__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Costmap>);
    fn eufs_msgs__msg__Costmap__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Costmap>, out_seq: *mut rosidl_runtime_rs::Sequence<Costmap>) -> bool;
}

// Corresponds to eufs_msgs__msg__Costmap
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// costmap for the MPPI algorithm in the form of a 2D image

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Costmap {
    /// the bounds give the scale and limits of the image
    pub x_bounds_min: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub x_bounds_max: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y_bounds_min: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y_bounds_max: f64,

    /// he pixels per meter are used in scaling the image up
    pub pixels_per_meter: f64,

    /// the costmap contains 4 channels of floats
    pub channel0: rosidl_runtime_rs::Sequence<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub channel1: rosidl_runtime_rs::Sequence<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub channel2: rosidl_runtime_rs::Sequence<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub channel3: rosidl_runtime_rs::Sequence<f32>,

}



impl Default for Costmap {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__Costmap__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__Costmap__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Costmap {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__Costmap__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__Costmap__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__Costmap__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Costmap {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Costmap where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/Costmap";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__Costmap() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__CpuStatus() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__CpuStatus__init(msg: *mut CpuStatus) -> bool;
    fn eufs_msgs__msg__CpuStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CpuStatus>, size: usize) -> bool;
    fn eufs_msgs__msg__CpuStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CpuStatus>);
    fn eufs_msgs__msg__CpuStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CpuStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<CpuStatus>) -> bool;
}

// Corresponds to eufs_msgs__msg__CpuStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// constants

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CpuStatus {
    /// fields
    pub status: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub total: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub usr: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub nice: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sys: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub idle: f32,

}

impl CpuStatus {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const OK: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HIGH_LOAD: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const VERY_HIGH_LOAD: u8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STALE: u8 = 3;

}


impl Default for CpuStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__CpuStatus__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__CpuStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CpuStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__CpuStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__CpuStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__CpuStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CpuStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CpuStatus where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/CpuStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__CpuStatus() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__CpuUsage() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__CpuUsage__init(msg: *mut CpuUsage) -> bool;
    fn eufs_msgs__msg__CpuUsage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CpuUsage>, size: usize) -> bool;
    fn eufs_msgs__msg__CpuUsage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CpuUsage>);
    fn eufs_msgs__msg__CpuUsage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CpuUsage>, out_seq: *mut rosidl_runtime_rs::Sequence<CpuUsage>) -> bool;
}

// Corresponds to eufs_msgs__msg__CpuUsage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CpuUsage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,


    // This member is not documented.
    #[allow(missing_docs)]
    pub all: super::super::msg::rmw::CpuStatus,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cpus: rosidl_runtime_rs::Sequence<super::super::msg::rmw::CpuStatus>,

}



impl Default for CpuUsage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__CpuUsage__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__CpuUsage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CpuUsage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__CpuUsage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__CpuUsage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__CpuUsage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CpuUsage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CpuUsage where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/CpuUsage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__CpuUsage() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__EKFErr() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__EKFErr__init(msg: *mut EKFErr) -> bool;
    fn eufs_msgs__msg__EKFErr__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<EKFErr>, size: usize) -> bool;
    fn eufs_msgs__msg__EKFErr__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<EKFErr>);
    fn eufs_msgs__msg__EKFErr__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<EKFErr>, out_seq: *mut rosidl_runtime_rs::Sequence<EKFErr>) -> bool;
}

// Corresponds to eufs_msgs__msg__EKFErr
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Message for the error of the EKF. All of them are based on euclidean distances.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EKFErr {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub gps_x_vel_err: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub gps_y_vel_err: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub imu_x_acc_err: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub imu_y_acc_err: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub imu_yaw_err: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ekf_x_vel_var: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ekf_y_vel_var: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ekf_x_acc_var: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ekf_y_acc_var: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ekf_yaw_var: f64,

}



impl Default for EKFErr {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__EKFErr__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__EKFErr__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for EKFErr {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__EKFErr__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__EKFErr__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__EKFErr__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for EKFErr {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for EKFErr where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/EKFErr";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__EKFErr() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__EKFState() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__EKFState__init(msg: *mut EKFState) -> bool;
    fn eufs_msgs__msg__EKFState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<EKFState>, size: usize) -> bool;
    fn eufs_msgs__msg__EKFState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<EKFState>);
    fn eufs_msgs__msg__EKFState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<EKFState>, out_seq: *mut rosidl_runtime_rs::Sequence<EKFState>) -> bool;
}

// Corresponds to eufs_msgs__msg__EKFState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message contains information about the state of the EKF
/// and is meant for use by the state monitor to ensure the EKF
/// isn't failing.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EKFState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub gps_received: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub imu_received: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub wheel_odom_received: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ekf_odom_received: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ekf_accel_received: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub currently_over_covariance_limit: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub consecutive_turns_over_covariance_limit: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub recommends_failure: bool,

}



impl Default for EKFState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__EKFState__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__EKFState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for EKFState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__EKFState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__EKFState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__EKFState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for EKFState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for EKFState where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/EKFState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__EKFState() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__FullState() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__FullState__init(msg: *mut FullState) -> bool;
    fn eufs_msgs__msg__FullState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FullState>, size: usize) -> bool;
    fn eufs_msgs__msg__FullState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FullState>);
    fn eufs_msgs__msg__FullState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FullState>, out_seq: *mut rosidl_runtime_rs::Sequence<FullState>) -> bool;
}

// Corresponds to eufs_msgs__msg__FullState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FullState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub x_pos: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y_pos: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yaw: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub roll: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub u_x: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub u_y: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yaw_mder: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub front_throttle: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rear_throttle: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub steering: f64,

}



impl Default for FullState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__FullState__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__FullState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FullState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__FullState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__FullState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__FullState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FullState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FullState where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/FullState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__FullState() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__Heartbeat() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__Heartbeat__init(msg: *mut Heartbeat) -> bool;
    fn eufs_msgs__msg__Heartbeat__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Heartbeat>, size: usize) -> bool;
    fn eufs_msgs__msg__Heartbeat__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Heartbeat>);
    fn eufs_msgs__msg__Heartbeat__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Heartbeat>, out_seq: *mut rosidl_runtime_rs::Sequence<Heartbeat>) -> bool;
}

// Corresponds to eufs_msgs__msg__Heartbeat
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Heartbeat {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data: u8,

}



impl Default for Heartbeat {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__Heartbeat__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__Heartbeat__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Heartbeat {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__Heartbeat__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__Heartbeat__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__Heartbeat__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Heartbeat {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Heartbeat where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/Heartbeat";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__Heartbeat() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__IntegrationErr() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__IntegrationErr__init(msg: *mut IntegrationErr) -> bool;
    fn eufs_msgs__msg__IntegrationErr__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<IntegrationErr>, size: usize) -> bool;
    fn eufs_msgs__msg__IntegrationErr__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<IntegrationErr>);
    fn eufs_msgs__msg__IntegrationErr__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<IntegrationErr>, out_seq: *mut rosidl_runtime_rs::Sequence<IntegrationErr>) -> bool;
}

// Corresponds to eufs_msgs__msg__IntegrationErr
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Message for the error of Odometry Integration. Error based on euclidean distance.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IntegrationErr {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub position_err: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub orientation_err: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub linear_vel_err: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub angular_vel_err: f64,

}



impl Default for IntegrationErr {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__IntegrationErr__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__IntegrationErr__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for IntegrationErr {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__IntegrationErr__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__IntegrationErr__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__IntegrationErr__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for IntegrationErr {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for IntegrationErr where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/IntegrationErr";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__IntegrationErr() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__LapStats() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__LapStats__init(msg: *mut LapStats) -> bool;
    fn eufs_msgs__msg__LapStats__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LapStats>, size: usize) -> bool;
    fn eufs_msgs__msg__LapStats__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LapStats>);
    fn eufs_msgs__msg__LapStats__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LapStats>, out_seq: *mut rosidl_runtime_rs::Sequence<LapStats>) -> bool;
}

// Corresponds to eufs_msgs__msg__LapStats
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LapStats {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub lap_number: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub lap_time: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub avg_speed: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub max_speed: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub speed_var: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub max_slip: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub max_lateral_accel: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub normalized_deviation_mse: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub deviation_var: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub max_deviation: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub oscillation_number: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub num_cones_hit: i64,

}



impl Default for LapStats {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__LapStats__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__LapStats__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LapStats {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__LapStats__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__LapStats__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__LapStats__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LapStats {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LapStats where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/LapStats";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__LapStats() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__MPCState() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__MPCState__init(msg: *mut MPCState) -> bool;
    fn eufs_msgs__msg__MPCState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MPCState>, size: usize) -> bool;
    fn eufs_msgs__msg__MPCState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MPCState>);
    fn eufs_msgs__msg__MPCState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MPCState>, out_seq: *mut rosidl_runtime_rs::Sequence<MPCState>) -> bool;
}

// Corresponds to eufs_msgs__msg__MPCState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MPCState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub exitflag: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub iterations: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub solve_time: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cost: f64,

}



impl Default for MPCState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__MPCState__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__MPCState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MPCState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__MPCState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__MPCState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__MPCState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MPCState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MPCState where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/MPCState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__MPCState() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__NodeState() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__NodeState__init(msg: *mut NodeState) -> bool;
    fn eufs_msgs__msg__NodeState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NodeState>, size: usize) -> bool;
    fn eufs_msgs__msg__NodeState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NodeState>);
    fn eufs_msgs__msg__NodeState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NodeState>, out_seq: *mut rosidl_runtime_rs::Sequence<NodeState>) -> bool;
}

// Corresponds to eufs_msgs__msg__NodeState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// ID of the node

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NodeState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: u16,

    /// Name of the node
    pub name: rosidl_runtime_rs::String,

    /// Next expected heartbeat value
    pub exp_heartbeat: u8,

    /// Time of last heartbeat
    pub last_heartbeat: builtin_interfaces::msg::rmw::Time,

    /// Describes the action to take if node is offline
    pub severity: u8,

    /// Whether the node is running or not according to the status monitor
    pub online: bool,

}



impl Default for NodeState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__NodeState__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__NodeState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NodeState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__NodeState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__NodeState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__NodeState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NodeState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NodeState where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/NodeState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__NodeState() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__NodeStateArray() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__NodeStateArray__init(msg: *mut NodeStateArray) -> bool;
    fn eufs_msgs__msg__NodeStateArray__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NodeStateArray>, size: usize) -> bool;
    fn eufs_msgs__msg__NodeStateArray__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NodeStateArray>);
    fn eufs_msgs__msg__NodeStateArray__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NodeStateArray>, out_seq: *mut rosidl_runtime_rs::Sequence<NodeStateArray>) -> bool;
}

// Corresponds to eufs_msgs__msg__NodeStateArray
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NodeStateArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub states: rosidl_runtime_rs::Sequence<super::super::msg::rmw::NodeState>,

}



impl Default for NodeStateArray {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__NodeStateArray__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__NodeStateArray__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NodeStateArray {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__NodeStateArray__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__NodeStateArray__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__NodeStateArray__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NodeStateArray {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NodeStateArray where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/NodeStateArray";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__NodeStateArray() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__OSS() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__OSS__init(msg: *mut OSS) -> bool;
    fn eufs_msgs__msg__OSS__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<OSS>, size: usize) -> bool;
    fn eufs_msgs__msg__OSS__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<OSS>);
    fn eufs_msgs__msg__OSS__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<OSS>, out_seq: *mut rosidl_runtime_rs::Sequence<OSS>) -> bool;
}

// Corresponds to eufs_msgs__msg__OSS
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Measurements from the ground speed sensor

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct OSS {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub velocity: f32,

    /// velocity in X-axis
    pub v_x: f32,

    /// velocity in Y-axis
    pub v_y: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yaw_rate: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub distance: f32,

}



impl Default for OSS {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__OSS__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__OSS__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for OSS {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__OSS__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__OSS__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__OSS__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for OSS {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for OSS where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/OSS";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__OSS() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__Particle() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__Particle__init(msg: *mut Particle) -> bool;
    fn eufs_msgs__msg__Particle__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Particle>, size: usize) -> bool;
    fn eufs_msgs__msg__Particle__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Particle>);
    fn eufs_msgs__msg__Particle__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Particle>, out_seq: *mut rosidl_runtime_rs::Sequence<Particle>) -> bool;
}

// Corresponds to eufs_msgs__msg__Particle
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Represents the state of a single particle

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Particle {
    /// Pose associated with the particle
    pub pose: geometry_msgs::msg::rmw::Pose,

    /// Map associate with the particle
    pub map: super::super::msg::rmw::ConeArrayWithCovariance,

    /// Weighting placed on this particle
    pub weight: f64,

}



impl Default for Particle {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__Particle__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__Particle__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Particle {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__Particle__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__Particle__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__Particle__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Particle {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Particle where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/Particle";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__Particle() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__ParticleSLAM() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__ParticleSLAM__init(msg: *mut ParticleSLAM) -> bool;
    fn eufs_msgs__msg__ParticleSLAM__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ParticleSLAM>, size: usize) -> bool;
    fn eufs_msgs__msg__ParticleSLAM__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ParticleSLAM>);
    fn eufs_msgs__msg__ParticleSLAM__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ParticleSLAM>, out_seq: *mut rosidl_runtime_rs::Sequence<ParticleSLAM>) -> bool;
}

// Corresponds to eufs_msgs__msg__ParticleSLAM
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Represents the state of a particle-based SLAM algorithm

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ParticleSLAM {
    /// The index position current best guess of the SLAM algorithm in `particles`
    pub best_particle_idx: u64,

    /// The set of particles of the SLAM algorithm
    pub particles: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Particle>,

}



impl Default for ParticleSLAM {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__ParticleSLAM__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__ParticleSLAM__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ParticleSLAM {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ParticleSLAM__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ParticleSLAM__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ParticleSLAM__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ParticleSLAM {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ParticleSLAM where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/ParticleSLAM";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__ParticleSLAM() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__ParticleSLAMStamped() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__ParticleSLAMStamped__init(msg: *mut ParticleSLAMStamped) -> bool;
    fn eufs_msgs__msg__ParticleSLAMStamped__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ParticleSLAMStamped>, size: usize) -> bool;
    fn eufs_msgs__msg__ParticleSLAMStamped__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ParticleSLAMStamped>);
    fn eufs_msgs__msg__ParticleSLAMStamped__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ParticleSLAMStamped>, out_seq: *mut rosidl_runtime_rs::Sequence<ParticleSLAMStamped>) -> bool;
}

// Corresponds to eufs_msgs__msg__ParticleSLAMStamped
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// ParticleSLAM with reference frame and timestamp

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ParticleSLAMStamped {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub state: super::super::msg::rmw::ParticleSLAM,

}



impl Default for ParticleSLAMStamped {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__ParticleSLAMStamped__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__ParticleSLAMStamped__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ParticleSLAMStamped {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ParticleSLAMStamped__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ParticleSLAMStamped__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ParticleSLAMStamped__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ParticleSLAMStamped {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ParticleSLAMStamped where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/ParticleSLAMStamped";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__ParticleSLAMStamped() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__ParticleStamped() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__ParticleStamped__init(msg: *mut ParticleStamped) -> bool;
    fn eufs_msgs__msg__ParticleStamped__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ParticleStamped>, size: usize) -> bool;
    fn eufs_msgs__msg__ParticleStamped__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ParticleStamped>);
    fn eufs_msgs__msg__ParticleStamped__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ParticleStamped>, out_seq: *mut rosidl_runtime_rs::Sequence<ParticleStamped>) -> bool;
}

// Corresponds to eufs_msgs__msg__ParticleStamped
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Particle with reference coordinate frame and timestamp

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ParticleStamped {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub particle: super::super::msg::rmw::Particle,

}



impl Default for ParticleStamped {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__ParticleStamped__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__ParticleStamped__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ParticleStamped {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ParticleStamped__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ParticleStamped__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__ParticleStamped__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ParticleStamped {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ParticleStamped where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/ParticleStamped";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__ParticleStamped() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__PathIntegralParams() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__PathIntegralParams__init(msg: *mut PathIntegralParams) -> bool;
    fn eufs_msgs__msg__PathIntegralParams__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PathIntegralParams>, size: usize) -> bool;
    fn eufs_msgs__msg__PathIntegralParams__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PathIntegralParams>);
    fn eufs_msgs__msg__PathIntegralParams__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PathIntegralParams>, out_seq: *mut rosidl_runtime_rs::Sequence<PathIntegralParams>) -> bool;
}

// Corresponds to eufs_msgs__msg__PathIntegralParams
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PathIntegralParams {

    // This member is not documented.
    #[allow(missing_docs)]
    pub hz: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub num_timesteps: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub num_iters: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub gamma: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub init_steering: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub init_throttle: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub steering_var: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub throttle_var: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub max_throttle: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub speed_coefficient: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub track_coefficient: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub max_slip_angle: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub track_slop: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub crash_coeff: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub map_path: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub desired_speed: f64,

}



impl Default for PathIntegralParams {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__PathIntegralParams__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__PathIntegralParams__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PathIntegralParams {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__PathIntegralParams__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__PathIntegralParams__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__PathIntegralParams__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PathIntegralParams {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PathIntegralParams where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/PathIntegralParams";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__PathIntegralParams() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__PathIntegralStats() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__PathIntegralStats__init(msg: *mut PathIntegralStats) -> bool;
    fn eufs_msgs__msg__PathIntegralStats__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PathIntegralStats>, size: usize) -> bool;
    fn eufs_msgs__msg__PathIntegralStats__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PathIntegralStats>);
    fn eufs_msgs__msg__PathIntegralStats__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PathIntegralStats>, out_seq: *mut rosidl_runtime_rs::Sequence<PathIntegralStats>) -> bool;
}

// Corresponds to eufs_msgs__msg__PathIntegralStats
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PathIntegralStats {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tag: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub params: super::super::msg::rmw::PathIntegralParams,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stats: super::super::msg::rmw::LapStats,

}



impl Default for PathIntegralStats {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__PathIntegralStats__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__PathIntegralStats__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PathIntegralStats {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__PathIntegralStats__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__PathIntegralStats__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__PathIntegralStats__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PathIntegralStats {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PathIntegralStats where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/PathIntegralStats";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__PathIntegralStats() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__PathIntegralStatus() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__PathIntegralStatus__init(msg: *mut PathIntegralStatus) -> bool;
    fn eufs_msgs__msg__PathIntegralStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PathIntegralStatus>, size: usize) -> bool;
    fn eufs_msgs__msg__PathIntegralStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PathIntegralStatus>);
    fn eufs_msgs__msg__PathIntegralStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PathIntegralStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<PathIntegralStatus>) -> bool;
}

// Corresponds to eufs_msgs__msg__PathIntegralStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PathIntegralStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub info: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i32,

}



impl Default for PathIntegralStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__PathIntegralStatus__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__PathIntegralStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PathIntegralStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__PathIntegralStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__PathIntegralStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__PathIntegralStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PathIntegralStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PathIntegralStatus where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/PathIntegralStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__PathIntegralStatus() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__PathIntegralTiming() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__PathIntegralTiming__init(msg: *mut PathIntegralTiming) -> bool;
    fn eufs_msgs__msg__PathIntegralTiming__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PathIntegralTiming>, size: usize) -> bool;
    fn eufs_msgs__msg__PathIntegralTiming__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PathIntegralTiming>);
    fn eufs_msgs__msg__PathIntegralTiming__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PathIntegralTiming>, out_seq: *mut rosidl_runtime_rs::Sequence<PathIntegralTiming>) -> bool;
}

// Corresponds to eufs_msgs__msg__PathIntegralTiming
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PathIntegralTiming {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub average_time_between_poses: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub average_optimization_cycle_time: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub average_sleep_time: f64,

}



impl Default for PathIntegralTiming {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__PathIntegralTiming__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__PathIntegralTiming__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PathIntegralTiming {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__PathIntegralTiming__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__PathIntegralTiming__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__PathIntegralTiming__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PathIntegralTiming {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PathIntegralTiming where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/PathIntegralTiming";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__PathIntegralTiming() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__PlanningMode() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__PlanningMode__init(msg: *mut PlanningMode) -> bool;
    fn eufs_msgs__msg__PlanningMode__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PlanningMode>, size: usize) -> bool;
    fn eufs_msgs__msg__PlanningMode__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PlanningMode>);
    fn eufs_msgs__msg__PlanningMode__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PlanningMode>, out_seq: *mut rosidl_runtime_rs::Sequence<PlanningMode>) -> bool;
}

// Corresponds to eufs_msgs__msg__PlanningMode
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlanningMode {
    /// modes enuerated above
    pub mode: i8,

}

impl PlanningMode {
    /// Local planning mode
    pub const LOCAL: i8 = 0;

    /// Global planning mode
    pub const GLOBAL: i8 = 1;

}


impl Default for PlanningMode {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__PlanningMode__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__PlanningMode__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PlanningMode {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__PlanningMode__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__PlanningMode__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__PlanningMode__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PlanningMode {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PlanningMode where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/PlanningMode";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__PlanningMode() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__PointArray() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__PointArray__init(msg: *mut PointArray) -> bool;
    fn eufs_msgs__msg__PointArray__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PointArray>, size: usize) -> bool;
    fn eufs_msgs__msg__PointArray__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PointArray>);
    fn eufs_msgs__msg__PointArray__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PointArray>, out_seq: *mut rosidl_runtime_rs::Sequence<PointArray>) -> bool;
}

// Corresponds to eufs_msgs__msg__PointArray
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Simple msg for array of points

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PointArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub points: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::Point>,

}



impl Default for PointArray {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__PointArray__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__PointArray__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PointArray {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__PointArray__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__PointArray__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__PointArray__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PointArray {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PointArray where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/PointArray";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__PointArray() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__PointArrayStamped() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__PointArrayStamped__init(msg: *mut PointArrayStamped) -> bool;
    fn eufs_msgs__msg__PointArrayStamped__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PointArrayStamped>, size: usize) -> bool;
    fn eufs_msgs__msg__PointArrayStamped__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PointArrayStamped>);
    fn eufs_msgs__msg__PointArrayStamped__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PointArrayStamped>, out_seq: *mut rosidl_runtime_rs::Sequence<PointArrayStamped>) -> bool;
}

// Corresponds to eufs_msgs__msg__PointArrayStamped
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PointArrayStamped {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// Simple msg for array of points
    pub points: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::Point>,

}



impl Default for PointArrayStamped {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__PointArrayStamped__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__PointArrayStamped__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PointArrayStamped {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__PointArrayStamped__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__PointArrayStamped__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__PointArrayStamped__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PointArrayStamped {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PointArrayStamped where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/PointArrayStamped";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__PointArrayStamped() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__PurePursuitCheckpoint() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__PurePursuitCheckpoint__init(msg: *mut PurePursuitCheckpoint) -> bool;
    fn eufs_msgs__msg__PurePursuitCheckpoint__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PurePursuitCheckpoint>, size: usize) -> bool;
    fn eufs_msgs__msg__PurePursuitCheckpoint__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PurePursuitCheckpoint>);
    fn eufs_msgs__msg__PurePursuitCheckpoint__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PurePursuitCheckpoint>, out_seq: *mut rosidl_runtime_rs::Sequence<PurePursuitCheckpoint>) -> bool;
}

// Corresponds to eufs_msgs__msg__PurePursuitCheckpoint
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Checkpoint position [x, y, z]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PurePursuitCheckpoint {

    // This member is not documented.
    #[allow(missing_docs)]
    pub position: geometry_msgs::msg::rmw::Point,

    /// Maximum speed
    pub max_speed: f64,

    /// Maximum lateral acceleration
    pub max_lateral_acceleration: f64,

}



impl Default for PurePursuitCheckpoint {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__PurePursuitCheckpoint__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__PurePursuitCheckpoint__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PurePursuitCheckpoint {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__PurePursuitCheckpoint__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__PurePursuitCheckpoint__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__PurePursuitCheckpoint__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PurePursuitCheckpoint {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PurePursuitCheckpoint where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/PurePursuitCheckpoint";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__PurePursuitCheckpoint() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__PurePursuitCheckpointArrayStamped() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__PurePursuitCheckpointArrayStamped__init(msg: *mut PurePursuitCheckpointArrayStamped) -> bool;
    fn eufs_msgs__msg__PurePursuitCheckpointArrayStamped__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PurePursuitCheckpointArrayStamped>, size: usize) -> bool;
    fn eufs_msgs__msg__PurePursuitCheckpointArrayStamped__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PurePursuitCheckpointArrayStamped>);
    fn eufs_msgs__msg__PurePursuitCheckpointArrayStamped__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PurePursuitCheckpointArrayStamped>, out_seq: *mut rosidl_runtime_rs::Sequence<PurePursuitCheckpointArrayStamped>) -> bool;
}

// Corresponds to eufs_msgs__msg__PurePursuitCheckpointArrayStamped
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Stamped checkpoint array msg

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PurePursuitCheckpointArrayStamped {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub checkpoints: rosidl_runtime_rs::Sequence<super::super::msg::rmw::PurePursuitCheckpoint>,

}



impl Default for PurePursuitCheckpointArrayStamped {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__PurePursuitCheckpointArrayStamped__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__PurePursuitCheckpointArrayStamped__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PurePursuitCheckpointArrayStamped {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__PurePursuitCheckpointArrayStamped__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__PurePursuitCheckpointArrayStamped__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__PurePursuitCheckpointArrayStamped__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PurePursuitCheckpointArrayStamped {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PurePursuitCheckpointArrayStamped where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/PurePursuitCheckpointArrayStamped";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__PurePursuitCheckpointArrayStamped() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__Runstop() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__Runstop__init(msg: *mut Runstop) -> bool;
    fn eufs_msgs__msg__Runstop__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Runstop>, size: usize) -> bool;
    fn eufs_msgs__msg__Runstop__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Runstop>);
    fn eufs_msgs__msg__Runstop__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Runstop>, out_seq: *mut rosidl_runtime_rs::Sequence<Runstop>) -> bool;
}

// Corresponds to eufs_msgs__msg__Runstop
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Runstop {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sender: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub motion_enabled: bool,

}



impl Default for Runstop {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__Runstop__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__Runstop__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Runstop {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__Runstop__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__Runstop__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__Runstop__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Runstop {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Runstop where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/Runstop";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__Runstop() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__SLAMErr() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__SLAMErr__init(msg: *mut SLAMErr) -> bool;
    fn eufs_msgs__msg__SLAMErr__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SLAMErr>, size: usize) -> bool;
    fn eufs_msgs__msg__SLAMErr__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SLAMErr>);
    fn eufs_msgs__msg__SLAMErr__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SLAMErr>, out_seq: *mut rosidl_runtime_rs::Sequence<SLAMErr>) -> bool;
}

// Corresponds to eufs_msgs__msg__SLAMErr
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Message for the error of our SLAM algorithm. All of them are based on euclidean distances, besides the map similarity,
/// which can be interpreted as a percentage of how well our algorithm describes the map.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SLAMErr {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub x_err: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y_err: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub z_err: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub x_orient_err: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y_orient_err: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub z_orient_err: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub w_orient_err: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub map_similarity: f64,

}



impl Default for SLAMErr {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__SLAMErr__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__SLAMErr__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SLAMErr {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__SLAMErr__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__SLAMErr__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__SLAMErr__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SLAMErr {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SLAMErr where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/SLAMErr";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__SLAMErr() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__SLAMState() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__SLAMState__init(msg: *mut SLAMState) -> bool;
    fn eufs_msgs__msg__SLAMState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SLAMState>, size: usize) -> bool;
    fn eufs_msgs__msg__SLAMState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SLAMState>);
    fn eufs_msgs__msg__SLAMState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SLAMState>, out_seq: *mut rosidl_runtime_rs::Sequence<SLAMState>) -> bool;
}

// Corresponds to eufs_msgs__msg__SLAMState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SLAMState {
    /// True after loop closure
    pub loop_closed: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub laps: i16,

    /// Dump for any logging purposes. Eg: "particle covariance high; can't localise"
    pub status: rosidl_runtime_rs::String,

    /// States enumerated above
    pub state: i8,

}

impl SLAMState {
    /// Possible states of the algorithm
    /// No inputs have been received yet
    pub const NO_INPUTS: i8 = 0;

    /// Building map
    pub const MAPPING: i8 = 1;

    /// Loop closed, now localising only
    pub const LOCALISING: i8 = 2;

    /// Input topics timedout
    pub const TIMEOUT: i8 = 3;

    /// If true, critical failure!  Recommends emergency shutdown
    pub const RECOMMENDS_FAILURE: i8 = 4;

}


impl Default for SLAMState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__SLAMState__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__SLAMState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SLAMState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__SLAMState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__SLAMState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__SLAMState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SLAMState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SLAMState where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/SLAMState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__SLAMState() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__StateMachineState() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__StateMachineState__init(msg: *mut StateMachineState) -> bool;
    fn eufs_msgs__msg__StateMachineState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<StateMachineState>, size: usize) -> bool;
    fn eufs_msgs__msg__StateMachineState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<StateMachineState>);
    fn eufs_msgs__msg__StateMachineState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<StateMachineState>, out_seq: *mut rosidl_runtime_rs::Sequence<StateMachineState>) -> bool;
}

// Corresponds to eufs_msgs__msg__StateMachineState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// State of the EUFS State Machine

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StateMachineState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub state: u16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub state_str: rosidl_runtime_rs::String,

}



impl Default for StateMachineState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__StateMachineState__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__StateMachineState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for StateMachineState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__StateMachineState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__StateMachineState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__StateMachineState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for StateMachineState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for StateMachineState where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/StateMachineState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__StateMachineState() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__StereoImage() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__StereoImage__init(msg: *mut StereoImage) -> bool;
    fn eufs_msgs__msg__StereoImage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<StereoImage>, size: usize) -> bool;
    fn eufs_msgs__msg__StereoImage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<StereoImage>);
    fn eufs_msgs__msg__StereoImage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<StereoImage>, out_seq: *mut rosidl_runtime_rs::Sequence<StereoImage>) -> bool;
}

// Corresponds to eufs_msgs__msg__StereoImage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StereoImage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub color: sensor_msgs::msg::rmw::Image,


    // This member is not documented.
    #[allow(missing_docs)]
    pub depth: sensor_msgs::msg::rmw::Image,

}



impl Default for StereoImage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__StereoImage__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__StereoImage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for StereoImage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__StereoImage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__StereoImage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__StereoImage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for StereoImage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for StereoImage where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/StereoImage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__StereoImage() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__SystemStatus() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__SystemStatus__init(msg: *mut SystemStatus) -> bool;
    fn eufs_msgs__msg__SystemStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SystemStatus>, size: usize) -> bool;
    fn eufs_msgs__msg__SystemStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SystemStatus>);
    fn eufs_msgs__msg__SystemStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SystemStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<SystemStatus>) -> bool;
}

// Corresponds to eufs_msgs__msg__SystemStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SystemStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub topic_statuses: rosidl_runtime_rs::Sequence<super::super::msg::rmw::TopicStatus>,

}



impl Default for SystemStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__SystemStatus__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__SystemStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SystemStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__SystemStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__SystemStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__SystemStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SystemStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SystemStatus where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/SystemStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__SystemStatus() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__TopicStatus() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__TopicStatus__init(msg: *mut TopicStatus) -> bool;
    fn eufs_msgs__msg__TopicStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TopicStatus>, size: usize) -> bool;
    fn eufs_msgs__msg__TopicStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TopicStatus>);
    fn eufs_msgs__msg__TopicStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TopicStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<TopicStatus>) -> bool;
}

// Corresponds to eufs_msgs__msg__TopicStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Topic information

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TopicStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub topic: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub description: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub group: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub trigger_ebs: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub log_level: rosidl_runtime_rs::String,

    /// Topic status
    pub status: u16,

}

impl TopicStatus {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const OFF: u16 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PUBLISHING: u16 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TIMEOUT_EXCEEDED: u16 = 2;

}


impl Default for TopicStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__TopicStatus__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__TopicStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TopicStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__TopicStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__TopicStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__TopicStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TopicStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TopicStatus where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/TopicStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__TopicStatus() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__VehicleCommands() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__VehicleCommands__init(msg: *mut VehicleCommands) -> bool;
    fn eufs_msgs__msg__VehicleCommands__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<VehicleCommands>, size: usize) -> bool;
    fn eufs_msgs__msg__VehicleCommands__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<VehicleCommands>);
    fn eufs_msgs__msg__VehicleCommands__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<VehicleCommands>, out_seq: *mut rosidl_runtime_rs::Sequence<VehicleCommands>) -> bool;
}

// Corresponds to eufs_msgs__msg__VehicleCommands
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VehicleCommands {

    // This member is not documented.
    #[allow(missing_docs)]
    pub handshake: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ebs: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub direction: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub mission_status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub braking: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub torque: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub steering: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rpm: f64,

}



impl Default for VehicleCommands {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__VehicleCommands__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__VehicleCommands__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for VehicleCommands {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__VehicleCommands__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__VehicleCommands__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__VehicleCommands__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for VehicleCommands {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for VehicleCommands where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/VehicleCommands";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__VehicleCommands() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__VehicleCommandsStamped() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__VehicleCommandsStamped__init(msg: *mut VehicleCommandsStamped) -> bool;
    fn eufs_msgs__msg__VehicleCommandsStamped__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<VehicleCommandsStamped>, size: usize) -> bool;
    fn eufs_msgs__msg__VehicleCommandsStamped__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<VehicleCommandsStamped>);
    fn eufs_msgs__msg__VehicleCommandsStamped__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<VehicleCommandsStamped>, out_seq: *mut rosidl_runtime_rs::Sequence<VehicleCommandsStamped>) -> bool;
}

// Corresponds to eufs_msgs__msg__VehicleCommandsStamped
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VehicleCommandsStamped {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub commands: super::super::msg::rmw::VehicleCommands,

}



impl Default for VehicleCommandsStamped {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__VehicleCommandsStamped__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__VehicleCommandsStamped__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for VehicleCommandsStamped {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__VehicleCommandsStamped__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__VehicleCommandsStamped__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__VehicleCommandsStamped__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for VehicleCommandsStamped {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for VehicleCommandsStamped where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/VehicleCommandsStamped";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__VehicleCommandsStamped() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__Waypoint() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__Waypoint__init(msg: *mut Waypoint) -> bool;
    fn eufs_msgs__msg__Waypoint__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Waypoint>, size: usize) -> bool;
    fn eufs_msgs__msg__Waypoint__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Waypoint>);
    fn eufs_msgs__msg__Waypoint__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Waypoint>, out_seq: *mut rosidl_runtime_rs::Sequence<Waypoint>) -> bool;
}

// Corresponds to eufs_msgs__msg__Waypoint
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Waypoint position

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Waypoint {

    // This member is not documented.
    #[allow(missing_docs)]
    pub position: geometry_msgs::msg::rmw::Point,

    /// Suggested forward velocity (x direction in car frame)
    /// m/s
    pub speed: f64,

    /// Suggested steering angle
    /// rad
    pub suggested_steering: f64,

}



impl Default for Waypoint {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__Waypoint__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__Waypoint__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Waypoint {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__Waypoint__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__Waypoint__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__Waypoint__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Waypoint {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Waypoint where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/Waypoint";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__Waypoint() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__WaypointArrayStamped() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__WaypointArrayStamped__init(msg: *mut WaypointArrayStamped) -> bool;
    fn eufs_msgs__msg__WaypointArrayStamped__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WaypointArrayStamped>, size: usize) -> bool;
    fn eufs_msgs__msg__WaypointArrayStamped__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WaypointArrayStamped>);
    fn eufs_msgs__msg__WaypointArrayStamped__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WaypointArrayStamped>, out_seq: *mut rosidl_runtime_rs::Sequence<WaypointArrayStamped>) -> bool;
}

// Corresponds to eufs_msgs__msg__WaypointArrayStamped
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message is used by planning nodes to advertise suggested trajectory waypoints
/// as well as suggested velocity and steering at each waypoint.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WaypointArrayStamped {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub waypoints: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Waypoint>,

}



impl Default for WaypointArrayStamped {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__WaypointArrayStamped__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__WaypointArrayStamped__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WaypointArrayStamped {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__WaypointArrayStamped__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__WaypointArrayStamped__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__WaypointArrayStamped__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WaypointArrayStamped {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WaypointArrayStamped where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/WaypointArrayStamped";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__WaypointArrayStamped() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__WheelOdometryErr() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__WheelOdometryErr__init(msg: *mut WheelOdometryErr) -> bool;
    fn eufs_msgs__msg__WheelOdometryErr__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WheelOdometryErr>, size: usize) -> bool;
    fn eufs_msgs__msg__WheelOdometryErr__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WheelOdometryErr>);
    fn eufs_msgs__msg__WheelOdometryErr__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WheelOdometryErr>, out_seq: *mut rosidl_runtime_rs::Sequence<WheelOdometryErr>) -> bool;
}

// Corresponds to eufs_msgs__msg__WheelOdometryErr
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Message for the error of Wheel Odometry. Error based on euclidean distance.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WheelOdometryErr {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub position_err: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub orientation_err: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub linear_vel_err: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub angular_vel_err: f64,

    /// Just a-axis motion, since that has special importance for Wheel Odometry
    pub forward_vel_err: f64,

}



impl Default for WheelOdometryErr {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__WheelOdometryErr__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__WheelOdometryErr__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WheelOdometryErr {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__WheelOdometryErr__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__WheelOdometryErr__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__WheelOdometryErr__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WheelOdometryErr {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WheelOdometryErr where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/WheelOdometryErr";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__WheelOdometryErr() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__WheelSpeeds() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__WheelSpeeds__init(msg: *mut WheelSpeeds) -> bool;
    fn eufs_msgs__msg__WheelSpeeds__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WheelSpeeds>, size: usize) -> bool;
    fn eufs_msgs__msg__WheelSpeeds__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WheelSpeeds>);
    fn eufs_msgs__msg__WheelSpeeds__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WheelSpeeds>, out_seq: *mut rosidl_runtime_rs::Sequence<WheelSpeeds>) -> bool;
}

// Corresponds to eufs_msgs__msg__WheelSpeeds
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WheelSpeeds {

    // This member is not documented.
    #[allow(missing_docs)]
    pub steering: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub lf_speed: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rf_speed: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub lb_speed: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rb_speed: f32,

}



impl Default for WheelSpeeds {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__WheelSpeeds__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__WheelSpeeds__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WheelSpeeds {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__WheelSpeeds__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__WheelSpeeds__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__WheelSpeeds__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WheelSpeeds {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WheelSpeeds where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/WheelSpeeds";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__WheelSpeeds() }
  }
}


#[link(name = "eufs_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__WheelSpeedsStamped() -> *const std::ffi::c_void;
}

#[link(name = "eufs_msgs__rosidl_generator_c")]
extern "C" {
    fn eufs_msgs__msg__WheelSpeedsStamped__init(msg: *mut WheelSpeedsStamped) -> bool;
    fn eufs_msgs__msg__WheelSpeedsStamped__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WheelSpeedsStamped>, size: usize) -> bool;
    fn eufs_msgs__msg__WheelSpeedsStamped__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WheelSpeedsStamped>);
    fn eufs_msgs__msg__WheelSpeedsStamped__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WheelSpeedsStamped>, out_seq: *mut rosidl_runtime_rs::Sequence<WheelSpeedsStamped>) -> bool;
}

// Corresponds to eufs_msgs__msg__WheelSpeedsStamped
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WheelSpeedsStamped {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub speeds: super::super::msg::rmw::WheelSpeeds,

}



impl Default for WheelSpeedsStamped {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !eufs_msgs__msg__WheelSpeedsStamped__init(&mut msg as *mut _) {
        panic!("Call to eufs_msgs__msg__WheelSpeedsStamped__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WheelSpeedsStamped {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__WheelSpeedsStamped__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__WheelSpeedsStamped__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { eufs_msgs__msg__WheelSpeedsStamped__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WheelSpeedsStamped {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WheelSpeedsStamped where Self: Sized {
  const TYPE_NAME: &'static str = "eufs_msgs/msg/WheelSpeedsStamped";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__eufs_msgs__msg__WheelSpeedsStamped() }
  }
}


