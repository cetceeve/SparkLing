// value = {
//             "vehicle_id": entity.vehicle.vehicle.id,
//             "trip_id": entity.vehicle.trip.trip_id,
//             "position": {
//                 "lat": entity.vehicle.position.latitude,
//                 "long": entity.vehicle.position.longitude,
//                 "bearing": entity.vehicle.position.bearing,
//                 "speed": entity.vehicle.position.speed,
//             }
//         }
const Vehicle =  L.Circle.extend({
    initialize: function(data) {
        L.Circle.prototype.initialize.call(this, [data.position.lat, data.position.long])
        L.setOptions(this, {
            radius: 10,
            color: (data.trip_id.length === 0) ? "red" : "blue",
        });
        this.id = data.vehicle_id;
        this.bearing = data.position.bearing;
        this.speed = data.position.speed;
        this.lastUpdateTimestamp = performance.now();
    },
    updateData: function(data) {
        let newTimestamp = performance.now();
        L.Circle.prototype.slideTo.call(this,
            [data.position.lat, data.position.long],
            { duration: newTimestamp - this.lastUpdateTimestamp});
        L.Circle.prototype.setStyle.call(this, { color: (data.trip_id.length === 0) ? "red" : "blue"});
        this.bearing = data.position.bearing;
        this.speed = data.position.speed;
        this.lastUpdateTimestamp = newTimestamp;
    },
})
