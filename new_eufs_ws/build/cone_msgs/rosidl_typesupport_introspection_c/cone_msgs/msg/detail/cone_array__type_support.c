// generated from rosidl_typesupport_introspection_c/resource/idl__type_support.c.em
// with input from cone_msgs:msg/ConeArray.idl
// generated code does not contain a copyright notice

#include <stddef.h>
#include "cone_msgs/msg/detail/cone_array__rosidl_typesupport_introspection_c.h"
#include "cone_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
#include "rosidl_typesupport_introspection_c/field_types.h"
#include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/message_introspection.h"
#include "cone_msgs/msg/detail/cone_array__functions.h"
#include "cone_msgs/msg/detail/cone_array__struct.h"


// Include directives for member types
// Member `cones`
#include "cone_msgs/msg/cone.h"
// Member `cones`
#include "cone_msgs/msg/detail/cone__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void cone_msgs__msg__ConeArray__rosidl_typesupport_introspection_c__ConeArray_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  cone_msgs__msg__ConeArray__init(message_memory);
}

void cone_msgs__msg__ConeArray__rosidl_typesupport_introspection_c__ConeArray_fini_function(void * message_memory)
{
  cone_msgs__msg__ConeArray__fini(message_memory);
}

size_t cone_msgs__msg__ConeArray__rosidl_typesupport_introspection_c__size_function__ConeArray__cones(
  const void * untyped_member)
{
  const cone_msgs__msg__Cone__Sequence * member =
    (const cone_msgs__msg__Cone__Sequence *)(untyped_member);
  return member->size;
}

const void * cone_msgs__msg__ConeArray__rosidl_typesupport_introspection_c__get_const_function__ConeArray__cones(
  const void * untyped_member, size_t index)
{
  const cone_msgs__msg__Cone__Sequence * member =
    (const cone_msgs__msg__Cone__Sequence *)(untyped_member);
  return &member->data[index];
}

void * cone_msgs__msg__ConeArray__rosidl_typesupport_introspection_c__get_function__ConeArray__cones(
  void * untyped_member, size_t index)
{
  cone_msgs__msg__Cone__Sequence * member =
    (cone_msgs__msg__Cone__Sequence *)(untyped_member);
  return &member->data[index];
}

void cone_msgs__msg__ConeArray__rosidl_typesupport_introspection_c__fetch_function__ConeArray__cones(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const cone_msgs__msg__Cone * item =
    ((const cone_msgs__msg__Cone *)
    cone_msgs__msg__ConeArray__rosidl_typesupport_introspection_c__get_const_function__ConeArray__cones(untyped_member, index));
  cone_msgs__msg__Cone * value =
    (cone_msgs__msg__Cone *)(untyped_value);
  *value = *item;
}

void cone_msgs__msg__ConeArray__rosidl_typesupport_introspection_c__assign_function__ConeArray__cones(
  void * untyped_member, size_t index, const void * untyped_value)
{
  cone_msgs__msg__Cone * item =
    ((cone_msgs__msg__Cone *)
    cone_msgs__msg__ConeArray__rosidl_typesupport_introspection_c__get_function__ConeArray__cones(untyped_member, index));
  const cone_msgs__msg__Cone * value =
    (const cone_msgs__msg__Cone *)(untyped_value);
  *item = *value;
}

bool cone_msgs__msg__ConeArray__rosidl_typesupport_introspection_c__resize_function__ConeArray__cones(
  void * untyped_member, size_t size)
{
  cone_msgs__msg__Cone__Sequence * member =
    (cone_msgs__msg__Cone__Sequence *)(untyped_member);
  cone_msgs__msg__Cone__Sequence__fini(member);
  return cone_msgs__msg__Cone__Sequence__init(member, size);
}

static rosidl_typesupport_introspection_c__MessageMember cone_msgs__msg__ConeArray__rosidl_typesupport_introspection_c__ConeArray_message_member_array[1] = {
  {
    "cones",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    true,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(cone_msgs__msg__ConeArray, cones),  // bytes offset in struct
    NULL,  // default value
    cone_msgs__msg__ConeArray__rosidl_typesupport_introspection_c__size_function__ConeArray__cones,  // size() function pointer
    cone_msgs__msg__ConeArray__rosidl_typesupport_introspection_c__get_const_function__ConeArray__cones,  // get_const(index) function pointer
    cone_msgs__msg__ConeArray__rosidl_typesupport_introspection_c__get_function__ConeArray__cones,  // get(index) function pointer
    cone_msgs__msg__ConeArray__rosidl_typesupport_introspection_c__fetch_function__ConeArray__cones,  // fetch(index, &value) function pointer
    cone_msgs__msg__ConeArray__rosidl_typesupport_introspection_c__assign_function__ConeArray__cones,  // assign(index, value) function pointer
    cone_msgs__msg__ConeArray__rosidl_typesupport_introspection_c__resize_function__ConeArray__cones  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers cone_msgs__msg__ConeArray__rosidl_typesupport_introspection_c__ConeArray_message_members = {
  "cone_msgs__msg",  // message namespace
  "ConeArray",  // message name
  1,  // number of fields
  sizeof(cone_msgs__msg__ConeArray),
  cone_msgs__msg__ConeArray__rosidl_typesupport_introspection_c__ConeArray_message_member_array,  // message members
  cone_msgs__msg__ConeArray__rosidl_typesupport_introspection_c__ConeArray_init_function,  // function to initialize message memory (memory has to be allocated)
  cone_msgs__msg__ConeArray__rosidl_typesupport_introspection_c__ConeArray_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t cone_msgs__msg__ConeArray__rosidl_typesupport_introspection_c__ConeArray_message_type_support_handle = {
  0,
  &cone_msgs__msg__ConeArray__rosidl_typesupport_introspection_c__ConeArray_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_cone_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, cone_msgs, msg, ConeArray)() {
  cone_msgs__msg__ConeArray__rosidl_typesupport_introspection_c__ConeArray_message_member_array[0].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, cone_msgs, msg, Cone)();
  if (!cone_msgs__msg__ConeArray__rosidl_typesupport_introspection_c__ConeArray_message_type_support_handle.typesupport_identifier) {
    cone_msgs__msg__ConeArray__rosidl_typesupport_introspection_c__ConeArray_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &cone_msgs__msg__ConeArray__rosidl_typesupport_introspection_c__ConeArray_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif
