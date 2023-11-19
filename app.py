import requests
from flask import Flask, request

app = Flask(__name__)

def get_repo_structure(user, repo):
    try:
        url = f'https://api.github.com/repos/{user}/{repo}'
        response = requests.get(url)
        branch = response.json()['default_branch']

        url = f'https://api.github.com/repos/{user}/{repo}/git/trees/{branch}?recursive=1'
        response = requests.get(url)
        tree = response.json()['tree']
        path_list = [i['path'] for i in tree]
        result = '\n'.join(path_list)
    except:
        result = ''

    return result

def get_file_details(user, repo, path):
    try:
        url = f'https://api.github.com/repos/{user}/{repo}/contents/{path}'
        response = requests.get(url)
    
        url = response.json()['download_url']
        response = requests.get(url)

        if response.status_code == 200:
            return response.content.decode()
        else:
            raise
    except:
        return ''

@app.route("/structure")
def structure():
    user = request.args.get('user')
    repo = request.args.get('repo')
    
    result = get_repo_structure(user, repo)

    if not result:
        result = 'repo not found'
    return result

@app.route("/details")
def details():
    user = request.args.get('user')
    repo = request.args.get('repo')
    path_list = request.args.get('path_list')

    path_list = path_list.split(', ')

    result = ''
    for path in path_list:
        result += f'{path}:\n{get_file_details(user, repo, path)}\n'

    if not result:
        result = 'file(s) not found'
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

@app.route("/schema")
def schema():
    with open('schema.json', 'r') as file:
        return file.read()
