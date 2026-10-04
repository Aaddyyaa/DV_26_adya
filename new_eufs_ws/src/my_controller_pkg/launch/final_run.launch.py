import os
from pathlib import Path

from ament_index_python.packages import get_package_share_directory

from launch import LaunchDescription
from launch.actions import IncludeLaunchDescription, OpaqueFunction, SetEnvironmentVariable
from launch.launch_description_sources import AnyLaunchDescriptionSource, PythonLaunchDescriptionSource


def patch_eufs_before_start(context, *args, **kwargs):
    """Fix only RViz/TF presentation issues before EUFS spawns the vehicle."""
    racecar_share = Path(get_package_share_directory("eufs_racecar"))
    xacro_path = racecar_share / "urdf" / "wheels.urdf.xacro"
    load_car_path = racecar_share / "launch" / "load_car.launch.py"

    if xacro_path.is_file():
        text = xacro_path.read_text()
        if '<material name="tire_mat"/>' in text and '<material name="tire_mat">' not in text:
            backup = xacro_path.with_name(xacro_path.name + ".final_backup")
            if not backup.exists():
                backup.write_text(text)
            text = text.replace(
                '<material name="tire_mat"/>',
                '<material name="tire_mat">\n'
                '            <color rgba="0.02 0.02 0.02 1.0"/>\n'
                '          </material>',
                1,
            )
            xacro_path.write_text(text)

    if load_car_path.is_file():
        text = load_car_path.read_text()
        patched = text.replace(
            "'robot_description': robot_description,\n                'rate': 200,",
            "'robot_description': robot_description,\n"
            "                'use_sim_time': True,\n"
            "                'rate': 200,",
        )
        if patched != text:
            backup = load_car_path.with_name(load_car_path.name + ".final_backup")
            if not backup.exists():
                backup.write_text(text)
            load_car_path.write_text(patched)

    return []


def generate_launch_description():
    eufs_tracks_share = Path(
        get_package_share_directory("eufs_tracks")
    )
    # EUFS small_track.launch uses $(env EUFS_MASTER) to locate the simulator
    # plugins. Set it here so the launch works from a fresh terminal and does
    # not depend on a manually exported shell variable.
    eufs_master = str(eufs_tracks_share.parents[3])

    eufs_sim = os.path.join(
        str(eufs_tracks_share),
        "launch",
        "small_track.launch",
    )
    bringup = os.path.join(
        get_package_share_directory("my_controller_pkg"),
        "launch",
        "bringup.launch.py",
    )

    return LaunchDescription([
        SetEnvironmentVariable("EUFS_MASTER", eufs_master),
        OpaqueFunction(function=patch_eufs_before_start),

        IncludeLaunchDescription(
            AnyLaunchDescriptionSource(eufs_sim),
            launch_arguments={
                "gazebo_gui": "false",
                "rviz": "true",
                "show_rqt_gui": "true",
                "vehicleModel": "DynamicBicycle",
                "commandMode": "acceleration",
                "robot_name": "eufs",
                "vehicleModelConfig": "configDry.yaml",
                "publish_gt_tf": "false",
                "pub_ground_truth": "true",
                "x": "-13.0",
                "y": "10.3",
                "z": "0.1",
                "roll": "0.0",
                "pitch": "0.0",
                "yaw": "0.0",
            }.items(),
        ),

        IncludeLaunchDescription(
            PythonLaunchDescriptionSource(bringup),
        ),
    ])


if __name__ == "__main__":
    generate_launch_description()
