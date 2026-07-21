from setuptools import find_packages
from setuptools import setup

setup(
    name='cone_msgs',
    version='0.0.0',
    packages=find_packages(
        include=('cone_msgs', 'cone_msgs.*')),
)
