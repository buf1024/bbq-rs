from setuptools import find_packages, setup

setup(
    name='qstrategy',
    version='0.0.1',
    packages=find_packages(include=['qstrategy']),
    include_package_data=True,
    zip_safe=False,
    platform="any",
    install_requires=[
        'pymongo',
        'numpy',
        'pandas'
    ],
)
