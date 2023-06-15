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
    filename = basename(file)
    if filename not in ['alice29.txt', 'asyoulik.txt', 'badfile', 'cp.html', 'ctest01', 'ctest02', 'ctest02a', 'ctest03', 'ctest04', 'fields.c', 'grammar.lsp', 'heic0611b.tif', 'kennedy.xls', 'lcet10.txt', 'plrabn12.txt', 'ptt5', 'sum', 'xargs.1']:
        continue
    for language in languages:
        write_sess(file, language)
        for rate in range(3100, 4100, 100):
            data = check_output(f'httperf --port=5000 --rate {rate} --wsesslog={rate},0.25,sess.txt'.split(' '))
            with open(join('httperf', f'{language},{basename(file)},{rate}.txt'), 'wb') as f:
                f.write(data)
