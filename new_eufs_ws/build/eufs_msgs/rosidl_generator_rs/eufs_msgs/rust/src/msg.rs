#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to eufs_msgs__msg__BoundingBox
/// Enum of types for the message.
/// PIXEL = use the pixel coordinates for xmin, xmax....
/// PERCENTAGE = use the percentage into the frame for xmin, xmax....

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BoundingBox {

    // This member is not documented.
    #[allow(missing_docs)]
    pub color: std::string::String,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::BoundingBox::default())
  }
}

impl rosidl_runtime_rs::Message for BoundingBox {
  type RmwMsg = super::msg::rmw::BoundingBox;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        color: msg.color.as_str().into(),
        probability: msg.probability,
        type_: msg.type_,
        xmin: msg.xmin,
        ymin: msg.ymin,
        xmax: msg.xmax,
        ymax: msg.ymax,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        color: msg.color.as_str().into(),
      probability: msg.probability,
      type_: msg.type_,
      xmin: msg.xmin,
      ymin: msg.ymin,
      xmax: msg.xmax,
      ymax: msg.ymax,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      color: msg.color.to_string(),
      probability: msg.probability,
      type_: msg.type_,
      xmin: msg.xmin,
      ymin: msg.ymin,
      xmax: msg.xmax,
      ymax: msg.ymax,
    }
  }
}


// Corresponds to eufs_msgs__msg__BoundingBoxes

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BoundingBoxes {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub image_header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub bounding_boxes: Vec<super::msg::BoundingBox>,

}



impl Default for BoundingBoxes {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::BoundingBoxes::default())
  }
}

impl rosidl_runtime_rs::Message for BoundingBoxes {
  type RmwMsg = super::msg::rmw::BoundingBoxes;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        image_header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.image_header)).into_owned(),
        bounding_boxes: msg.bounding_boxes
          .into_iter()
          .map(|elem| super::msg::BoundingBox::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        image_header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.image_header)).into_owned(),
        bounding_boxes: msg.bounding_boxes
          .iter()
          .map(|elem| super::msg::BoundingBox::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      image_header: std_msgs::msg::Header::from_rmw_message(msg.image_header),
      bounding_boxes: msg.bounding_boxes
          .into_iter()
          .map(super::msg::BoundingBox::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to eufs_msgs__msg__CanState
/// State of the Autonomous System

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CanState::default())
  }
}

impl rosidl_runtime_rs::Message for CanState {
  type RmwMsg = super::msg::rmw::CanState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        as_state: msg.as_state,
        ami_state: msg.ami_state,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      as_state: msg.as_state,
      ami_state: msg.ami_state,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      as_state: msg.as_state,
      ami_state: msg.ami_state,
    }
  }
}


// Corresponds to eufs_msgs__msg__CarForces
/// A message containing various forces on the car

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CarForces::default())
  }
}

impl rosidl_runtime_rs::Message for CarForces {
  type RmwMsg = super::msg::rmw::CarForces;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        front_left_lateral: msg.front_left_lateral,
        front_left_longitudinal: msg.front_left_longitudinal,
        front_left_vertical: msg.front_left_vertical,
        front_right_lateral: msg.front_right_lateral,
        front_right_longitudinal: msg.front_right_longitudinal,
        front_right_vertical: msg.front_right_vertical,
        rear_left_lateral: msg.rear_left_lateral,
        rear_left_longitudinal: msg.rear_left_longitudinal,
        rear_left_vertical: msg.rear_left_vertical,
        rear_right_lateral: msg.rear_right_lateral,
        rear_right_longitudinal: msg.rear_right_longitudinal,
        rear_right_vertical: msg.rear_right_vertical,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      front_left_lateral: msg.front_left_lateral,
      front_left_longitudinal: msg.front_left_longitudinal,
      front_left_vertical: msg.front_left_vertical,
      front_right_lateral: msg.front_right_lateral,
      front_right_longitudinal: msg.front_right_longitudinal,
      front_right_vertical: msg.front_right_vertical,
      rear_left_lateral: msg.rear_left_lateral,
      rear_left_longitudinal: msg.rear_left_longitudinal,
      rear_left_vertical: msg.rear_left_vertical,
      rear_right_lateral: msg.rear_right_lateral,
      rear_right_longitudinal: msg.rear_right_longitudinal,
      rear_right_vertical: msg.rear_right_vertical,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      front_left_lateral: msg.front_left_lateral,
      front_left_longitudinal: msg.front_left_longitudinal,
      front_left_vertical: msg.front_left_vertical,
      front_right_lateral: msg.front_right_lateral,
      front_right_longitudinal: msg.front_right_longitudinal,
      front_right_vertical: msg.front_right_vertical,
      rear_left_lateral: msg.rear_left_lateral,
      rear_left_longitudinal: msg.rear_left_longitudinal,
      rear_left_vertical: msg.rear_left_vertical,
      rear_right_lateral: msg.rear_right_lateral,
      rear_right_longitudinal: msg.rear_right_longitudinal,
      rear_right_vertical: msg.rear_right_vertical,
    }
  }
}


// Corresponds to eufs_msgs__msg__CarState
/// This message includes both the kinematic and the dynamic state of the vehicle
/// All values are in SI units

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// aka frame of the car
    pub child_frame_id: std::string::String,

    /// Kinematic State (x_k) in header frame
    pub pose: geometry_msgs::msg::PoseWithCovariance,

    /// Dynamic State (x_d) in child_frame_id
    /// m/s
    pub twist: geometry_msgs::msg::TwistWithCovariance,

    /// m/s^2
    pub linear_acceleration: geometry_msgs::msg::Vector3,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CarState::default())
  }
}

impl rosidl_runtime_rs::Message for CarState {
  type RmwMsg = super::msg::rmw::CarState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        child_frame_id: msg.child_frame_id.as_str().into(),
        pose: geometry_msgs::msg::PoseWithCovariance::into_rmw_message(std::borrow::Cow::Owned(msg.pose)).into_owned(),
        twist: geometry_msgs::msg::TwistWithCovariance::into_rmw_message(std::borrow::Cow::Owned(msg.twist)).into_owned(),
        linear_acceleration: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.linear_acceleration)).into_owned(),
        linear_acceleration_covariance: msg.linear_acceleration_covariance,
        slip_angle: msg.slip_angle,
        state_of_charge: msg.state_of_charge,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        child_frame_id: msg.child_frame_id.as_str().into(),
        pose: geometry_msgs::msg::PoseWithCovariance::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose)).into_owned(),
        twist: geometry_msgs::msg::TwistWithCovariance::into_rmw_message(std::borrow::Cow::Borrowed(&msg.twist)).into_owned(),
        linear_acceleration: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.linear_acceleration)).into_owned(),
        linear_acceleration_covariance: msg.linear_acceleration_covariance,
      slip_angle: msg.slip_angle,
      state_of_charge: msg.state_of_charge,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      child_frame_id: msg.child_frame_id.to_string(),
      pose: geometry_msgs::msg::PoseWithCovariance::from_rmw_message(msg.pose),
      twist: geometry_msgs::msg::TwistWithCovariance::from_rmw_message(msg.twist),
      linear_acceleration: geometry_msgs::msg::Vector3::from_rmw_message(msg.linear_acceleration),
      linear_acceleration_covariance: msg.linear_acceleration_covariance,
      slip_angle: msg.slip_angle,
      state_of_charge: msg.state_of_charge,
    }
  }
}


// Corresponds to eufs_msgs__msg__ChassisCommand

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ChassisCommand {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sender: std::string::String,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ChassisCommand::default())
  }
}

