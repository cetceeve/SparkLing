// hold references to all vehicles
const vehicleHM = new Map();

// holds references to vehicles that should be rendered
var vehiclesOnScreen = new Map();


// our custom canvasLayer, used to render vehicles
const canvasLayer = new L.CustomLayer({
    container: document.createElement("canvas"),
    padding: 0,
    zIndex: 1000,
});

// var canvasCTX;
// event handlers for our custom canvasLayer
canvasLayer.on("layer-render", function() {
    // update canvas bounds when the map moves or resizes
    let { ctx } = this.setFullLayerBounds();
    ctx.fillStyle = "rgb(0, 100, 255)";
});
canvasLayer.on("layer-mounted", function() {
    let layer = this;

    // frame function is called every frame
    function frame(timestamp) {
        // let { ctx } = layer.setFullLayerBounds();
        let canvas = layer.getContainer();
        let ctx = canvas.getContext("2d");
        ctx.clearRect(0, 0, canvas.width, canvas.height);

        // let pointRadius = layer._map.getZoom();
        let pointRadius = 5; // TODO: better size calculation
        vehiclesOnScreen.forEach(function(vehicle, _, _) {
            let currPoint;
            while (vehicle.waypoints.length > 1 && vehicle.waypoints[1].animateUntil < timestamp) {
                vehicle.waypoints.shift(); // remove old waypoints
            }
            if (vehicle.waypoints.length > 3) {
                // DEBUG
                console.log(vehicle.waypoints.length);
            }
            if (vehicle.waypoints.length == 1) {
                currPoint = layer._map.latLngToContainerPoint(vehicle.waypoints[0].latlng);
            } else {
                let animationDuration = vehicle.waypoints[1].animateUntil - vehicle.waypoints[1].timestamp;
                let remainingTime = vehicle.waypoints[1].animateUntil - timestamp;
                let percentDone = (animationDuration - remainingTime) / animationDuration;
                let startPoint = layer._map.latLngToContainerPoint(vehicle.waypoints[0].latlng);
                let endPoint = layer._map.latLngToContainerPoint(vehicle.waypoints[1].latlng);
                currPoint = endPoint.multiplyBy(percentDone).add(startPoint.multiplyBy(1 - percentDone));
            }

            // draw
            ctx.beginPath();
            ctx.arc(currPoint.x, currPoint.y, pointRadius, 0, 2*Math.PI);
            ctx.fill();
        });
        window.requestAnimationFrame(frame);
    }
    window.requestAnimationFrame(frame);
});





function initiateLeaflet() {
    let map = L.map('map', {
        center: [59.85825859695506, 17.647523157741706],
        zoom: 15,
        renderer: L.canvas(),
    });
    L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
        maxZoom: 19,
        attribution: '&copy; <a href="http://www.openstreetmap.org/copyright">OpenStreetMap</a>'
    }).addTo(map);
    // L.tileLayer('https://tile.thunderforest.com/pioneer/{z}/{x}/{y}.png?apikey=131eec8b12b3471cb55357e19c1c1b62', {
    //     maxZoom: 19,
    // }).addTo(map);

    // add out custom canvasLayer
    canvasLayer.addTo(map);
    return map;
}

// start
const map = initiateLeaflet();

function isOnScreen(vehicle) {
    let mapBounds = map.getBounds();
    let eastBound = mapBounds.getEast();
    let westBound = mapBounds.getWest();
    let northBound = mapBounds.getNorth();
    let southBound = mapBounds.getSouth();

    for (waypoint of vehicle.waypoints) {
        let lat = waypoint.latlng[0];
        let lng = waypoint.latlng[1];
        if (lat > northBound || lat < southBound) continue
        if (eastBound < westBound) {
            if (lng < eastBound || lng > westBound) continue
        } else {
            if (lng > eastBound || lng < westBound) continue
        }
        return true
    }
    return false
}

// update vehiclesOnScreen when the map moves
map.on('moveend', function(e) {
    let mapBounds = this.getBounds();
    let eastBound = mapBounds.getEast();
    let westBound = mapBounds.getWest();
    let northBound = mapBounds.getNorth();
    let southBound = mapBounds.getSouth();

    let newVehiclesOnScreen = new Map();

    vehicleHM.forEach(function(val, key, _) {
        let isInBounds = false;
        for (waypoint of val.waypoints) {
            let lat = waypoint.latlng[0];
            let lng = waypoint.latlng[1];
            if (lat > northBound || lat < southBound) continue
            if (eastBound < westBound) {
                if (lng < eastBound || lng > westBound) continue
            } else {
                if (lng > eastBound || lng < westBound) continue
            }
            isInBounds = true;
            break
        }
        if (isInBounds) {
            newVehiclesOnScreen.set(key, val);
        }
    });

    vehiclesOnScreen = newVehiclesOnScreen;
});




// define stream source
const evtSource = new EventSource(window.location.origin + "/realtime");

// main event loop
evtSource.onmessage = (event) => {
    let data = JSON.parse(event.data);

    if (vehicleHM.has(data.vehicle_id)) {
        let vehicle = vehicleHM.get(data.vehicle_id);
        vehicle.updateData(data);
        if (isOnScreen(vehicle)) {
            vehiclesOnScreen.set(vehicle.id, vehicle);
        } else if (vehiclesOnScreen.has(vehicle.id)) {
            vehiclesOnScreen.delete(vehicle.id);
        }
    } else {
        let vehicle = new Vehicle(data);
        vehicleHM.set(vehicle.id, vehicle);
        if (isOnScreen(vehicle)) {
            vehiclesOnScreen.set(vehicle.id, vehicle);
        }
    }
};
