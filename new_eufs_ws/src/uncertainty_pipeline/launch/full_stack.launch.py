from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():
    # Keep a single canonical launch path. The EUFS simulator provides
    # covariance-bearing cone observations on /camera_0/cones.
    return LaunchDescription([
        Node(
            package='my_state_estimator',
            executable='state_estimator_node',
            name='state_estimator',
            output='screen',
            parameters=[{'use_sim_time': True}],
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
                'odom_topic': '/slam/odom',
                'min_landmark_hits': 1,
                'num_particles': 30,
                'landmark_match_distance': 1.0,
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
                'min_track_width_m': 2.0,
                'max_track_width_m': 5.5,
                'max_segment_length_m': 6.0,
                'nominal_track_width_m': 4.5,
                'max_pair_station_gap_m': 2.5,
                'max_gate_turn_deg': 95.0,
                'local_range_m': 18.0,
                'local_half_width_m': 9.0,
                'path_sample_step_m': 0.5,
                'max_path_jump_m': 3.5,
                'path_hold_time_sec': 8.0,
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
                'odom_topic': '/slam/odom',
                'max_speed_limit': 1.5,
                'min_speed_mps': 0.6,
                'L_base': 1.53,
                'L_min': 1.8,
                'k_pure': 0.15,
                'max_accel': 1.5,
                'max_steering': 0.28,
                'max_tracking_error_m': 1.50,
                'max_steering_rate_rad_s': 1.5,
            }],
        ),
    ])
