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
            package='my_controller_pkg',
            executable='joint_state_relay',
            name='eufs_joint_state_relay',
            output='screen',
            parameters=[sim_time],
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
                'landmark_consolidation_distance': 0.65,
                'landmark_confirmation_hits': 3,
                'max_landmark_missed_updates': 20,
                'negative_evidence_range_m': 15.0,
                'negative_evidence_fov_deg': 120.0,
                'landmark_publish_stride': 2,
                'odom_topic': '/custom_odom',
                'min_landmark_hits': 3,
                'num_particles': 15,
                'landmark_match_distance': 0.75,
                'local_cone_smoothing_alpha': 0.55,
                'local_cone_match_distance_m': 1.0,
                'local_cone_hold_sec': 1.0,
                'planning_landmark_max_missed_updates': 7,
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
                'allow_pair_reuse_fallback': False,
                'cone_dedup_distance_m': 0.75,
                'path_hold_time_sec': 2.5,
                'degraded_path_hold_sec': 1.0,
                'degraded_path_ratio': 0.55,
                'degraded_path_min_m': 4.0,
                'max_speed_mps': 2.0,
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
                'dedup_distance_m': 0.75,
                # Keep uncertainty visualisation/metrics on the same local
                # observable horizon as the cone-driven stack.
                'local_forward_m': 18.0,
                'local_lateral_m': 10.0,
                'local_fov_deg': 110.0,
            }],
        ),

        Node(
            package='my_controller_pkg',
            executable='pure_pursuit_node',
            name='pure_pursuit_node',
            output='screen',
            parameters=[{
                'use_sim_time': True,
                'max_speed_limit': 2.0,
                'min_speed_mps': 0.45,
                'corner_steering_start': 0.22,
                'corner_steering_full': 0.45,
                'corner_speed_cap': 0.85,
                'L_base': 1.53,
                'L_min': 1.9,
                'k_pure': 0.25,
            }],
        ),

        Node(
            package='uncertainty_pipeline',
            executable='run_metrics_logger',
            name='run_metrics_logger',
            output='screen',
            parameters=[{
                'use_sim_time': True,
                'output_csv': 'experiments/raw/run_metrics.csv',
            }],
        ),

        Node(
            package='my_controller_pkg',
            executable='lap_guard_node',
            name='lap_guard',
            output='screen',
            parameters=[{
                'use_sim_time': True,
                'min_distance_before_finish_m': 60.0,
                'finish_radius_m': 3.0,
                'finish_line_half_width_m': 3.0,
                'finish_heading_tolerance_rad': 1.6,
            }],
        ),

        # Mission selection remains owned by the official EUFS GUI.
        # This removes the need to use Manual Drive or remember a GUI mission step.
        Node(
            package='my_controller_pkg',
            executable='mission_manager',
            name='mission_manager',
            output='screen',
            parameters=[{
                'use_sim_time': True,
                'enable': False,
                'mission': 'trackdrive',
            }],
        ),
    ])
