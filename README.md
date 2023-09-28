# SparkLing

## Notes
- we can try to use this kafka-connect plugin to pull data from the APIs: [kafka-connect-rest](https://github.com/llofberg/kafka-connect-rest)
- we can probably use [leaflet.marker.slideto](https://www.npmjs.com/package/leaflet.marker.slideto) for marker animations
- pyspark natively [integrates](https://spark.apache.org/docs/latest/structured-streaming-kafka-integration.html) with kafka

All the data comes in gtfs format.
For static data, this is a gzip compressed collection of csv file (I think?).
For realtime data, this is a (also compressed?) protobuf message.

## Curl commands
```
# static
curl -X 'GET' 'https://opendata.samtrafiken.se/gtfs-sweden/sweden.zip?key=<key>' -H 'accept: application/octet-stream' -H 'Accept-encoding: gzip' -H 'If-Modified-Since: Mon, 13 Jul 2020 04:24:36 GMT' -H 'If-None-Match: "bfc13a64729c4290ef5b2c2730249c88ca92d82d"' -o static.gtfs.gz

# realtime vehicle positions
curl -X 'GET' 'https://opendata.samtrafiken.se/gtfs-rt-sweden/ul/VehiclePositionsSweden.pb?key=<key>' -H'accept: application/octet-stream' -H 'Accept-encoding: gzip' -H 'If-Modified-Since: Mon, 13 Jul 2020 04:24:36 GMT' -H 'If-None-Match: "bfc13a64729c4290ef5b2c2730249c88ca92d82d"' -o realtime.gtfs.gz

# to inspect the static data:
gtfsutils info static.gtfs.gz
unzip static.gtfs.gz

# to read the realtime data, first compile the protobuf definition, then use the python script or similar:
protoc -I proto/ --python_out=proto/ proto/gtfs-realtime.proto



```
For future refrence:
Entities in the trip update dataset
```
entity {
  id: "33010501569051078"
  trip_update {
    trip {
      trip_id: "33010000189513002"
      start_date: "20230928"
      schedule_relationship: SCHEDULED
    }
    vehicle {
      id: "9031003002600044"
    }
    stop_time_update {
      stop_sequence: 1
      stop_id: "9022050004324003"
      arrival {
        delay: -120
        time: 1695903300
        uncertainty: 0
      }
      departure {
        delay: 0
        time: 1695903420
      }
    }
    stop_time_update {
      stop_sequence: 2
```