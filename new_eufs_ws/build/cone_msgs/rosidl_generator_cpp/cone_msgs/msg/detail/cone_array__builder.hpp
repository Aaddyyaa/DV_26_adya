// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from cone_msgs:msg/ConeArray.idl
// generated code does not contain a copyright notice

#ifndef CONE_MSGS__MSG__DETAIL__CONE_ARRAY__BUILDER_HPP_
#define CONE_MSGS__MSG__DETAIL__CONE_ARRAY__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "cone_msgs/msg/detail/cone_array__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace cone_msgs
{

namespace msg
{

namespace builder
{

class Init_ConeArray_cones
{
public:
  Init_ConeArray_cones()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::cone_msgs::msg::ConeArray cones(::cone_msgs::msg::ConeArray::_cones_type arg)
  {
    msg_.cones = std::move(arg);
    return std::move(msg_);
  }

private:
  ::cone_msgs::msg::ConeArray msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::cone_msgs::msg::ConeArray>()
{
  return cone_msgs::msg::builder::Init_ConeArray_cones();
}

}  // namespace cone_msgs

#endif  // CONE_MSGS__MSG__DETAIL__CONE_ARRAY__BUILDER_HPP_
