function checkBounds(vehicle, eastBound, westBound, northBound, southBound) {
    return (
        (
            (vehicle.animationEndLatlng[0] < northBound && vehicle.animationEndLatlng[0] > southBound)
            || (vehicle.animationStartLatlng[0] < northBound && vehicle.animationStartLatlng[0] > southBound)
        ) && (
            (
                eastBound < westBound && (
                    (vehicle.animationEndLatlng[1] > eastBound && vehicle.animationEndLatlng[1] < westBound)
                    || (vehicle.animationStartLatlng[1] > eastBound && vehicle.animationStartLatlng[1] < westBound)
                )
            ) || (
                eastBound > westBound && (
                    (vehicle.animationEndLatlng[1] < eastBound && vehicle.animationEndLatlng[1] > westBound)
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
    let pointRadius = Math.min(
        16,
        Math.pow(
            1.5,
            Math.max(0, zoom - 9)
        )
    );
    if (zoom < clickableZoomLevel && pointRadius > 1) {
        pointRadius -= 1;
    }
    return Math.ceil(pointRadius);
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

async function getMetadata(trip_id) {
    let resp = await fetch(window.location.origin + "/trip_metadata/" + trip_id);
    let metadata = await resp.json();
    return metadata;
}
