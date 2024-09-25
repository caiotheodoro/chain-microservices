FROM python:3.11.4

RUN apt-get update \
  && apt-get install -y --no-install-recommends graphviz libsnappy-dev \
  && rm -rf /var/lib/apt/lists/* \
  && pip install --no-cache-dir pyparsing pydot

WORKDIR /usr/src/app