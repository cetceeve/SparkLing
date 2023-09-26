# SparkLing

## Notes
- we can try to use this kafka-connect plugin to pull data from the APIs: [kafka-connect-rest](https://github.com/llofberg/kafka-connect-rest)
- we can probably use [leaflet.marker.slideto](https://www.npmjs.com/package/leaflet.marker.slideto) for marker animations

All the data comes in gtfs format.
For static data, this is a gzip compressed collection of csv file (I think?).
For realtime data, this is a (also compressed?) protobuf message.

## Curl commands
```
# static
curl -X 'GET' 'https://opendata.samtrafiken.se/gtfs-sweden/sweden.zip?key=f620df859fb141f39e0bbc3f1abad609' -H 'accept: application/octet-stream' -H 'Accept-encoding: gzip' -H 'If-Modified-Since: Mon, 13 Jul 2020 04:24:36 GMT' -H 'If-None-Match: "bfc13a64729c4290ef5b2c2730249c88ca92d82d"' -o static.gtfs.gz

# realtime vehicle positions
curl -X 'GET' 'https://opendata.samtrafiken.se/gtfs-rt-sweden/ul/VehiclePositionsSweden.pb?key=92fccb7fca894b499bce18d23faa9d94' -H'accept: application/octet-stream' -H 'Accept-encoding: gzip' -H 'If-Modified-Since: Mon, 13 Jul 2020 04:24:36 GMT' -H 'If-None-Match: "bfc13a64729c4290ef5b2c2730249c88ca92d82d"' -o realtime.gtfs.gz

# to inspect the static data:
gtfsutils info static.gtfs.gz
unzip static.gtfs.gz

# to read the realtime data, first compile the protobuf definition, then use the python script or similar:
protoc -I proto/ --python_out=proto/ proto/gtfs-realtime.proto
```
