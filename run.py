from sanic import Sanic
from sanic.response import text
from src.utils import get_repo_structure, get_file_details

app = Sanic(__name__)


@app.route("/structure")
async def structure(request):
    user = request.args.get('user')
    repo = request.args.get('repo')
    
    result = await get_repo_structure(user, repo)

    return text(result)


@app.route("/details")
async def details(request):
    user = request.args.get('user')
    repo = request.args.get('repo')
    path_list = request.args.get('path_list')

    path_list = path_list.split(', ')

    result = ''
    for path in path_list:
        result += f'{path}:\n{await get_file_details(user, repo, path)}\n\n'

    return text(result)


@app.route("/privacy")
async def privacy(request):
    with open('src/static/privacy.txt', 'r') as file:
        return text(file.read())


@app.route("/schema")
async def schema(request):
    with open('src/static/schema.json', 'r') as file:
        return text(file.read())
