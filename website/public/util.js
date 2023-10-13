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
