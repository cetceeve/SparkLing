# SparkLing

SparkLing is an interactive realtime visualisation of all public transport in Sweden (or those parts of it that have realtime geolocations, anyway).

## How to run locally
SparkLing can run locally inside docker-compose.
This requires a small amount of setup, as follows.

### TrafikLab API Keys
To run SparkLink yourself, you need to provide your own API keys from [TrafikLab](https://www.trafiklab.se/).
This is completely free of charge. Follow these steps to set it up:

1. Login or create an account on [TrafikLab](https://www.trafiklab.se/).
2. Create a new project in your TrafikLab account.
3. Add API keys for the `GTFS Sweden Realtime (beta)` and `GTFS Sweden Static data (beta)` APIs to your project.
4. Create a file named `.env` in the root directory of this project and add your API keys to it. It should look like the following.

```
TRAFIKLAB_GTFS_RT_KEY=<realtime-api-key>
TRAFIKLAB_GTFS_STATIC_KEY=<static-data-api-key>
```
Note that, while the API keys you have just set up are are perfectly fine for testing, they not enough to run SparkLing continuously.
For this, the `Guld` API tier is required on the realtime API. This can also be requested from TrafikLab free of charge, but processing
the request typically takes a couple days.

### Running
Once you have set up your API keys, you can simply run SparkLing with the following command.
```
docker-compose up --build
```
The cluster takes a couple minutes to start fully.
Once everything is running, you can connect to the application in your browser on [localhost](http://localhost:80).
