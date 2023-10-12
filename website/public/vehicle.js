class Vehicle {
    constructor(data) {
        this.id = data.vehicle_id;
        this.onTrip = data.trip_id ? true : false;
        if (this.on_trip) {
            this.agencyName = data.agency_name;
            this.routeShortName = data.route_short_name;
            this.routeLongName = data.route_long_name;
        }
        let timestamp = performance.now();
        this.waypoints = [{
            timestamp: timestamp,
            animateUntil: timestamp,
            latlng: [data.latitude, data.longitude],
        }];
    }
    updateData(data) {
        this.onTrip = data.trip_id ? true : false;
        if (this.on_trip) {
            this.agencyName = data.agency_name;
            this.routeShortName = data.route_short_name;
            this.routeLongName = data.route_long_name;
        }
        let timestamp = performance.now();
        let prevWaypoint = this.waypoints[this.waypoints.length-1];
        // let duration = timestamp - prevWaypoint.timestamp;
        let duration = 1000;
        if (prevWaypoint.animateUntil > timestamp) {
            prevWaypoint.animateUntil = Math.min(prevWaypoint.animateUntil, timestamp + (duration / 3)); // cut prev animation short
        }
        this.waypoints.push({
            timestamp: timestamp,
            animateUntil: timestamp + duration,
            latlng: [data.latitude, data.longitude],
        });
    }
}
