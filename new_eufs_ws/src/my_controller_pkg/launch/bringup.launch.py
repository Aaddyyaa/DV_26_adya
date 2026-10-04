from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():
    # Canonical simulator bring-up for the uncertainty experiment.
    # The simulator already publishes covariance-bearing cones on
    # /camera_0/cones, so the custom YOLO/OpenCV perception node is deliberately
    # kept out of this launch. This removes the OpenCV/ONNX runtime dependency
    # from the Gazebo experiment while retaining the perception package for
    # camera-based development.
    sim_time = {'use_sim_time': True}

    return LaunchDescription([
        # EUFS publishes simulator joint states on /eufs/joint_states.
        # Relay them to the standard /joint_states topic consumed by
        # robot_state_publisher and the state estimator.
        Node(
            package='topic_tools',
            executable='relay',
            name='eufs_joint_states_relay',
            output='screen',
            arguments=['/eufs/joint_states', '/joint_states'],
        ),

        Node(
            package='my_state_estimator',
            executable='state_estimator_node',
            name='state_estimator',
            output='screen',
            parameters=[sim_time],
            remappings=[
                ('/joint_states', '/joint_states'),
                ('/odometry/filtered', '/custom_odom'),
            ],
        ),

        Node(
            package='my_slam_pkg',
            executable='eufs_slam_node',
            name='eufs_slam_node',
            output='screen',
            parameters=[{
                'use_sim_time': True,
                'cones_topic': '/camera_0/cones',
                'cones_topic_secondary': '/camera_1/cones',
                'cone_merge_distance': 0.45,
                'odom_topic': '/custom_odom',
                'min_landmark_hits': 1,
                'num_particles': 30,
                'landmark_match_distance': 0.75,
                'fallback_dt': 0.05,
            }],
        ),

        Node(
            package='fsd_planner',
            executable='planner_node',
            name='path_planner',
            output='screen',
            parameters=[{
                'use_sim_time': True,
                'planning_cones_topic': '/planning/cones',
                'odom_topic': '/slam/odom',
                'min_centerline_points': 2,
                'min_track_width_m': 1.5,
                'max_track_width_m': 6.0,
                'max_segment_length_m': 12.0,
                'allow_pair_reuse_fallback': True,
                'max_speed_mps': 1.5,
                'min_speed_mps': 0.6,
            }],
        ),

        Node(
            package='uncertainty_pipeline',
            executable='corridor_node',
            name='uncertainty_corridor',
            output='screen',
            parameters=[{
                'use_sim_time': True,
                'landmarks_topic': '/slam/landmarks',
                'odom_topic': '/slam/odom',
                'safety_k': 2.0,
                'grid_step_m': 0.5,
            }],
        ),

        Node(
            package='my_controller_pkg',
            executable='pure_pursuit_node',
            name='pure_pursuit_node',
            output='screen',
            parameters=[{
                'use_sim_time': True,
                'max_speed_limit': 1.5,
                'min_speed_mps': 0.6,
                'L_base': 1.53,
                'L_min': 1.5,
                'k_pure': 0.2,
            }],
        ),

        # Automatically request Track Drive once the EUFS state service is ready.
        # This removes the need to use Manual Drive or remember a GUI mission step.
        Node(
            package='my_controller_pkg',
            executable='mission_manager',
            name='mission_manager',
            output='screen',
            parameters=[{
                'use_sim_time': True,
                'enable': True,
                'mission': 'trackdrive',
            }],
        ),
    ])
