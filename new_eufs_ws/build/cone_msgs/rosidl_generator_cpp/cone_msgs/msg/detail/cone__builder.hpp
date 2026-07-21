// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from cone_msgs:msg/Cone.idl
// generated code does not contain a copyright notice

#ifndef CONE_MSGS__MSG__DETAIL__CONE__BUILDER_HPP_
#define CONE_MSGS__MSG__DETAIL__CONE__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "cone_msgs/msg/detail/cone__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace cone_msgs
{

namespace msg
{

namespace builder
{

class Init_Cone_confidence
{
public:
  explicit Init_Cone_confidence(::cone_msgs::msg::Cone & msg)
  : msg_(msg)
  {}
  ::cone_msgs::msg::Cone confidence(::cone_msgs::msg::Cone::_confidence_type arg)
  {
    msg_.confidence = std::move(arg);
    return std::move(msg_);
  }

private:
  ::cone_msgs::msg::Cone msg_;
};

class Init_Cone_class_id
{
public:
  explicit Init_Cone_class_id(::cone_msgs::msg::Cone & msg)
  : msg_(msg)
  {}
  Init_Cone_confidence class_id(::cone_msgs::msg::Cone::_class_id_type arg)
  {
    msg_.class_id = std::move(arg);
    return Init_Cone_confidence(msg_);
  }

private:
  ::cone_msgs::msg::Cone msg_;
};

class Init_Cone_z
{
public:
  explicit Init_Cone_z(::cone_msgs::msg::Cone & msg)
  : msg_(msg)
  {}
  Init_Cone_class_id z(::cone_msgs::msg::Cone::_z_type arg)
  {
    msg_.z = std::move(arg);
    return Init_Cone_class_id(msg_);
  }

private:
  ::cone_msgs::msg::Cone msg_;
};

class Init_Cone_y
{
public:
  explicit Init_Cone_y(::cone_msgs::msg::Cone & msg)
  : msg_(msg)
  {}
  Init_Cone_z y(::cone_msgs::msg::Cone::_y_type arg)
  {
    msg_.y = std::move(arg);
    return Init_Cone_z(msg_);
  }

private:
  ::cone_msgs::msg::Cone msg_;
};

class Init_Cone_x
{
public:
  Init_Cone_x()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_Cone_y x(::cone_msgs::msg::Cone::_x_type arg)
  {
    msg_.x = std::move(arg);
    return Init_Cone_y(msg_);
  }

private:
  ::cone_msgs::msg::Cone msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::cone_msgs::msg::Cone>()
{
  return cone_msgs::msg::builder::Init_Cone_x();
}

}  // namespace cone_msgs

#endif  // CONE_MSGS__MSG__DETAIL__CONE__BUILDER_HPP_
