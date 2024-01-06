FROM python:3.11

COPY ./requirements.txt ./
RUN python3 -m pip install --upgrade pip setuptools wheel
RUN pip3 install -r requirements.txt

COPY . .

ENTRYPOINT ["python", "main.py"]
