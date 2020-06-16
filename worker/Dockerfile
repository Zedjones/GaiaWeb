FROM python
WORKDIR /opt/gaia_worker
COPY ./requirements.txt ./
COPY ./Gaia ./Gaia
COPY ./WebWorker.py ./
RUN pip3 install -r requirements.txt
CMD [ "python3", "WebWorker.py" ]