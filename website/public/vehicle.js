const Vehicle =  L.Circle.extend({
    initialize: function(data) {
        console.log(data);
        L.Circle.prototype.initialize.call(this, [data.latitude, data.longitude])
        L.setOptions(this, {
            radius: 10,
            color: data.trip_id ? "blue" : "red",
        });
        this.id = data.vehicle_id;
        this.bearing = data.bearing;
        this.speed = data.speed;
        this.lastUpdateTimestamp = performance.now();
    },
    updateData: function(data) {
        console.log(data);
        let newTimestamp = performance.now();
        L.Circle.prototype.slideCancel(); // TODO: is this a fix for the missing latlng???
        L.Circle.prototype.slideTo.call(this,
            [data.latitude, data.longitude],
            { duration: newTimestamp - this.lastUpdateTimestamp});
        L.Circle.prototype.setStyle.call(this, { color: data.trip_id ? "blue" : "red"});
        this.bearing = data.bearing;
        this.speed = data.speed;
        this.lastUpdateTimestamp = newTimestamp;
    },
})