impl rosidl_runtime_rs::Message for ChassisCommand {
  type RmwMsg = super::msg::rmw::ChassisCommand;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        sender: msg.sender.as_str().into(),
        throttle: msg.throttle,
        steering: msg.steering,
        front_brake: msg.front_brake,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        sender: msg.sender.as_str().into(),
      throttle: msg.throttle,
      steering: msg.steering,
      front_brake: msg.front_brake,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      sender: msg.sender.to_string(),
      throttle: msg.throttle,
      steering: msg.steering,
      front_brake: msg.front_brake,
    }
  }
}


// Corresponds to eufs_msgs__msg__ChassisState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ChassisState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


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
    pub steering_commander: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub steering: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub throttle_commander: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub throttle: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub front_brake_commander: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub front_brake: f64,

}



impl Default for ChassisState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ChassisState::default())
  }
}

impl rosidl_runtime_rs::Message for ChassisState {
  type RmwMsg = super::msg::rmw::ChassisState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        throttle_relay_enabled: msg.throttle_relay_enabled,
        autonomous_enabled: msg.autonomous_enabled,
        runstop_motion_enabled: msg.runstop_motion_enabled,
        steering_commander: msg.steering_commander.as_str().into(),
        steering: msg.steering,
        throttle_commander: msg.throttle_commander.as_str().into(),
        throttle: msg.throttle,
        front_brake_commander: msg.front_brake_commander.as_str().into(),
        front_brake: msg.front_brake,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      throttle_relay_enabled: msg.throttle_relay_enabled,
      autonomous_enabled: msg.autonomous_enabled,
      runstop_motion_enabled: msg.runstop_motion_enabled,
        steering_commander: msg.steering_commander.as_str().into(),
      steering: msg.steering,
        throttle_commander: msg.throttle_commander.as_str().into(),
      throttle: msg.throttle,
        front_brake_commander: msg.front_brake_commander.as_str().into(),
      front_brake: msg.front_brake,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      throttle_relay_enabled: msg.throttle_relay_enabled,
      autonomous_enabled: msg.autonomous_enabled,
      runstop_motion_enabled: msg.runstop_motion_enabled,
      steering_commander: msg.steering_commander.to_string(),
      steering: msg.steering,
      throttle_commander: msg.throttle_commander.to_string(),
      throttle: msg.throttle,
      front_brake_commander: msg.front_brake_commander.to_string(),
      front_brake: msg.front_brake,
    }
  }
}


// Corresponds to eufs_msgs__msg__ConeArray
/// Array of 2D cone locations (z = 0)

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConeArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub blue_cones: Vec<geometry_msgs::msg::Point>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yellow_cones: Vec<geometry_msgs::msg::Point>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub orange_cones: Vec<geometry_msgs::msg::Point>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub big_orange_cones: Vec<geometry_msgs::msg::Point>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub unknown_color_cones: Vec<geometry_msgs::msg::Point>,

}



impl Default for ConeArray {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ConeArray::default())
  }
}

