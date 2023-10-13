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
        this.realLatlng = [data.latitude, data.longitude];
        this.animatedLatlng = this.realLatlng;
        this.animationStartLatlng = this.realLatlng;
        this.animationStart = timestamp;
        this.animateUntil = timestamp;
    }
    updateData(data, isOnScreen) {
        this.onTrip = data.trip_id ? true : false;
        if (this.on_trip) {
            this.agencyName = data.agency_name;
            this.routeShortName = data.route_short_name;
            this.routeLongName = data.route_long_name;
        }
        let timestamp = performance.now();
        let duration = timestamp - this.animationStart;
        if (isOnScreen) {
            this.animationStartLatlng = this.animatedLatlng;
        } else {
            this.animationStartLatlng = this.realLatlng;
            this.animatedLatlng = this.animationStartLatlng;
        }
        this.realLatlng = [data.latitude, data.longitude];
        this.animationStart = timestamp;
        this.animateUntil = timestamp + duration * 1.5;
    }
}
