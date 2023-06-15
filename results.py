from datetime import datetime
from os import listdir
from os.path import join

import numpy as np

path = join('httperf', 'backup')
files = [join(path, x) for x in listdir(path)]

points = np.zeros((43, 3), dtype=np.float64)
for ix, file in enumerate(files):
    file_ix = ix % 43
    language_ix = ix // 43
    basename = file.split('\\')[-1]
    language, filename = basename.rsplit('.', 1)[0].split(',')
    with open(file, 'r') as f:
        data = f.readlines()
    connection_time = float(data[6].split(' ')[6])
    points[file_ix, language_ix] = connection_time
print(points)
