class Vehicle {
    constructor(data) {
        this.id = data.id;
        this.onTrip = false;
        this.realLatLng = [data.lat, data.lng];
        this.animationStart = performance.now();
        this.delay = [];
        this.updateData(data, false);
    }

    updateData(data, isOnScreen) {
        // perform metadata updates
        if (typeof data.trip_id !== "undefined") {
            this.tripId = data.trip_id;
        }
        this.onTrip = this.tripId ? true : false;
        if (typeof data.real_stop_times !== "undefined") {
            this.realStopTimes = data.real_stop_times;
        }
        if (typeof data.delay !== "undefined") {
            this.delay = data.delay;
        }
        if (data.metadata) {
            if (typeof data.metadata.route_type !== "undefined") {
                this.routeType = data.metadata.route_type;
                this.color = this.recomputeDisplayColor();
            }
            if (typeof data.metadata.route_short_name !== "undefined") {
                this.routeShortName = data.metadata.route_short_name;
            }
            if (typeof data.metadata.route_long_name !== "undefined") {
                this.routeLongName = data.metadata.route_long_name;
            }
            if (typeof data.metadata.trip_headsign !== "undefined") {
                this.tripHeadsign = data.metadata.trip_headsign;
            }
            this.displayText = this.recomputeDisplayText();
            if (typeof data.metadata.agency_name !== "undefined") {
                this.agencyName = data.metadata.agency_name;
            }
            if (typeof data.metadata.stops !== "undefined") {
                this.stops = data.metadata.stops;
            }
        }

        // update animation stuff
        if (data.lat && data.lng) {
            if (isOnScreen) {
                if (this.animatedLatlng) {
                    this.animationStartLatlng = this.animatedLatlng;
                } else {
                    this.animationStartLatlng = this.realLatlng;
                }
            } else {
                this.animationStartLatlng = this.realLatLng;
                this.animatedLatlng = null;
            }
            this.realLatlng = [data.lat, data.lng];
            let timestamp = performance.now();
            let duration = timestamp - this.animationStart;
            this.animationStart = timestamp;
            this.animateUntil = timestamp + duration * 1.5;
        }
    }

    recomputeDisplayText() {
        let text;
        if (!this.tripId) {
            text = "Ej i trafik";
        } else {
            text = "";
            if (this.routeShortName) {
                text += this.routeShortName;
            }
            if (this.routeLongName) {
                if (text.length > 0) {
                    text += " "
                }
                text += this.routeLongName;
            }
            if (this.tripHeadsign) {
                if (text.length > 0) {
                    text += " "
                }
                text += "mot " + this.tripHeadsign;
            }
            if (text.length == 0) {
                text = "Ingen information";
            }
        }
        return text;
    }

    recomputeDisplayColor() {
        if (!this.routeType) {
            return "#8B8B8B";
        }
        // train
        if (this.routeType < 400) {
            return "#FF7600";
        }
        // metro
        if (this.routeType < 700) {
            return "#D61355";
        }
        // bus
        if (this.routeType < 900) {
            return "#0078FF";
        }
        // tram
        if (this.routeType < 1000) {
            return "#2BA714";
        }
        // water
        if (this.routeType == 1000 || this.routeType == 1200) {
            return "#0D1282";
        }
        // taxi
        if (this.routeType >= 1500 && this.routeType <= 1507) {
            return "#FBCB0A";
        }
        // other
        return "#8B8B8B";
    }
}
