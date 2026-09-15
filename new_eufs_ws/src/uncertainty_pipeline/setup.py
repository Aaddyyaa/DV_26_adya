from glob import glob
import os

from setuptools import setup


package_name = 'uncertainty_pipeline'


setup(
    name=package_name,
    version='0.1.0',
    packages=[package_name],
    data_files=[
        ('share/ament_index/resource_index/packages', ['resource/' + package_name]),
        ('share/' + package_name, ['package.xml']),
        (os.path.join('share', package_name, 'launch'), glob('launch/*.launch.py')),
    ],
    install_requires=['setuptools'],
    zip_safe=True,
    maintainer='DV Team',
    maintainer_email='maintainer@example.com',
    description='Uncertainty propagation and safe-corridor tools for the DV stack.',
    license='Apache-2.0',
    entry_points={
        'console_scripts': [
            'corridor_node = uncertainty_pipeline.corridor_node:main',
            'noise_injector = uncertainty_pipeline.noise_injector:main',
            'experiment_logger = uncertainty_pipeline.experiment_logger:main',
        ],
    },
)