impl rosidl_runtime_rs::Message for ConeArray {
  type RmwMsg = super::msg::rmw::ConeArray;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        blue_cones: msg.blue_cones
          .into_iter()
          .map(|elem| geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        yellow_cones: msg.yellow_cones
          .into_iter()
          .map(|elem| geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        orange_cones: msg.orange_cones
          .into_iter()
          .map(|elem| geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        big_orange_cones: msg.big_orange_cones
          .into_iter()
          .map(|elem| geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        unknown_color_cones: msg.unknown_color_cones
          .into_iter()
          .map(|elem| geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        blue_cones: msg.blue_cones
          .iter()
          .map(|elem| geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        yellow_cones: msg.yellow_cones
          .iter()
          .map(|elem| geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        orange_cones: msg.orange_cones
          .iter()
          .map(|elem| geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        big_orange_cones: msg.big_orange_cones
          .iter()
          .map(|elem| geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        unknown_color_cones: msg.unknown_color_cones
          .iter()
          .map(|elem| geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      blue_cones: msg.blue_cones
          .into_iter()
          .map(geometry_msgs::msg::Point::from_rmw_message)
          .collect(),
      yellow_cones: msg.yellow_cones
          .into_iter()
          .map(geometry_msgs::msg::Point::from_rmw_message)
          .collect(),
      orange_cones: msg.orange_cones
          .into_iter()
          .map(geometry_msgs::msg::Point::from_rmw_message)
          .collect(),
      big_orange_cones: msg.big_orange_cones
          .into_iter()
          .map(geometry_msgs::msg::Point::from_rmw_message)
          .collect(),
      unknown_color_cones: msg.unknown_color_cones
          .into_iter()
          .map(geometry_msgs::msg::Point::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to eufs_msgs__msg__ConeArrayWithCovariance
/// Array of 2D cone locations (z = 0) with covariances

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConeArrayWithCovariance {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub blue_cones: Vec<super::msg::ConeWithCovariance>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yellow_cones: Vec<super::msg::ConeWithCovariance>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub orange_cones: Vec<super::msg::ConeWithCovariance>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub big_orange_cones: Vec<super::msg::ConeWithCovariance>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub unknown_color_cones: Vec<super::msg::ConeWithCovariance>,

}



impl Default for ConeArrayWithCovariance {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ConeArrayWithCovariance::default())
  }
}

impl rosidl_runtime_rs::Message for ConeArrayWithCovariance {
  type RmwMsg = super::msg::rmw::ConeArrayWithCovariance;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        blue_cones: msg.blue_cones
          .into_iter()
          .map(|elem| super::msg::ConeWithCovariance::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        yellow_cones: msg.yellow_cones
          .into_iter()
          .map(|elem| super::msg::ConeWithCovariance::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        orange_cones: msg.orange_cones
          .into_iter()
          .map(|elem| super::msg::ConeWithCovariance::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        big_orange_cones: msg.big_orange_cones
          .into_iter()
          .map(|elem| super::msg::ConeWithCovariance::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        unknown_color_cones: msg.unknown_color_cones
          .into_iter()
          .map(|elem| super::msg::ConeWithCovariance::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        blue_cones: msg.blue_cones
          .iter()
          .map(|elem| super::msg::ConeWithCovariance::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        yellow_cones: msg.yellow_cones
          .iter()
          .map(|elem| super::msg::ConeWithCovariance::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        orange_cones: msg.orange_cones
          .iter()
          .map(|elem| super::msg::ConeWithCovariance::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        big_orange_cones: msg.big_orange_cones
          .iter()
          .map(|elem| super::msg::ConeWithCovariance::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        unknown_color_cones: msg.unknown_color_cones
          .iter()
          .map(|elem| super::msg::ConeWithCovariance::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      blue_cones: msg.blue_cones
          .into_iter()
          .map(super::msg::ConeWithCovariance::from_rmw_message)
          .collect(),
      yellow_cones: msg.yellow_cones
          .into_iter()
          .map(super::msg::ConeWithCovariance::from_rmw_message)
          .collect(),
      orange_cones: msg.orange_cones
          .into_iter()
          .map(super::msg::ConeWithCovariance::from_rmw_message)
          .collect(),
      big_orange_cones: msg.big_orange_cones
          .into_iter()
          .map(super::msg::ConeWithCovariance::from_rmw_message)
          .collect(),
      unknown_color_cones: msg.unknown_color_cones
          .into_iter()
          .map(super::msg::ConeWithCovariance::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to eufs_msgs__msg__ConeWithColorProbability
/// 2D cone locations (z = 0) with id, covariance and probabilities of each colour

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    pub point: geometry_msgs::msg::Point,


    // This member is not documented.
    #[allow(missing_docs)]
    pub covariance: [f64; 4],

}



impl Default for ConeWithColorProbability {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ConeWithColorProbability::default())
  }
}

impl rosidl_runtime_rs::Message for ConeWithColorProbability {
  type RmwMsg = super::msg::rmw::ConeWithColorProbability;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id,
        blue_prob: msg.blue_prob,
        yellow_prob: msg.yellow_prob,
        orange_prob: msg.orange_prob,
        big_orange_prob: msg.big_orange_prob,
        unknown_prob: msg.unknown_prob,
        point: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Owned(msg.point)).into_owned(),
        covariance: msg.covariance,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      id: msg.id,
      blue_prob: msg.blue_prob,
      yellow_prob: msg.yellow_prob,
      orange_prob: msg.orange_prob,
      big_orange_prob: msg.big_orange_prob,
      unknown_prob: msg.unknown_prob,
        point: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Borrowed(&msg.point)).into_owned(),
        covariance: msg.covariance,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id: msg.id,
      blue_prob: msg.blue_prob,
      yellow_prob: msg.yellow_prob,
      orange_prob: msg.orange_prob,
      big_orange_prob: msg.big_orange_prob,
      unknown_prob: msg.unknown_prob,
      point: geometry_msgs::msg::Point::from_rmw_message(msg.point),
      covariance: msg.covariance,
    }
  }
}


// Corresponds to eufs_msgs__msg__ConeWithColorProbabilityArray
/// Array of ConeWithColourProbability.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConeWithColorProbabilityArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cones: Vec<super::msg::ConeWithColorProbability>,

}



impl Default for ConeWithColorProbabilityArray {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ConeWithColorProbabilityArray::default())
  }
}

impl rosidl_runtime_rs::Message for ConeWithColorProbabilityArray {
  type RmwMsg = super::msg::rmw::ConeWithColorProbabilityArray;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        cones: msg.cones
          .into_iter()
          .map(|elem| super::msg::ConeWithColorProbability::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        cones: msg.cones
          .iter()
          .map(|elem| super::msg::ConeWithColorProbability::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      cones: msg.cones
          .into_iter()
          .map(super::msg::ConeWithColorProbability::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to eufs_msgs__msg__ConeWithCovariance
/// Cone information

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConeWithCovariance {

    // This member is not documented.
    #[allow(missing_docs)]
    pub point: geometry_msgs::msg::Point,


    // This member is not documented.
    #[allow(missing_docs)]
    pub covariance: [f64; 4],

}



impl Default for ConeWithCovariance {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ConeWithCovariance::default())
  }
}

impl rosidl_runtime_rs::Message for ConeWithCovariance {
  type RmwMsg = super::msg::rmw::ConeWithCovariance;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        point: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Owned(msg.point)).into_owned(),
        covariance: msg.covariance,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        point: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Borrowed(&msg.point)).into_owned(),
        covariance: msg.covariance,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      point: geometry_msgs::msg::Point::from_rmw_message(msg.point),
      covariance: msg.covariance,
    }
  }
}


// Corresponds to eufs_msgs__msg__ConeAssociation
/// Represents an link between two cones in the same frame

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConeAssociation {
    /// First cone in pair
    /// Typically a cone observed by perception
    pub first: super::msg::ConeWithCovariance,

    /// Second cone in pair
    /// Typically a cone in a map
    pub second: super::msg::ConeWithCovariance,

}



impl Default for ConeAssociation {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ConeAssociation::default())
  }
}

impl rosidl_runtime_rs::Message for ConeAssociation {
  type RmwMsg = super::msg::rmw::ConeAssociation;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        first: super::msg::ConeWithCovariance::into_rmw_message(std::borrow::Cow::Owned(msg.first)).into_owned(),
        second: super::msg::ConeWithCovariance::into_rmw_message(std::borrow::Cow::Owned(msg.second)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        first: super::msg::ConeWithCovariance::into_rmw_message(std::borrow::Cow::Borrowed(&msg.first)).into_owned(),
        second: super::msg::ConeWithCovariance::into_rmw_message(std::borrow::Cow::Borrowed(&msg.second)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      first: super::msg::ConeWithCovariance::from_rmw_message(msg.first),
      second: super::msg::ConeWithCovariance::from_rmw_message(msg.second),
    }
  }
}


// Corresponds to eufs_msgs__msg__ConeAssociationArray
/// Represents a set of cone associations

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConeAssociationArray {
    /// Metadata relating to association type
    /// Association type
    pub type_: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub threshold: f64,

    /// Information relating to matched cones
    pub matched: Vec<super::msg::ConeAssociation>,

    /// Information relating to unmatched cones
    pub unmatched: Vec<super::msg::ConeWithCovariance>,

}



impl Default for ConeAssociationArray {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ConeAssociationArray::default())
  }
}

impl rosidl_runtime_rs::Message for ConeAssociationArray {
  type RmwMsg = super::msg::rmw::ConeAssociationArray;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        type_: msg.type_.as_str().into(),
        threshold: msg.threshold,
        matched: msg.matched
          .into_iter()
          .map(|elem| super::msg::ConeAssociation::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        unmatched: msg.unmatched
          .into_iter()
          .map(|elem| super::msg::ConeWithCovariance::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        type_: msg.type_.as_str().into(),
      threshold: msg.threshold,
        matched: msg.matched
          .iter()
          .map(|elem| super::msg::ConeAssociation::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        unmatched: msg.unmatched
          .iter()
          .map(|elem| super::msg::ConeWithCovariance::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      type_: msg.type_.to_string(),
      threshold: msg.threshold,
      matched: msg.matched
          .into_iter()
          .map(super::msg::ConeAssociation::from_rmw_message)
          .collect(),
      unmatched: msg.unmatched
          .into_iter()
          .map(super::msg::ConeWithCovariance::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to eufs_msgs__msg__ConeAssociationArrayArrayStamped

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConeAssociationArrayArrayStamped {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub associations: Vec<super::msg::ConeAssociationArray>,

}



impl Default for ConeAssociationArrayArrayStamped {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ConeAssociationArrayArrayStamped::default())
  }
}

impl rosidl_runtime_rs::Message for ConeAssociationArrayArrayStamped {
  type RmwMsg = super::msg::rmw::ConeAssociationArrayArrayStamped;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        associations: msg.associations
          .into_iter()
          .map(|elem| super::msg::ConeAssociationArray::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        associations: msg.associations
          .iter()
          .map(|elem| super::msg::ConeAssociationArray::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      associations: msg.associations
          .into_iter()
          .map(super::msg::ConeAssociationArray::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to eufs_msgs__msg__ConeAssociationArrayStamped
/// Represents a set of cone associations

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConeAssociationArrayStamped {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub associations: super::msg::ConeAssociationArray,

}



impl Default for ConeAssociationArrayStamped {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ConeAssociationArrayStamped::default())
  }
}

impl rosidl_runtime_rs::Message for ConeAssociationArrayStamped {
  type RmwMsg = super::msg::rmw::ConeAssociationArrayStamped;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        associations: super::msg::ConeAssociationArray::into_rmw_message(std::borrow::Cow::Owned(msg.associations)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        associations: super::msg::ConeAssociationArray::into_rmw_message(std::borrow::Cow::Borrowed(&msg.associations)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      associations: super::msg::ConeAssociationArray::from_rmw_message(msg.associations),
    }
  }
}


// Corresponds to eufs_msgs__msg__Costmap
/// costmap for the MPPI algorithm in the form of a 2D image

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    pub channel0: Vec<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub channel1: Vec<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub channel2: Vec<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub channel3: Vec<f32>,

}



impl Default for Costmap {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Costmap::default())
  }
}

impl rosidl_runtime_rs::Message for Costmap {
  type RmwMsg = super::msg::rmw::Costmap;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        x_bounds_min: msg.x_bounds_min,
        x_bounds_max: msg.x_bounds_max,
        y_bounds_min: msg.y_bounds_min,
        y_bounds_max: msg.y_bounds_max,
        pixels_per_meter: msg.pixels_per_meter,
        channel0: msg.channel0.into(),
        channel1: msg.channel1.into(),
        channel2: msg.channel2.into(),
        channel3: msg.channel3.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      x_bounds_min: msg.x_bounds_min,
      x_bounds_max: msg.x_bounds_max,
      y_bounds_min: msg.y_bounds_min,
      y_bounds_max: msg.y_bounds_max,
      pixels_per_meter: msg.pixels_per_meter,
        channel0: msg.channel0.as_slice().into(),
        channel1: msg.channel1.as_slice().into(),
        channel2: msg.channel2.as_slice().into(),
        channel3: msg.channel3.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      x_bounds_min: msg.x_bounds_min,
      x_bounds_max: msg.x_bounds_max,
      y_bounds_min: msg.y_bounds_min,
      y_bounds_max: msg.y_bounds_max,
      pixels_per_meter: msg.pixels_per_meter,
      channel0: msg.channel0
          .into_iter()
          .collect(),
      channel1: msg.channel1
          .into_iter()
          .collect(),
      channel2: msg.channel2
          .into_iter()
          .collect(),
      channel3: msg.channel3
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to eufs_msgs__msg__CpuStatus
/// constants

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CpuStatus::default())
  }
}

impl rosidl_runtime_rs::Message for CpuStatus {
  type RmwMsg = super::msg::rmw::CpuStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        total: msg.total,
        usr: msg.usr,
        nice: msg.nice,
        sys: msg.sys,
        idle: msg.idle,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
      total: msg.total,
      usr: msg.usr,
      nice: msg.nice,
      sys: msg.sys,
      idle: msg.idle,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      total: msg.total,
      usr: msg.usr,
      nice: msg.nice,
      sys: msg.sys,
      idle: msg.idle,
    }
  }
}


// Corresponds to eufs_msgs__msg__CpuUsage

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CpuUsage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,


    // This member is not documented.
    #[allow(missing_docs)]
    pub all: super::msg::CpuStatus,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cpus: Vec<super::msg::CpuStatus>,

}



impl Default for CpuUsage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CpuUsage::default())
  }
}

impl rosidl_runtime_rs::Message for CpuUsage {
  type RmwMsg = super::msg::rmw::CpuUsage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
        all: super::msg::CpuStatus::into_rmw_message(std::borrow::Cow::Owned(msg.all)).into_owned(),
        cpus: msg.cpus
          .into_iter()
          .map(|elem| super::msg::CpuStatus::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
        all: super::msg::CpuStatus::into_rmw_message(std::borrow::Cow::Borrowed(&msg.all)).into_owned(),
        cpus: msg.cpus
          .iter()
          .map(|elem| super::msg::CpuStatus::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
      all: super::msg::CpuStatus::from_rmw_message(msg.all),
      cpus: msg.cpus
          .into_iter()
          .map(super::msg::CpuStatus::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to eufs_msgs__msg__EKFErr
/// Message for the error of the EKF. All of them are based on euclidean distances.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EKFErr {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::EKFErr::default())
  }
}

impl rosidl_runtime_rs::Message for EKFErr {
  type RmwMsg = super::msg::rmw::EKFErr;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        gps_x_vel_err: msg.gps_x_vel_err,
        gps_y_vel_err: msg.gps_y_vel_err,
        imu_x_acc_err: msg.imu_x_acc_err,
        imu_y_acc_err: msg.imu_y_acc_err,
        imu_yaw_err: msg.imu_yaw_err,
        ekf_x_vel_var: msg.ekf_x_vel_var,
        ekf_y_vel_var: msg.ekf_y_vel_var,
        ekf_x_acc_var: msg.ekf_x_acc_var,
        ekf_y_acc_var: msg.ekf_y_acc_var,
        ekf_yaw_var: msg.ekf_yaw_var,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      gps_x_vel_err: msg.gps_x_vel_err,
      gps_y_vel_err: msg.gps_y_vel_err,
      imu_x_acc_err: msg.imu_x_acc_err,
      imu_y_acc_err: msg.imu_y_acc_err,
      imu_yaw_err: msg.imu_yaw_err,
      ekf_x_vel_var: msg.ekf_x_vel_var,
      ekf_y_vel_var: msg.ekf_y_vel_var,
      ekf_x_acc_var: msg.ekf_x_acc_var,
      ekf_y_acc_var: msg.ekf_y_acc_var,
      ekf_yaw_var: msg.ekf_yaw_var,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      gps_x_vel_err: msg.gps_x_vel_err,
      gps_y_vel_err: msg.gps_y_vel_err,
      imu_x_acc_err: msg.imu_x_acc_err,
      imu_y_acc_err: msg.imu_y_acc_err,
      imu_yaw_err: msg.imu_yaw_err,
      ekf_x_vel_var: msg.ekf_x_vel_var,
      ekf_y_vel_var: msg.ekf_y_vel_var,
      ekf_x_acc_var: msg.ekf_x_acc_var,
      ekf_y_acc_var: msg.ekf_y_acc_var,
      ekf_yaw_var: msg.ekf_yaw_var,
    }
  }
}


// Corresponds to eufs_msgs__msg__EKFState
/// This message contains information about the state of the EKF
/// and is meant for use by the state monitor to ensure the EKF
/// isn't failing.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::EKFState::default())
  }
}

impl rosidl_runtime_rs::Message for EKFState {
  type RmwMsg = super::msg::rmw::EKFState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        gps_received: msg.gps_received,
        imu_received: msg.imu_received,
        wheel_odom_received: msg.wheel_odom_received,
        ekf_odom_received: msg.ekf_odom_received,
        ekf_accel_received: msg.ekf_accel_received,
        currently_over_covariance_limit: msg.currently_over_covariance_limit,
        consecutive_turns_over_covariance_limit: msg.consecutive_turns_over_covariance_limit,
        recommends_failure: msg.recommends_failure,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      gps_received: msg.gps_received,
      imu_received: msg.imu_received,
      wheel_odom_received: msg.wheel_odom_received,
      ekf_odom_received: msg.ekf_odom_received,
      ekf_accel_received: msg.ekf_accel_received,
      currently_over_covariance_limit: msg.currently_over_covariance_limit,
      consecutive_turns_over_covariance_limit: msg.consecutive_turns_over_covariance_limit,
      recommends_failure: msg.recommends_failure,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      gps_received: msg.gps_received,
      imu_received: msg.imu_received,
      wheel_odom_received: msg.wheel_odom_received,
      ekf_odom_received: msg.ekf_odom_received,
      ekf_accel_received: msg.ekf_accel_received,
      currently_over_covariance_limit: msg.currently_over_covariance_limit,
      consecutive_turns_over_covariance_limit: msg.consecutive_turns_over_covariance_limit,
      recommends_failure: msg.recommends_failure,
    }
  }
}


// Corresponds to eufs_msgs__msg__FullState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FullState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::FullState::default())
  }
}

impl rosidl_runtime_rs::Message for FullState {
  type RmwMsg = super::msg::rmw::FullState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        x_pos: msg.x_pos,
        y_pos: msg.y_pos,
        yaw: msg.yaw,
        roll: msg.roll,
        u_x: msg.u_x,
        u_y: msg.u_y,
        yaw_mder: msg.yaw_mder,
        front_throttle: msg.front_throttle,
        rear_throttle: msg.rear_throttle,
        steering: msg.steering,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      x_pos: msg.x_pos,
      y_pos: msg.y_pos,
      yaw: msg.yaw,
      roll: msg.roll,
      u_x: msg.u_x,
      u_y: msg.u_y,
      yaw_mder: msg.yaw_mder,
      front_throttle: msg.front_throttle,
      rear_throttle: msg.rear_throttle,
      steering: msg.steering,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      x_pos: msg.x_pos,
      y_pos: msg.y_pos,
      yaw: msg.yaw,
      roll: msg.roll,
      u_x: msg.u_x,
      u_y: msg.u_y,
      yaw_mder: msg.yaw_mder,
      front_throttle: msg.front_throttle,
      rear_throttle: msg.rear_throttle,
      steering: msg.steering,
    }
  }
}


