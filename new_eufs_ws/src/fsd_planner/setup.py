from setuptools import setup
import os
from glob import glob

package_name = 'fsd_planner'

setup(
    name=package_name,
    version='0.0.0',

    packages=[
        package_name,
    ],

    data_files=[
        (
            'share/ament_index/resource_index/packages',
            ['resource/' + package_name]
        ),
        (
            'share/' + package_name,
            ['package.xml']
        ),
        (
            os.path.join('share', package_name, 'launch'),
            glob('launch/*.launch.py')
        ),
    ],

    install_requires=['setuptools'],

    zip_safe=True,

    maintainer='mayuresh',
    maintainer_email='mayuresh@todo.todo',

    description='Path planning wrapper for EUFS using ft-fsd algorithm',

    license='Apache-2.0',

    tests_require=['pytest'],

    entry_points={
        'console_scripts': [
            'planner_node = fsd_planner.planner_node:main',
        ],
    },
)