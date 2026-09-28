from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():
    return LaunchDescription([
        Node(
            package='my_state_estimator',
            executable='state_estimator_node',
            name='state_estimator',
            output='screen',
        ),
        Node(
            package='my_slam_pkg',
            executable='eufs_slam_node',
            name='fast_slam_node',
            output='screen',
            parameters=[{
                'odom_topic': '/odometry/filtered',
                'cones_topic': '/camera_0/cones',
                'min_landmark_hits': 1,
            }],
        ),
        Node(
            package='fsd_planner',
            executable='planner_node',
            name='path_planner',
            output='screen',
            parameters=[{
                'min_centerline_points': 2,
            }],
        ),
        Node(
            package='uncertainty_pipeline',
            executable='corridor_node',
            name='uncertainty_corridor',
            output='screen',
            parameters=[{
                'safety_k': 2.0,
                'grid_step_m': 0.5,
            }],
        ),
        Node(
            package='my_controller_pkg',
            executable='pure_pursuit_node',
            name='pure_pursuit_node',
            output='screen',
        ),
        Node(
            package='my_controller_pkg',
            executable='mission_manager',
            name='mission_manager',
            output='screen',
            parameters=[{'mission': 'trackdrive'}],
        ),
    ])