// Corresponds to eufs_msgs__msg__Heartbeat

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Heartbeat::default())
  }
}

impl rosidl_runtime_rs::Message for Heartbeat {
  type RmwMsg = super::msg::rmw::Heartbeat;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id,
        data: msg.data,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      id: msg.id,
      data: msg.data,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id: msg.id,
      data: msg.data,
    }
  }
}


// Corresponds to eufs_msgs__msg__IntegrationErr
/// Message for the error of Odometry Integration. Error based on euclidean distance.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IntegrationErr {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::IntegrationErr::default())
  }
}

impl rosidl_runtime_rs::Message for IntegrationErr {
  type RmwMsg = super::msg::rmw::IntegrationErr;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        position_err: msg.position_err,
        orientation_err: msg.orientation_err,
        linear_vel_err: msg.linear_vel_err,
        angular_vel_err: msg.angular_vel_err,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      position_err: msg.position_err,
      orientation_err: msg.orientation_err,
      linear_vel_err: msg.linear_vel_err,
      angular_vel_err: msg.angular_vel_err,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      position_err: msg.position_err,
      orientation_err: msg.orientation_err,
      linear_vel_err: msg.linear_vel_err,
      angular_vel_err: msg.angular_vel_err,
    }
  }
}


// Corresponds to eufs_msgs__msg__LapStats

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LapStats {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::LapStats::default())
  }
}

impl rosidl_runtime_rs::Message for LapStats {
  type RmwMsg = super::msg::rmw::LapStats;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        lap_number: msg.lap_number,
        lap_time: msg.lap_time,
        avg_speed: msg.avg_speed,
        max_speed: msg.max_speed,
        speed_var: msg.speed_var,
        max_slip: msg.max_slip,
        max_lateral_accel: msg.max_lateral_accel,
        normalized_deviation_mse: msg.normalized_deviation_mse,
        deviation_var: msg.deviation_var,
        max_deviation: msg.max_deviation,
        oscillation_number: msg.oscillation_number,
        num_cones_hit: msg.num_cones_hit,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      lap_number: msg.lap_number,
      lap_time: msg.lap_time,
      avg_speed: msg.avg_speed,
      max_speed: msg.max_speed,
      speed_var: msg.speed_var,
      max_slip: msg.max_slip,
      max_lateral_accel: msg.max_lateral_accel,
      normalized_deviation_mse: msg.normalized_deviation_mse,
      deviation_var: msg.deviation_var,
      max_deviation: msg.max_deviation,
      oscillation_number: msg.oscillation_number,
      num_cones_hit: msg.num_cones_hit,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      lap_number: msg.lap_number,
      lap_time: msg.lap_time,
      avg_speed: msg.avg_speed,
      max_speed: msg.max_speed,
      speed_var: msg.speed_var,
      max_slip: msg.max_slip,
      max_lateral_accel: msg.max_lateral_accel,
      normalized_deviation_mse: msg.normalized_deviation_mse,
      deviation_var: msg.deviation_var,
      max_deviation: msg.max_deviation,
      oscillation_number: msg.oscillation_number,
      num_cones_hit: msg.num_cones_hit,
    }
  }
}


// Corresponds to eufs_msgs__msg__MPCState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MPCState::default())
  }
}

impl rosidl_runtime_rs::Message for MPCState {
  type RmwMsg = super::msg::rmw::MPCState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        exitflag: msg.exitflag,
        iterations: msg.iterations,
        solve_time: msg.solve_time,
        cost: msg.cost,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      exitflag: msg.exitflag,
      iterations: msg.iterations,
      solve_time: msg.solve_time,
      cost: msg.cost,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      exitflag: msg.exitflag,
      iterations: msg.iterations,
      solve_time: msg.solve_time,
      cost: msg.cost,
    }
  }
}


// Corresponds to eufs_msgs__msg__NodeState
/// ID of the node

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NodeState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: u16,

    /// Name of the node
    pub name: std::string::String,

    /// Next expected heartbeat value
    pub exp_heartbeat: u8,

    /// Time of last heartbeat
    pub last_heartbeat: builtin_interfaces::msg::Time,

    /// Describes the action to take if node is offline
    pub severity: u8,

    /// Whether the node is running or not according to the status monitor
    pub online: bool,

}



impl Default for NodeState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::NodeState::default())
  }
}

impl rosidl_runtime_rs::Message for NodeState {
  type RmwMsg = super::msg::rmw::NodeState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id,
        name: msg.name.as_str().into(),
        exp_heartbeat: msg.exp_heartbeat,
        last_heartbeat: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.last_heartbeat)).into_owned(),
        severity: msg.severity,
        online: msg.online,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      id: msg.id,
        name: msg.name.as_str().into(),
      exp_heartbeat: msg.exp_heartbeat,
        last_heartbeat: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.last_heartbeat)).into_owned(),
      severity: msg.severity,
      online: msg.online,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id: msg.id,
      name: msg.name.to_string(),
      exp_heartbeat: msg.exp_heartbeat,
      last_heartbeat: builtin_interfaces::msg::Time::from_rmw_message(msg.last_heartbeat),
      severity: msg.severity,
      online: msg.online,
    }
  }
}


// Corresponds to eufs_msgs__msg__NodeStateArray

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NodeStateArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub states: Vec<super::msg::NodeState>,

}



impl Default for NodeStateArray {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::NodeStateArray::default())
  }
}

impl rosidl_runtime_rs::Message for NodeStateArray {
  type RmwMsg = super::msg::rmw::NodeStateArray;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        states: msg.states
          .into_iter()
          .map(|elem| super::msg::NodeState::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        states: msg.states
          .iter()
          .map(|elem| super::msg::NodeState::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      states: msg.states
          .into_iter()
          .map(super::msg::NodeState::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to eufs_msgs__msg__OSS
/// Measurements from the ground speed sensor

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct OSS {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::OSS::default())
  }
}

impl rosidl_runtime_rs::Message for OSS {
  type RmwMsg = super::msg::rmw::OSS;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        velocity: msg.velocity,
        v_x: msg.v_x,
        v_y: msg.v_y,
        yaw_rate: msg.yaw_rate,
        distance: msg.distance,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      velocity: msg.velocity,
      v_x: msg.v_x,
      v_y: msg.v_y,
      yaw_rate: msg.yaw_rate,
      distance: msg.distance,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      velocity: msg.velocity,
      v_x: msg.v_x,
      v_y: msg.v_y,
      yaw_rate: msg.yaw_rate,
      distance: msg.distance,
    }
  }
}


// Corresponds to eufs_msgs__msg__Particle
/// Represents the state of a single particle

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Particle {
    /// Pose associated with the particle
    pub pose: geometry_msgs::msg::Pose,

    /// Map associate with the particle
    pub map: super::msg::ConeArrayWithCovariance,

    /// Weighting placed on this particle
    pub weight: f64,

}



impl Default for Particle {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Particle::default())
  }
}

impl rosidl_runtime_rs::Message for Particle {
  type RmwMsg = super::msg::rmw::Particle;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.pose)).into_owned(),
        map: super::msg::ConeArrayWithCovariance::into_rmw_message(std::borrow::Cow::Owned(msg.map)).into_owned(),
        weight: msg.weight,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose)).into_owned(),
        map: super::msg::ConeArrayWithCovariance::into_rmw_message(std::borrow::Cow::Borrowed(&msg.map)).into_owned(),
      weight: msg.weight,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      pose: geometry_msgs::msg::Pose::from_rmw_message(msg.pose),
      map: super::msg::ConeArrayWithCovariance::from_rmw_message(msg.map),
      weight: msg.weight,
    }
  }
}


