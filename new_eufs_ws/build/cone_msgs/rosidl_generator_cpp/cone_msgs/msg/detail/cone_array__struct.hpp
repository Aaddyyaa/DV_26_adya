// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from cone_msgs:msg/ConeArray.idl
// generated code does not contain a copyright notice

#ifndef CONE_MSGS__MSG__DETAIL__CONE_ARRAY__STRUCT_HPP_
#define CONE_MSGS__MSG__DETAIL__CONE_ARRAY__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


// Include directives for member types
// Member 'cones'
#include "cone_msgs/msg/detail/cone__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__cone_msgs__msg__ConeArray __attribute__((deprecated))
#else
# define DEPRECATED__cone_msgs__msg__ConeArray __declspec(deprecated)
#endif

namespace cone_msgs
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct ConeArray_
{
  using Type = ConeArray_<ContainerAllocator>;

  explicit ConeArray_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_init;
  }

  explicit ConeArray_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_init;
    (void)_alloc;
  }

  // field types and members
  using _cones_type =
    std::vector<cone_msgs::msg::Cone_<ContainerAllocator>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<cone_msgs::msg::Cone_<ContainerAllocator>>>;
  _cones_type cones;

  // setters for named parameter idiom
  Type & set__cones(
    const std::vector<cone_msgs::msg::Cone_<ContainerAllocator>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<cone_msgs::msg::Cone_<ContainerAllocator>>> & _arg)
  {
    this->cones = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    cone_msgs::msg::ConeArray_<ContainerAllocator> *;
  using ConstRawPtr =
    const cone_msgs::msg::ConeArray_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<cone_msgs::msg::ConeArray_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<cone_msgs::msg::ConeArray_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      cone_msgs::msg::ConeArray_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<cone_msgs::msg::ConeArray_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      cone_msgs::msg::ConeArray_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<cone_msgs::msg::ConeArray_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<cone_msgs::msg::ConeArray_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<cone_msgs::msg::ConeArray_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__cone_msgs__msg__ConeArray
    std::shared_ptr<cone_msgs::msg::ConeArray_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__cone_msgs__msg__ConeArray
    std::shared_ptr<cone_msgs::msg::ConeArray_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const ConeArray_ & other) const
  {
    if (this->cones != other.cones) {
      return false;
    }
    return true;
  }
  bool operator!=(const ConeArray_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct ConeArray_

// alias to use template instance with default allocator
using ConeArray =
  cone_msgs::msg::ConeArray_<std::allocator<void>>;

// constant definitions

}  // namespace msg

}  // namespace cone_msgs

#endif  // CONE_MSGS__MSG__DETAIL__CONE_ARRAY__STRUCT_HPP_
