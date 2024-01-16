// factory function that can fail if required data is missing
function newVehicle(data) {
    if (data.id && data.lat && data.lng && data.trip_id && data.route_type) {
        return new Vehicle(data);
    } else {
        return undefined;
    }
}

class Vehicle {
    constructor(data) {
        this._assign(data);
        this.color = routeTypeToColor(data.route_type);
        this.initAnimationState();
    }
    updateData(data, onScreen) {
        this._assign(data);
        if ([this.lat, this.lng] !== this.animationEndLatlng) {
            this.updateAnimationState(onScreen);
        }
    }
    _assign(data) {
        for (const key of Object.keys(data)) {
            const val = data[key];
            if (val !== undefined) {
                this[key] = val;
            }
        }
    }
    initAnimationState() {
        let timestamp = performance.now();
        this.animationStartLatlng = [this.lat, this.lng];
        this.animationEndLatlng = this.animationStartLatlng;
        this.animatedLatlng = this.animationStartLatlng;
        this.animationStart = timestamp;
        this.animateUntil = timestamp;
    }
    updateAnimationState(onScreen) {
        let timestamp = performance.now();
        let duration = timestamp - this.animationStart;
        if (onScreen) {
            if (this.animatedLatlng) {
                this.animationStartLatlng = this.animatedLatlng;
            } else {
                this.animationStartLatlng = this.animationEndLatlng;
            }
        } else {
            this.animationStartLatlng = this.animationEndLatlng;
            this.animatedLatlng = null;
        }
        this.animationEndLatlng = [this.lat, this.lng];
        this.animationStart = timestamp;
        this.animateUntil = timestamp + duration * 1.5;
    }
    getDisplayText() {
        let text = "";
        if (this.route_short_name) {
            text += this.route_short_name;
        }
        if (this.route_long_name) {
            if (text.length > 0) {
                text += " "
            }
            text += this.route_long_name;
        }
        if (this.trip_headsign) {
            if (text.length > 0) {
                text += " "
            }
            text += "mot " + this.trip_headsign;
        }
        if (text.length == 0) {
            text = "Ingen information";
        }
        return text;
    }
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
