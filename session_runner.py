from os import listdir
from os.path import basename, isdir, join
from subprocess import Popen, check_output, run

languages = ['C', 'C++', 'Rust', 'Python']


def glob_files(path):
    out = []
    for file in listdir(path):
        file_path = join(path, file)
        if file.endswith('lzw'):
            continue
        if isdir(file_path):
            out += glob_files(file_path)
        else:
            out.append(file_path)
    return out


def write_sess(file_path, language):
    with open('sess.txt', 'w') as f:
        f.write(f'/\n/file method=POST contents="{language}:{file_path}"')


files = list(glob_files(join('tests', 'files')))
for file in files:
    for language in languages:
        write_sess(file, language)
        data = check_output('httperf --port=5000 --wsesslog=50,0.25,sess.txt'.split(' '))
        with open(join('httperf', f'{language},{basename(file)}.txt'), 'wb') as f:
            f.write(data)