// Corresponds to eufs_msgs__msg__ParticleSLAM
/// Represents the state of a particle-based SLAM algorithm

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ParticleSLAM {
    /// The index position current best guess of the SLAM algorithm in `particles`
    pub best_particle_idx: u64,

    /// The set of particles of the SLAM algorithm
    pub particles: Vec<super::msg::Particle>,

}



impl Default for ParticleSLAM {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ParticleSLAM::default())
  }
}

impl rosidl_runtime_rs::Message for ParticleSLAM {
  type RmwMsg = super::msg::rmw::ParticleSLAM;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        best_particle_idx: msg.best_particle_idx,
        particles: msg.particles
          .into_iter()
          .map(|elem| super::msg::Particle::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      best_particle_idx: msg.best_particle_idx,
        particles: msg.particles
          .iter()
          .map(|elem| super::msg::Particle::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      best_particle_idx: msg.best_particle_idx,
      particles: msg.particles
          .into_iter()
          .map(super::msg::Particle::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to eufs_msgs__msg__ParticleSLAMStamped
/// ParticleSLAM with reference frame and timestamp

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ParticleSLAMStamped {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub state: super::msg::ParticleSLAM,

}



impl Default for ParticleSLAMStamped {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ParticleSLAMStamped::default())
  }
}

impl rosidl_runtime_rs::Message for ParticleSLAMStamped {
  type RmwMsg = super::msg::rmw::ParticleSLAMStamped;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        state: super::msg::ParticleSLAM::into_rmw_message(std::borrow::Cow::Owned(msg.state)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        state: super::msg::ParticleSLAM::into_rmw_message(std::borrow::Cow::Borrowed(&msg.state)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      state: super::msg::ParticleSLAM::from_rmw_message(msg.state),
    }
  }
}


// Corresponds to eufs_msgs__msg__ParticleStamped
/// Particle with reference coordinate frame and timestamp

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ParticleStamped {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub particle: super::msg::Particle,

}



impl Default for ParticleStamped {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ParticleStamped::default())
  }
}

impl rosidl_runtime_rs::Message for ParticleStamped {
  type RmwMsg = super::msg::rmw::ParticleStamped;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        particle: super::msg::Particle::into_rmw_message(std::borrow::Cow::Owned(msg.particle)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        particle: super::msg::Particle::into_rmw_message(std::borrow::Cow::Borrowed(&msg.particle)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      particle: super::msg::Particle::from_rmw_message(msg.particle),
    }
  }
}


// Corresponds to eufs_msgs__msg__PathIntegralParams

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    pub map_path: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub desired_speed: f64,

}



impl Default for PathIntegralParams {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PathIntegralParams::default())
  }
}

impl rosidl_runtime_rs::Message for PathIntegralParams {
  type RmwMsg = super::msg::rmw::PathIntegralParams;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        hz: msg.hz,
        num_timesteps: msg.num_timesteps,
        num_iters: msg.num_iters,
        gamma: msg.gamma,
        init_steering: msg.init_steering,
        init_throttle: msg.init_throttle,
        steering_var: msg.steering_var,
        throttle_var: msg.throttle_var,
        max_throttle: msg.max_throttle,
        speed_coefficient: msg.speed_coefficient,
        track_coefficient: msg.track_coefficient,
        max_slip_angle: msg.max_slip_angle,
        track_slop: msg.track_slop,
        crash_coeff: msg.crash_coeff,
        map_path: msg.map_path.as_str().into(),
        desired_speed: msg.desired_speed,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      hz: msg.hz,
      num_timesteps: msg.num_timesteps,
      num_iters: msg.num_iters,
      gamma: msg.gamma,
      init_steering: msg.init_steering,
      init_throttle: msg.init_throttle,
      steering_var: msg.steering_var,
      throttle_var: msg.throttle_var,
      max_throttle: msg.max_throttle,
      speed_coefficient: msg.speed_coefficient,
      track_coefficient: msg.track_coefficient,
      max_slip_angle: msg.max_slip_angle,
      track_slop: msg.track_slop,
      crash_coeff: msg.crash_coeff,
        map_path: msg.map_path.as_str().into(),
      desired_speed: msg.desired_speed,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      hz: msg.hz,
      num_timesteps: msg.num_timesteps,
      num_iters: msg.num_iters,
      gamma: msg.gamma,
      init_steering: msg.init_steering,
      init_throttle: msg.init_throttle,
      steering_var: msg.steering_var,
      throttle_var: msg.throttle_var,
      max_throttle: msg.max_throttle,
      speed_coefficient: msg.speed_coefficient,
      track_coefficient: msg.track_coefficient,
      max_slip_angle: msg.max_slip_angle,
      track_slop: msg.track_slop,
      crash_coeff: msg.crash_coeff,
      map_path: msg.map_path.to_string(),
      desired_speed: msg.desired_speed,
    }
  }
}


// Corresponds to eufs_msgs__msg__PathIntegralStats

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PathIntegralStats {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tag: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub params: super::msg::PathIntegralParams,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stats: super::msg::LapStats,

}



impl Default for PathIntegralStats {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PathIntegralStats::default())
  }
}

impl rosidl_runtime_rs::Message for PathIntegralStats {
  type RmwMsg = super::msg::rmw::PathIntegralStats;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        tag: msg.tag.as_str().into(),
        params: super::msg::PathIntegralParams::into_rmw_message(std::borrow::Cow::Owned(msg.params)).into_owned(),
        stats: super::msg::LapStats::into_rmw_message(std::borrow::Cow::Owned(msg.stats)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        tag: msg.tag.as_str().into(),
        params: super::msg::PathIntegralParams::into_rmw_message(std::borrow::Cow::Borrowed(&msg.params)).into_owned(),
        stats: super::msg::LapStats::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stats)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      tag: msg.tag.to_string(),
      params: super::msg::PathIntegralParams::from_rmw_message(msg.params),
      stats: super::msg::LapStats::from_rmw_message(msg.stats),
    }
  }
}


// Corresponds to eufs_msgs__msg__PathIntegralStatus

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PathIntegralStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub info: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i32,

}



impl Default for PathIntegralStatus {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PathIntegralStatus::default())
  }
}

impl rosidl_runtime_rs::Message for PathIntegralStatus {
  type RmwMsg = super::msg::rmw::PathIntegralStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        info: msg.info.as_str().into(),
        status: msg.status,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        info: msg.info.as_str().into(),
      status: msg.status,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      info: msg.info.to_string(),
      status: msg.status,
    }
  }
}


// Corresponds to eufs_msgs__msg__PathIntegralTiming

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PathIntegralTiming {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PathIntegralTiming::default())
  }
}

impl rosidl_runtime_rs::Message for PathIntegralTiming {
  type RmwMsg = super::msg::rmw::PathIntegralTiming;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        average_time_between_poses: msg.average_time_between_poses,
        average_optimization_cycle_time: msg.average_optimization_cycle_time,
        average_sleep_time: msg.average_sleep_time,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      average_time_between_poses: msg.average_time_between_poses,
      average_optimization_cycle_time: msg.average_optimization_cycle_time,
      average_sleep_time: msg.average_sleep_time,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      average_time_between_poses: msg.average_time_between_poses,
      average_optimization_cycle_time: msg.average_optimization_cycle_time,
      average_sleep_time: msg.average_sleep_time,
    }
  }
}


// Corresponds to eufs_msgs__msg__PlanningMode

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PlanningMode::default())
  }
}

impl rosidl_runtime_rs::Message for PlanningMode {
  type RmwMsg = super::msg::rmw::PlanningMode;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      mode: msg.mode,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mode: msg.mode,
    }
  }
}


// Corresponds to eufs_msgs__msg__PointArray
/// Simple msg for array of points

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PointArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub points: Vec<geometry_msgs::msg::Point>,

}



