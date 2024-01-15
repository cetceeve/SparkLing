function checkBounds(vehicle, eastBound, westBound, northBound, southBound) {
    return (
        (
            (vehicle.realLatlng[0] < northBound && vehicle.realLatlng[0] > southBound)
            || (vehicle.animationStartLatlng[0] < northBound && vehicle.animationStartLatlng[0] > southBound)
        ) && (
            (
                eastBound < westBound && (
                    (vehicle.realLatlng[1] > eastBound && vehicle.realLatlng[1] < westBound)
                    || (vehicle.animationStartLatlng[1] > eastBound && vehicle.animationStartLatlng[1] < westBound)
                )
            ) || (
                eastBound > westBound && (
                    (vehicle.realLatlng[1] < eastBound && vehicle.realLatlng[1] > westBound)
                    || (vehicle.animationStartLatlng[1] < eastBound && vehicle.animationStartLatlng[1] > westBound)
                )
            )
        )
    )
}

function isOnScreen(vehicle, map) {
    let mapBounds = map.getBounds();
    let eastBound = mapBounds.getEast();
    let westBound = mapBounds.getWest();
    let northBound = mapBounds.getNorth();
    let southBound = mapBounds.getSouth();
    return checkBounds(vehicle, eastBound, westBound, northBound, southBound);
}

function zoomToPointRadius(zoom) {
    let pointRadius = Math.max(1, zoom - 9);
    if (pointRadius > 6) {
        pointRadius += 5;
    }
    return pointRadius;
}

function metersToPixels(meters, map) {
    let metersPerPixel = 40075016.686 * Math.abs(Math.cos(map.getCenter().lat * Math.PI/180)) / Math.pow(2, map.getZoom()+8);
    return Math.ceil(meters / metersPerPixel);
}

function refreshVehiclesOnScreen() {
    let mapBounds = map.getBounds();
    let eastBound = mapBounds.getEast();
    let westBound = mapBounds.getWest();
    let northBound = mapBounds.getNorth();
    let southBound = mapBounds.getSouth();
    let newVehiclesOnScreen = new Map();
    vehicleHM.forEach(function(val, key, _) {
        if (checkBounds(val, eastBound, westBound, northBound, southBound)) {
            newVehiclesOnScreen.set(key, val);
        }
    });
    vehiclesOnScreen = newVehiclesOnScreen;
}
