import sys
if sys.prefix == '/usr':
    sys.real_prefix = sys.prefix
    sys.prefix = sys.exec_prefix = '/home/alr/Desktop/new_perception/new_eufs_ws/install/fsd_planner'
