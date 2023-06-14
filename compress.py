from platform import system
from subprocess import run
from utils import read_file

# TODO: Get System Info and run the correct executable


def c_compress(filename):
    run(['lzw_c/linux-x64.out', filename])
    return read_file(filename + '.lzw')


if __name__ == "__main__":
    cdata = c_compress('data/test.txt')
    print(cdata)
