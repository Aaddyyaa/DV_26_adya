from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():
    return LaunchDescription([
        Node(
            package='uncertainty_pipeline',
            executable='corridor_node',
            name='uncertainty_corridor',
            output='screen',
            parameters=[{'safety_k': 2.0, 'grid_step_m': 0.5}],
        ),
    ])
