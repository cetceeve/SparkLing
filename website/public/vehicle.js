function dataToDisplayText(data) {
    let text;
    if (!data.trip_id) {
        text = "Ej i trafik";
    } else if (!data.metadata) {
        text = "Ingen information";
    } else {
        text = "";
        if (data.metadata.route_short_name) {
            text += data.metadata.route_short_name;
        }
        if (data.metadata.route_long_name) {
            if (text.length > 0) {
                text += " "
            }
            text += data.metadata.route_long_name;
        }
        if (data.metadata.trip_headsign) {
            if (text.length > 0) {
                text += " "
            }
            text += "mot " + data.metadata.trip_headsign;
        }
        if (text.length == 0) {
            text = "Ingen information";
        }
    }
    return text;
}

function dataToColor(data) {
    if (!data.metadata || !data.metadata.route_type) {
        return "#8B8B8B";
    }
    let routeType = data.metadata.route_type;

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
        this.id = data.id;
        this.onTrip = data.trip_id ? true : false;
        this.displayText = dataToDisplayText(data);
        this.color = dataToColor(data);
        let timestamp = performance.now();
        this.realLatlng = [data.lat, data.lng];
        this.animationStartLatlng = this.realLatlng;
        this.animationStart = timestamp;
        this.animateUntil = timestamp;
    }
    updateData(data, isOnScreen) {
        this.onTrip = data.trip_id ? true : false;
        this.displayText = dataToDisplayText(data);
        this.color = dataToColor(data);
        let timestamp = performance.now();
        let duration = timestamp - this.animationStart;
        if (isOnScreen) {
            if (this.animatedLatlng) {
                this.animationStartLatlng = this.animatedLatlng;
            } else {
                this.animationStartLatlng = this.realLatlng;
            }
        } else {
            this.animationStartLatlng = this.realLatlng;
            this.animatedLatlng = null;
        }
        this.realLatlng = [data.lat, data.lng];
        this.animationStart = timestamp;
        this.animateUntil = timestamp + duration * 1.5;
    }
}
