// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from cone_msgs:msg/Cone.idl
// generated code does not contain a copyright notice

#ifndef CONE_MSGS__MSG__DETAIL__CONE__TRAITS_HPP_
#define CONE_MSGS__MSG__DETAIL__CONE__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "cone_msgs/msg/detail/cone__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace cone_msgs
{

namespace msg
{

inline void to_flow_style_yaml(
  const Cone & msg,
  std::ostream & out)
{
  out << "{";
  // member: x
  {
    out << "x: ";
    rosidl_generator_traits::value_to_yaml(msg.x, out);
    out << ", ";
  }

  // member: y
  {
    out << "y: ";
    rosidl_generator_traits::value_to_yaml(msg.y, out);
    out << ", ";
  }

  // member: z
  {
    out << "z: ";
    rosidl_generator_traits::value_to_yaml(msg.z, out);
    out << ", ";
  }

  // member: class_id
  {
    out << "class_id: ";
    rosidl_generator_traits::value_to_yaml(msg.class_id, out);
    out << ", ";
  }

  // member: confidence
  {
    out << "confidence: ";
    rosidl_generator_traits::value_to_yaml(msg.confidence, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const Cone & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: x
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "x: ";
    rosidl_generator_traits::value_to_yaml(msg.x, out);
    out << "\n";
  }

  // member: y
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "y: ";
    rosidl_generator_traits::value_to_yaml(msg.y, out);
    out << "\n";
  }

  // member: z
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "z: ";
    rosidl_generator_traits::value_to_yaml(msg.z, out);
    out << "\n";
  }

  // member: class_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "class_id: ";
    rosidl_generator_traits::value_to_yaml(msg.class_id, out);
    out << "\n";
  }

  // member: confidence
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "confidence: ";
    rosidl_generator_traits::value_to_yaml(msg.confidence, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const Cone & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace msg

}  // namespace cone_msgs

namespace rosidl_generator_traits
{

[[deprecated("use cone_msgs::msg::to_block_style_yaml() instead")]]
inline void to_yaml(
  const cone_msgs::msg::Cone & msg,
  std::ostream & out, size_t indentation = 0)
{
  cone_msgs::msg::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use cone_msgs::msg::to_yaml() instead")]]
inline std::string to_yaml(const cone_msgs::msg::Cone & msg)
{
  return cone_msgs::msg::to_yaml(msg);
}

template<>
inline const char * data_type<cone_msgs::msg::Cone>()
{
  return "cone_msgs::msg::Cone";
}

template<>
inline const char * name<cone_msgs::msg::Cone>()
{
  return "cone_msgs/msg/Cone";
}

template<>
struct has_fixed_size<cone_msgs::msg::Cone>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<cone_msgs::msg::Cone>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<cone_msgs::msg::Cone>
  : std::true_type {};

}  // namespace rosidl_generator_traits

#endif  // CONE_MSGS__MSG__DETAIL__CONE__TRAITS_HPP_
