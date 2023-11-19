FROM python:3.8.10
WORKDIR /workdir
COPY ./ ./
RUN pip install --no-cache-dir sanic aiohttp
CMD sanic run.app -H 0.0.0.0 -p 8080
