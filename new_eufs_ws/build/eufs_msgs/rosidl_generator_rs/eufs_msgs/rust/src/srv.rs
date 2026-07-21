#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to eufs_msgs__srv__RecordStart_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RecordStart_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pack_number: i64,

}



impl Default for RecordStart_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RecordStart_Request::default())
  }
}

impl rosidl_runtime_rs::Message for RecordStart_Request {
  type RmwMsg = super::srv::rmw::RecordStart_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pack_number: msg.pack_number,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      pack_number: msg.pack_number,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      pack_number: msg.pack_number,
    }
  }
}


// Corresponds to eufs_msgs__srv__RecordStart_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RecordStart_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub start_recording: bool,

}



impl Default for RecordStart_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RecordStart_Response::default())
  }
}

impl rosidl_runtime_rs::Message for RecordStart_Response {
  type RmwMsg = super::srv::rmw::RecordStart_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        start_recording: msg.start_recording,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      start_recording: msg.start_recording,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      start_recording: msg.start_recording,
    }
  }
}


// Corresponds to eufs_msgs__srv__RecordStop_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RecordStop_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for RecordStop_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RecordStop_Request::default())
  }
}

impl rosidl_runtime_rs::Message for RecordStop_Request {
  type RmwMsg = super::srv::rmw::RecordStop_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to eufs_msgs__srv__RecordStop_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RecordStop_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub stop_recording: bool,

}



impl Default for RecordStop_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RecordStop_Response::default())
  }
}

impl rosidl_runtime_rs::Message for RecordStop_Response {
  type RmwMsg = super::srv::rmw::RecordStop_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        stop_recording: msg.stop_recording,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      stop_recording: msg.stop_recording,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      stop_recording: msg.stop_recording,
    }
  }
}


// Corresponds to eufs_msgs__srv__Register_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Register_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub node_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub severity: u8,

}



impl Default for Register_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Register_Request::default())
  }
}

impl rosidl_runtime_rs::Message for Register_Request {
  type RmwMsg = super::srv::rmw::Register_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        node_name: msg.node_name.as_str().into(),
        severity: msg.severity,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        node_name: msg.node_name.as_str().into(),
      severity: msg.severity,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      node_name: msg.node_name.to_string(),
      severity: msg.severity,
    }
  }
}


// Corresponds to eufs_msgs__srv__Register_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Register_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: u8,

}



impl Default for Register_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Register_Response::default())
  }
}

impl rosidl_runtime_rs::Message for Register_Response {
  type RmwMsg = super::srv::rmw::Register_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      id: msg.id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id: msg.id,
    }
  }
}


// Corresponds to eufs_msgs__srv__SetCanState_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetCanState_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetCanState_Request {
  type RmwMsg = super::srv::rmw::SetCanState_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        ami_state: msg.ami_state,
        as_state: msg.as_state,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      ami_state: msg.ami_state,
      as_state: msg.as_state,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      ami_state: msg.ami_state,
      as_state: msg.as_state,
    }
  }
}


// Corresponds to eufs_msgs__srv__SetCanState_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetCanState_Response {
    /// indicate successful run of triggered service
    pub success: bool,

    /// informational, e.g. for error messages
    pub message: std::string::String,

}



impl Default for SetCanState_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetCanState_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetCanState_Response {
  type RmwMsg = super::srv::rmw::SetCanState_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to eufs_msgs__srv__SetString_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetString_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub data: std::string::String,

}



impl Default for SetString_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetString_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetString_Request {
  type RmwMsg = super::srv::rmw::SetString_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        data: msg.data.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        data: msg.data.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      data: msg.data.to_string(),
    }
  }
}


// Corresponds to eufs_msgs__srv__SetString_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetString_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for SetString_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetString_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetString_Response {
  type RmwMsg = super::srv::rmw::SetString_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to eufs_msgs__srv__SetTrack_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetTrack_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub data: super::msg::ConeArrayWithCovariance,

}



impl Default for SetTrack_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetTrack_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetTrack_Request {
  type RmwMsg = super::srv::rmw::SetTrack_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        data: super::msg::ConeArrayWithCovariance::into_rmw_message(std::borrow::Cow::Owned(msg.data)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        data: super::msg::ConeArrayWithCovariance::into_rmw_message(std::borrow::Cow::Borrowed(&msg.data)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      data: super::msg::ConeArrayWithCovariance::from_rmw_message(msg.data),
    }
  }
}


// Corresponds to eufs_msgs__srv__SetTrack_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetTrack_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for SetTrack_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetTrack_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetTrack_Response {
  type RmwMsg = super::srv::rmw::SetTrack_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to eufs_msgs__srv__SetMission_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetMission_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetMission_Request {
  type RmwMsg = super::srv::rmw::SetMission_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mission: msg.mission,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      mission: msg.mission,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mission: msg.mission,
    }
  }
}


// Corresponds to eufs_msgs__srv__SetMission_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetMission_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for SetMission_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetMission_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetMission_Response {
  type RmwMsg = super::srv::rmw::SetMission_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to eufs_msgs__srv__GetMap_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetMap_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetMap_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetMap_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetMap_Request {
  type RmwMsg = super::srv::rmw::GetMap_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to eufs_msgs__srv__GetMap_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetMap_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub map_path: Vec<std::string::String>,

}



impl Default for GetMap_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetMap_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetMap_Response {
  type RmwMsg = super::srv::rmw::GetMap_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        map_path: msg.map_path
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        map_path: msg.map_path
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      map_path: msg.map_path
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
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


