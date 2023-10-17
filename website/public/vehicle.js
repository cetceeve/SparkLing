function dataToDisplayText(data) {
    let text;
    if (!data.trip_id) {
        text = "Ej i traffik";
    } else {
        text = "";
        if (data.route_short_name) {
            text += data.route_short_name;
        }
        if (data.route_long_name) {
            if (text.length > 0) {
                text += " "
            }
            text += data.route_long_name;
        }
        if (data.trip_headsign) {
            if (text.length > 0) {
                text += " "
            }
            text += "mot " + data.trip_headsign;
        }
        if (text.length == 0) {
            text = "Ingen information";
        }
    }
    return text;
}

function routeTypeToColor(routeType) {
    if (!routeType) {
        return "#8B8B8B";
    }
    // train
    if (routeType < 400) {
        return "#FF7600";
    }
    // metro
    if (routeType < 700) {
        return "#D61355";
    }
    // bus
    if (routeType < 900) {
        return "#0078FF";
    }
    // tram
    if (routeType < 1000) {
        return "#2BA714";
    }
    // water
    if (routeType == 1000 || routeType == 1200) {
        return "#0D1282";
    }
    // taxi
    if (routeType >= 1500 && routeType <= 1507) {
        return "#FBCB0A";
    }
    // other
    return "#8B8B8B";
}

class Vehicle {
    constructor(data) {
        this.id = data.vehicle_id;
        this.onTrip = data.trip_id ? true : false;
        this.displayText = dataToDisplayText(data);
        this.routeType = data.route_type;
        this.color = routeTypeToColor(data.route_type);
        let timestamp = performance.now();
        this.realLatlng = [data.latitude, data.longitude];
        this.animatedLatlng = this.realLatlng;
        this.animationStartLatlng = this.realLatlng;
        this.animationStart = timestamp;
        this.animateUntil = timestamp;
    }
    updateData(data, isOnScreen) {
        this.onTrip = data.trip_id ? true : false;
        this.displayText = dataToDisplayText(data);
        this.routeType = data.route_type;
        this.color = routeTypeToColor(data.route_type);
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
