import sys
import lzw


def main(args):
    if len(args) not in (2, 3):
        print('Usage: python lzw.py <filename>')
        return
    filename = args[1]
    data = lzw.readbytes(filename)
    compressed = lzw.compress(data)
    lzw.writebytes(filename + '.lzw', compressed)


if __name__ == '__main__':
    main(sys.argv)
