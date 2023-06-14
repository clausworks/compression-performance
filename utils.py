def read_file(filename):
    with open(filename, 'rb') as f:
        return f.read()


def write_file(filename, data):
    with open(filename, 'wb') as f:
        f.write(data)
