from os import getcwd, listdir
from os.path import exists, join
from flask import Flask, render_template, request

from compress import c_compress
from utils import read_file

app = Flask(__name__)

app.config['UPLOAD_FOLDER'] = join(getcwd(), 'data')
app.config['MAX_CONTENT_PATH'] = 512 * 1024 * 1024

print(listdir(getcwd()))


@app.route('/')
def home():
    return render_template('upload.html')


@app.route('/file', methods=['GET', 'POST'])
def file():
    if request.method == 'POST':
        f = request.files['file']
        print(request.data)

        data_file = join('data', f.filename)
        comp_file = join('data', f.filename + '.gz')
        if exists(comp_file):
            return read_file(comp_file)
        if not exists(data_file):
            f.save(data_file)
        return c_compress(data_file)
    return 'File Failed to Upload'


if __name__ == '__main__':
    app.run(debug=True)
