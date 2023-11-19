from aiohttp import ClientSession


async def get_repo_structure(user, repo):
    try:
        async with ClientSession() as session:
            url = f'https://api.github.com/repos/{user}/{repo}'
            async with session.get(url) as response:
                branch = (await response.json())['default_branch']

            url = f'https://api.github.com/repos/{user}/{repo}/git/trees/{branch}?recursive=1'
            async with session.get(url) as response:
                tree = (await response.json())['tree']

        path_list = [i['path'] for i in tree]
        result = '\n'.join(path_list)

    except:
        result = 'repo not found'

    return result


async def get_file_details(user, repo, path):
    try:
        async with ClientSession() as session:
            url = f'https://api.github.com/repos/{user}/{repo}/contents/{path}'
            async with session.get(url) as response:
                url = (await response.json())['download_url']
            
            async with session.get(url) as response:
                if response.status == 200:
                    return await response.text()
                else:
                    raise
    except:
        return 'file not found'