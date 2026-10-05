import os

from launch import LaunchDescription
from launch.actions import SetEnvironmentVariable
from launch_ros.actions import Node


def generate_launch_description():
    # Official EUFS launcher GUI. It provides the track dropdown and Launch
    # button for starting the selected EUFS simulation track.
    # The EUFS launch stack expects EUFS_MASTER to point at this workspace.
    workspace = os.path.expanduser("~/projects/DV_26/new_eufs_ws")

    return LaunchDescription([
        SetEnvironmentVariable("EUFS_MASTER", workspace),
        Node(
            package='eufs_launcher',
            executable='eufs_launcher',
            name='eufs_launcher',
            output='screen',
            parameters=[{'use_sim_time': True}],
        ),
    ])
