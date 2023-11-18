import requests
import os
from flask import Flask, request

app = Flask(__name__)

def get_repo_contents(user, repo, path=''):
    url = f"https://api.github.com/repos/{user}/{repo}/contents/{path}"
    response = requests.get(url)
    if response.status_code == 200:
        return response.json()
    else:
        return None

def get_directory_structure(user, repo, parent=''):
    contents = get_repo_contents(user, repo, parent)
    result = ''
    if contents is not None:
        for item in contents:
            if item['type'] == 'dir':
                result += get_directory_structure(user, repo, item['path'])
            else:
                result += os.path.join(parent, item['name']) + '\n'
    return result

def get_file_details(user, repo, path):
    url = f'https://api.github.com/repos/{user}/{repo}/contents/{path}'
    response = requests.get(url)
    url = response.json()['download_url']
    response = requests.get(url)

    if response.status_code == 200:
        return response.content.decode()
    else:
        return ''

@app.route("/structure")
def structure():
    user = request.args.get('user')
    repo = request.args.get('repo')
    
    result = get_directory_structure(user, repo)

    if not result:
        result = 'repo not found'
    return result

@app.route("/details")
def details():
    user = request.args.get('user')
    repo = request.args.get('repo')
    path = request.args.get('path')

    result = get_file_details(user, repo, path)

    if not result:
        result = 'file not found'
    return result

@app.route("/privacy")
def privacy():
    return '''
<h1>MIT License</h1>
<h2>Copyright (c) 2023 L-ING<h2>
<p>
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
</p>
<p>
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
</p>
<p>
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
</p>
'''
