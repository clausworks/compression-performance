from datetime import datetime
from os import listdir
from os.path import join
import matplotlib.pyplot as plt
import numpy as np

path = join('httperf')
files = [join(path, x) for x in listdir(path)]
files.remove(join(path, 'backup'))

data = {}
for ix, file in enumerate(files):
    basename = file.split('\\')[-1]
    language, filename, rate = basename.rsplit('.', 1)[0].split(',')
    rate = int(rate)

    print(language, filename, rate, file)

    if filename not in data:
        data[filename] = {}
    if rate not in data[filename]:
        data[filename][rate] = {}
    if language not in data[filename][rate]:
        data[filename][rate][language] = {}

    with open(file, 'r') as f:
        file_data = f.readlines()

    connection_data = file_data[6].split(' ')
    min_connection_time = float(connection_data[4])
    avg_connection_time = float(connection_data[6])
    max_connection_time = float(connection_data[8])
    standard_deviation = float(connection_data[12])

    data[filename][rate][language][basename] = [min_connection_time, avg_connection_time, standard_deviation, max_connection_time]

files = []
for file_ix, filename in enumerate(data.keys()):
    files.append(filename)
    points = np.zeros((4, 40, 4))
    for rate_ix, rate in enumerate(sorted(data[filename].keys())):
        for language_ix, language in enumerate(data[filename][rate].keys()):
            for file_ix, file in enumerate(data[filename][rate][language].keys()):
                points[language_ix, rate_ix, 0] = int(rate)
                points[language_ix, rate_ix, 1] = data[filename][rate][language][file][1]
                points[language_ix, rate_ix, 2] = data[filename][rate][language][file][2]
                points[language_ix, rate_ix, 3] = data[filename][rate][language][file][2]

    print(points[0, :, 2:4].reshape((2, -1)))
    fig = plt.figure()
    plt.plot(points[0, :, 0], points[0, :, 1], color='red', label='C')
    plt.plot(points[1, :, 0], points[1, :, 1], color='green', label='C++')
    plt.plot(points[2, :, 0], points[2, :, 1], color='blue', label='Rust')
    plt.plot(points[3, :, 0], points[3, :, 1], color='yellow', label='Python')
    plt.title(f"Connection Time vs Connection Rate for {filename}")
    plt.xlabel("Connection Rate (Requests per Second)")
    plt.ylabel("Connection Time (ms)")
    plt.legend()
    fig.savefig(join('graphs', f'{filename}.png'))

print(len(files), files)