// value = {
//             "vehicle_id": entity.vehicle.vehicle.id,
//             "position": {
//                 "lat": entity.vehicle.position.latitude,
//                 "long": entity.vehicle.position.longitude,
//                 "bearing": entity.vehicle.position.bearing,
//                 "speed": entity.vehicle.position.speed,
//             }
//         }
const Vehicle =  L.Circle.extend({
    initialize: function(data, options = { radius: 50 }) {
        L.Circle.prototype.initialize.call(this, [data.position.lat, data.position.long]) 
        L.setOptions(this, options);
        this.id = data.vehicle_id;
        this.bearing = data.position.bearing;
        this.speed = data.position.speed;
        this.lastUpdateTimestamp = performance.now()
    },
    updateData: function(data) {
        let newTimestamp = performance.now();
        L.Circle.prototype.slideTo.call(this,
            [data.position.lat, data.position.long],
            { duration: newTimestamp - this.lastUpdateTimestamp});
        this.bearing = data.position.bearing;
        this.speed = data.position.speed;
        this.lastUpdateTimestamp = newTimestamp;
    },
})
