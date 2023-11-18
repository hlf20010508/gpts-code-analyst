From python:3.8.10
WORKDIR /workdir
COPY ./ ./
RUN pip install --no-cache-dir requests flask