impl Default for PointArray {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PointArray::default())
  }
}

impl rosidl_runtime_rs::Message for PointArray {
  type RmwMsg = super::msg::rmw::PointArray;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        points: msg.points
          .into_iter()
          .map(|elem| geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        points: msg.points
          .iter()
          .map(|elem| geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      points: msg.points
          .into_iter()
          .map(geometry_msgs::msg::Point::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to eufs_msgs__msg__PointArrayStamped

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PointArrayStamped {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// Simple msg for array of points
    pub points: Vec<geometry_msgs::msg::Point>,

}



impl Default for PointArrayStamped {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PointArrayStamped::default())
  }
}

impl rosidl_runtime_rs::Message for PointArrayStamped {
  type RmwMsg = super::msg::rmw::PointArrayStamped;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        points: msg.points
          .into_iter()
          .map(|elem| geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        points: msg.points
          .iter()
          .map(|elem| geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      points: msg.points
          .into_iter()
          .map(geometry_msgs::msg::Point::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to eufs_msgs__msg__PurePursuitCheckpoint
/// Checkpoint position [x, y, z]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PurePursuitCheckpoint {

    // This member is not documented.
    #[allow(missing_docs)]
    pub position: geometry_msgs::msg::Point,

    /// Maximum speed
    pub max_speed: f64,

    /// Maximum lateral acceleration
    pub max_lateral_acceleration: f64,

}



impl Default for PurePursuitCheckpoint {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PurePursuitCheckpoint::default())
  }
}

impl rosidl_runtime_rs::Message for PurePursuitCheckpoint {
  type RmwMsg = super::msg::rmw::PurePursuitCheckpoint;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        position: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Owned(msg.position)).into_owned(),
        max_speed: msg.max_speed,
        max_lateral_acceleration: msg.max_lateral_acceleration,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        position: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Borrowed(&msg.position)).into_owned(),
      max_speed: msg.max_speed,
      max_lateral_acceleration: msg.max_lateral_acceleration,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      position: geometry_msgs::msg::Point::from_rmw_message(msg.position),
      max_speed: msg.max_speed,
      max_lateral_acceleration: msg.max_lateral_acceleration,
    }
  }
}


// Corresponds to eufs_msgs__msg__PurePursuitCheckpointArrayStamped
/// Stamped checkpoint array msg

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PurePursuitCheckpointArrayStamped {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub checkpoints: Vec<super::msg::PurePursuitCheckpoint>,

}



impl Default for PurePursuitCheckpointArrayStamped {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PurePursuitCheckpointArrayStamped::default())
  }
}

impl rosidl_runtime_rs::Message for PurePursuitCheckpointArrayStamped {
  type RmwMsg = super::msg::rmw::PurePursuitCheckpointArrayStamped;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        checkpoints: msg.checkpoints
          .into_iter()
          .map(|elem| super::msg::PurePursuitCheckpoint::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        checkpoints: msg.checkpoints
          .iter()
          .map(|elem| super::msg::PurePursuitCheckpoint::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      checkpoints: msg.checkpoints
          .into_iter()
          .map(super::msg::PurePursuitCheckpoint::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to eufs_msgs__msg__Runstop

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Runstop {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sender: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub motion_enabled: bool,

}



impl Default for Runstop {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Runstop::default())
  }
}

impl rosidl_runtime_rs::Message for Runstop {
  type RmwMsg = super::msg::rmw::Runstop;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        sender: msg.sender.as_str().into(),
        motion_enabled: msg.motion_enabled,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        sender: msg.sender.as_str().into(),
      motion_enabled: msg.motion_enabled,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      sender: msg.sender.to_string(),
      motion_enabled: msg.motion_enabled,
    }
  }
}


// Corresponds to eufs_msgs__msg__SLAMErr
/// Message for the error of our SLAM algorithm. All of them are based on euclidean distances, besides the map similarity,
/// which can be interpreted as a percentage of how well our algorithm describes the map.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SLAMErr {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::SLAMErr::default())
  }
}

impl rosidl_runtime_rs::Message for SLAMErr {
  type RmwMsg = super::msg::rmw::SLAMErr;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        x_err: msg.x_err,
        y_err: msg.y_err,
        z_err: msg.z_err,
        x_orient_err: msg.x_orient_err,
        y_orient_err: msg.y_orient_err,
        z_orient_err: msg.z_orient_err,
        w_orient_err: msg.w_orient_err,
        map_similarity: msg.map_similarity,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      x_err: msg.x_err,
      y_err: msg.y_err,
      z_err: msg.z_err,
      x_orient_err: msg.x_orient_err,
      y_orient_err: msg.y_orient_err,
      z_orient_err: msg.z_orient_err,
      w_orient_err: msg.w_orient_err,
      map_similarity: msg.map_similarity,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      x_err: msg.x_err,
      y_err: msg.y_err,
      z_err: msg.z_err,
      x_orient_err: msg.x_orient_err,
      y_orient_err: msg.y_orient_err,
      z_orient_err: msg.z_orient_err,
      w_orient_err: msg.w_orient_err,
      map_similarity: msg.map_similarity,
    }
  }
}


// Corresponds to eufs_msgs__msg__SLAMState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SLAMState {
    /// True after loop closure
    pub loop_closed: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub laps: i16,

    /// Dump for any logging purposes. Eg: "particle covariance high; can't localise"
    pub status: std::string::String,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::SLAMState::default())
  }
}

impl rosidl_runtime_rs::Message for SLAMState {
  type RmwMsg = super::msg::rmw::SLAMState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        loop_closed: msg.loop_closed,
        laps: msg.laps,
        status: msg.status.as_str().into(),
        state: msg.state,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      loop_closed: msg.loop_closed,
      laps: msg.laps,
        status: msg.status.as_str().into(),
      state: msg.state,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      loop_closed: msg.loop_closed,
      laps: msg.laps,
      status: msg.status.to_string(),
      state: msg.state,
    }
  }
}


// Corresponds to eufs_msgs__msg__StateMachineState
/// State of the EUFS State Machine

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StateMachineState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub state: u16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub state_str: std::string::String,

}



impl Default for StateMachineState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::StateMachineState::default())
  }
}

impl rosidl_runtime_rs::Message for StateMachineState {
  type RmwMsg = super::msg::rmw::StateMachineState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        state: msg.state,
        state_str: msg.state_str.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      state: msg.state,
        state_str: msg.state_str.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      state: msg.state,
      state_str: msg.state_str.to_string(),
    }
  }
}


// Corresponds to eufs_msgs__msg__StereoImage

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StereoImage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub color: sensor_msgs::msg::Image,


    // This member is not documented.
    #[allow(missing_docs)]
    pub depth: sensor_msgs::msg::Image,

}



impl Default for StereoImage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::StereoImage::default())
  }
}

impl rosidl_runtime_rs::Message for StereoImage {
  type RmwMsg = super::msg::rmw::StereoImage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        color: sensor_msgs::msg::Image::into_rmw_message(std::borrow::Cow::Owned(msg.color)).into_owned(),
        depth: sensor_msgs::msg::Image::into_rmw_message(std::borrow::Cow::Owned(msg.depth)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        color: sensor_msgs::msg::Image::into_rmw_message(std::borrow::Cow::Borrowed(&msg.color)).into_owned(),
        depth: sensor_msgs::msg::Image::into_rmw_message(std::borrow::Cow::Borrowed(&msg.depth)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      color: sensor_msgs::msg::Image::from_rmw_message(msg.color),
      depth: sensor_msgs::msg::Image::from_rmw_message(msg.depth),
    }
  }
}


// Corresponds to eufs_msgs__msg__SystemStatus

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SystemStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub topic_statuses: Vec<super::msg::TopicStatus>,

}



impl Default for SystemStatus {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::SystemStatus::default())
  }
}

