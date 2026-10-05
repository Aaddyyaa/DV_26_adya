from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():
    # Official EUFS launcher GUI. It provides the track dropdown and Launch
    # button for starting the selected EUFS simulation track.
    return LaunchDescription([
        Node(
            package='eufs_launcher',
            executable='eufs_launcher',
            name='eufs_launcher',
            output='screen',
            parameters=[{'use_sim_time': True}],
        ),
    ])
