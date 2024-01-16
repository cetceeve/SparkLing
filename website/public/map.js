{ // we use JS scoping, to make sure nothing leaks into global scope
    function initiateLeaflet() {
        let map = L.map('map', {
            center:  [59.34563446044922, 18.071327209472656],
            zoom: 16,
            renderer: L.canvas(),
            zoomControl: false,
        });
        L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
            maxZoom: 19,
            attribution: '&copy; <a href="http://www.openstreetmap.org/copyright">OpenStreetMap</a>'
        }).addTo(map);

        return map;
    }

    map = initiateLeaflet();

    map.locate({ watch: true, enableHighAccuracy: true });
    map.on("locationfound", function(e) {
        let latlng = [e.latitude, e.longitude];
        if (!userPosition) {
            this.flyTo(latlng, 16);
        }
        userPosition = {
            latlng,
            accuracy: e.accuracy,
        };
    });
    map.on("click", function(e) {
        let zoom = this.getZoom()
        if (zoom < clickableZoomLevel) {
            selectedVehicle = undefined;
            return
        }
        let closestDist = zoomToPointRadius(zoom) + 2;
        let closestVehicle = undefined;
        vehiclesOnScreen.forEach(function(vehicle, _, _) {
            if (vehicle.containerPoint) {
                let dist = vehicle.containerPoint.distanceTo(e.containerPoint);
                if (dist < closestDist) {
                    closestDist = dist;
                    closestVehicle = vehicle;
                }
            }
        });
        selectedVehicle = closestVehicle;
        if (selectedVehicle) {
            Alpine.store("selectedVehicle").update(selectedVehicle);
        }
    });
}