impl rosidl_runtime_rs::Message for SystemStatus {
  type RmwMsg = super::msg::rmw::SystemStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        topic_statuses: msg.topic_statuses
          .into_iter()
          .map(|elem| super::msg::TopicStatus::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        topic_statuses: msg.topic_statuses
          .iter()
          .map(|elem| super::msg::TopicStatus::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      topic_statuses: msg.topic_statuses
          .into_iter()
          .map(super::msg::TopicStatus::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to eufs_msgs__msg__TopicStatus
/// Topic information

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TopicStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub topic: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub description: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub group: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub trigger_ebs: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub log_level: std::string::String,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::TopicStatus::default())
  }
}

impl rosidl_runtime_rs::Message for TopicStatus {
  type RmwMsg = super::msg::rmw::TopicStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        topic: msg.topic.as_str().into(),
        description: msg.description.as_str().into(),
        group: msg.group.as_str().into(),
        trigger_ebs: msg.trigger_ebs,
        log_level: msg.log_level.as_str().into(),
        status: msg.status,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        topic: msg.topic.as_str().into(),
        description: msg.description.as_str().into(),
        group: msg.group.as_str().into(),
      trigger_ebs: msg.trigger_ebs,
        log_level: msg.log_level.as_str().into(),
      status: msg.status,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      topic: msg.topic.to_string(),
      description: msg.description.to_string(),
      group: msg.group.to_string(),
      trigger_ebs: msg.trigger_ebs,
      log_level: msg.log_level.to_string(),
      status: msg.status,
    }
  }
}


// Corresponds to eufs_msgs__msg__VehicleCommands

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::VehicleCommands::default())
  }
}

impl rosidl_runtime_rs::Message for VehicleCommands {
  type RmwMsg = super::msg::rmw::VehicleCommands;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        handshake: msg.handshake,
        ebs: msg.ebs,
        direction: msg.direction,
        mission_status: msg.mission_status,
        braking: msg.braking,
        torque: msg.torque,
        steering: msg.steering,
        rpm: msg.rpm,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      handshake: msg.handshake,
      ebs: msg.ebs,
      direction: msg.direction,
      mission_status: msg.mission_status,
      braking: msg.braking,
      torque: msg.torque,
      steering: msg.steering,
      rpm: msg.rpm,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      handshake: msg.handshake,
      ebs: msg.ebs,
      direction: msg.direction,
      mission_status: msg.mission_status,
      braking: msg.braking,
      torque: msg.torque,
      steering: msg.steering,
      rpm: msg.rpm,
    }
  }
}


// Corresponds to eufs_msgs__msg__VehicleCommandsStamped

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VehicleCommandsStamped {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub commands: super::msg::VehicleCommands,

}



impl Default for VehicleCommandsStamped {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::VehicleCommandsStamped::default())
  }
}

impl rosidl_runtime_rs::Message for VehicleCommandsStamped {
  type RmwMsg = super::msg::rmw::VehicleCommandsStamped;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        commands: super::msg::VehicleCommands::into_rmw_message(std::borrow::Cow::Owned(msg.commands)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        commands: super::msg::VehicleCommands::into_rmw_message(std::borrow::Cow::Borrowed(&msg.commands)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      commands: super::msg::VehicleCommands::from_rmw_message(msg.commands),
    }
  }
}


// Corresponds to eufs_msgs__msg__Waypoint
/// Waypoint position

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Waypoint {

    // This member is not documented.
    #[allow(missing_docs)]
    pub position: geometry_msgs::msg::Point,

    /// Suggested forward velocity (x direction in car frame)
    /// m/s
    pub speed: f64,

    /// Suggested steering angle
    /// rad
    pub suggested_steering: f64,

}



impl Default for Waypoint {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Waypoint::default())
  }
}

impl rosidl_runtime_rs::Message for Waypoint {
  type RmwMsg = super::msg::rmw::Waypoint;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        position: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Owned(msg.position)).into_owned(),
        speed: msg.speed,
        suggested_steering: msg.suggested_steering,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        position: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Borrowed(&msg.position)).into_owned(),
      speed: msg.speed,
      suggested_steering: msg.suggested_steering,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      position: geometry_msgs::msg::Point::from_rmw_message(msg.position),
      speed: msg.speed,
      suggested_steering: msg.suggested_steering,
    }
  }
}


// Corresponds to eufs_msgs__msg__WaypointArrayStamped
/// This message is used by planning nodes to advertise suggested trajectory waypoints
/// as well as suggested velocity and steering at each waypoint.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WaypointArrayStamped {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub waypoints: Vec<super::msg::Waypoint>,

}



impl Default for WaypointArrayStamped {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::WaypointArrayStamped::default())
  }
}

impl rosidl_runtime_rs::Message for WaypointArrayStamped {
  type RmwMsg = super::msg::rmw::WaypointArrayStamped;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        waypoints: msg.waypoints
          .into_iter()
          .map(|elem| super::msg::Waypoint::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        waypoints: msg.waypoints
          .iter()
          .map(|elem| super::msg::Waypoint::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      waypoints: msg.waypoints
          .into_iter()
          .map(super::msg::Waypoint::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to eufs_msgs__msg__WheelOdometryErr
/// Message for the error of Wheel Odometry. Error based on euclidean distance.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WheelOdometryErr {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::WheelOdometryErr::default())
  }
}

impl rosidl_runtime_rs::Message for WheelOdometryErr {
  type RmwMsg = super::msg::rmw::WheelOdometryErr;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        position_err: msg.position_err,
        orientation_err: msg.orientation_err,
        linear_vel_err: msg.linear_vel_err,
        angular_vel_err: msg.angular_vel_err,
        forward_vel_err: msg.forward_vel_err,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      position_err: msg.position_err,
      orientation_err: msg.orientation_err,
      linear_vel_err: msg.linear_vel_err,
      angular_vel_err: msg.angular_vel_err,
      forward_vel_err: msg.forward_vel_err,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      position_err: msg.position_err,
      orientation_err: msg.orientation_err,
      linear_vel_err: msg.linear_vel_err,
      angular_vel_err: msg.angular_vel_err,
      forward_vel_err: msg.forward_vel_err,
    }
  }
}


// Corresponds to eufs_msgs__msg__WheelSpeeds

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::WheelSpeeds::default())
  }
}

impl rosidl_runtime_rs::Message for WheelSpeeds {
  type RmwMsg = super::msg::rmw::WheelSpeeds;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        steering: msg.steering,
        lf_speed: msg.lf_speed,
        rf_speed: msg.rf_speed,
        lb_speed: msg.lb_speed,
        rb_speed: msg.rb_speed,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      steering: msg.steering,
      lf_speed: msg.lf_speed,
      rf_speed: msg.rf_speed,
      lb_speed: msg.lb_speed,
      rb_speed: msg.rb_speed,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      steering: msg.steering,
      lf_speed: msg.lf_speed,
      rf_speed: msg.rf_speed,
      lb_speed: msg.lb_speed,
      rb_speed: msg.rb_speed,
    }
  }
}


// Corresponds to eufs_msgs__msg__WheelSpeedsStamped

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WheelSpeedsStamped {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub speeds: super::msg::WheelSpeeds,

}



impl Default for WheelSpeedsStamped {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::WheelSpeedsStamped::default())
  }
}

impl rosidl_runtime_rs::Message for WheelSpeedsStamped {
  type RmwMsg = super::msg::rmw::WheelSpeedsStamped;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        speeds: super::msg::WheelSpeeds::into_rmw_message(std::borrow::Cow::Owned(msg.speeds)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        speeds: super::msg::WheelSpeeds::into_rmw_message(std::borrow::Cow::Borrowed(&msg.speeds)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      speeds: super::msg::WheelSpeeds::from_rmw_message(msg.speeds),
    }
  }
}


