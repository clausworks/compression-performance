from platform import system
from subprocess import check_output, run
from utils import read_file
import lzw

# TODO: Get System Info and run the correct executable


def c_compress(filename):
    run(['lzw_c/linux-x64.out', filename])
    return read_file(filename + '.lzw')


def cpp_compress(filename):
    run(['lzw_cpp/lzw', '-c', filename, filename + '.lzw'])
    return read_file(filename + '.lzw')


def rust_compress(filename):
    run(['lzw_rust/target/debug/lzw_rust', filename])
    return read_file(filename + '.lzw')


def python_compress(filename):
    run(['python', 'lzw_python/lzw.py', filename])
    return read_file(filename + '.lzw')


if __name__ == "__main__":
    cdata = c_compress('data/test.txt')
    print(cdata)
