// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from cone_msgs:msg/Cone.idl
// generated code does not contain a copyright notice

#ifndef CONE_MSGS__MSG__DETAIL__CONE__STRUCT_H_
#define CONE_MSGS__MSG__DETAIL__CONE__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Struct defined in msg/Cone in the package cone_msgs.
typedef struct cone_msgs__msg__Cone
{
  float x;
  float y;
  float z;
  int32_t class_id;
  float confidence;
} cone_msgs__msg__Cone;

// Struct for a sequence of cone_msgs__msg__Cone.
typedef struct cone_msgs__msg__Cone__Sequence
{
  cone_msgs__msg__Cone * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} cone_msgs__msg__Cone__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // CONE_MSGS__MSG__DETAIL__CONE__STRUCT_H_
