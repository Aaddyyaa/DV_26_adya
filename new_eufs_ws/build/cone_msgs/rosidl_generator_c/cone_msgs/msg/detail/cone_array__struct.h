// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from cone_msgs:msg/ConeArray.idl
// generated code does not contain a copyright notice

#ifndef CONE_MSGS__MSG__DETAIL__CONE_ARRAY__STRUCT_H_
#define CONE_MSGS__MSG__DETAIL__CONE_ARRAY__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'cones'
#include "cone_msgs/msg/detail/cone__struct.h"

/// Struct defined in msg/ConeArray in the package cone_msgs.
typedef struct cone_msgs__msg__ConeArray
{
  cone_msgs__msg__Cone__Sequence cones;
} cone_msgs__msg__ConeArray;

// Struct for a sequence of cone_msgs__msg__ConeArray.
typedef struct cone_msgs__msg__ConeArray__Sequence
{
  cone_msgs__msg__ConeArray * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} cone_msgs__msg__ConeArray__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // CONE_MSGS__MSG__DETAIL__CONE_ARRAY__STRUCT_H_
