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
                'odom_topic': '/custom_odom',
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
                'max_speed_mps': 2.0,
                'allow_pair_reuse_fallback': True,
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
                'max_speed_limit': 2.0,
                'min_speed_mps': 1.0,
            }],
        ),
    ])
