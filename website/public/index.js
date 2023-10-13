// hold references to all vehicles
const vehicleHM = new Map();

// holds references to vehicles that should be rendered
var vehiclesOnScreen = new Map();

// global state to control the animation
var mapIsMoving = false; // animation is paused during map movements
const smoothZoomLevel = 10;
var animateSmooth; // smooth animation only if zoomed in

// our custom canvasLayer, used to render vehicles
const canvasLayer = new L.CustomLayer({
    container: document.createElement("canvas"),
    maxZoom: 19,
});
canvasLayer.animate = function() {
    // frame function is called every frame
    let layer = this;
    let frameCounter = 0;
    function frame(timestamp) {
        if (mapIsMoving) {
            return // pause animation when scrolling
        }
        if (animateSmooth || frameCounter % 60 == 0) {
            let canvas = layer.getContainer();
            let ctx = canvas.getContext("2d");
            ctx.clearRect(0, 0, canvas.width, canvas.height);

            let pointRadius = Math.max(1, layer._map.getZoom() - 9);
            vehiclesOnScreen.forEach(function(vehicle, _, _) {
                let point;
                if (vehicle.animateUntil < timestamp) {
                    vehicle.animatedLatlng = vehicle.realLatlng;
                    point = layer._map.latLngToContainerPoint(vehicle.animatedLatlng);
                } else {
                    let animationDuration = vehicle.animateUntil - vehicle.animationStart;
                    let remainingTime = vehicle.animateUntil - timestamp;
                    let percentDone = (animationDuration - remainingTime) / (animationDuration + 1);
                    let startPoint = layer._map.latLngToContainerPoint(vehicle.animationStartLatlng);
                    let endPoint = layer._map.latLngToContainerPoint(vehicle.realLatlng);
                    point = endPoint.multiplyBy(percentDone).add(startPoint.multiplyBy(1 - percentDone));
                    vehicle.animatedLatlng = layer._map.containerPointToLatLng(point);
                }

                // draw
                ctx.beginPath();
                ctx.arc(point.x, point.y, pointRadius, 0, 2*Math.PI);
                ctx.fill();
            });
        }
        frameCounter++;
        window.requestAnimationFrame(frame);
    }

    // reset bounds and start the animation loop
    let { ctx } = layer.setFullLayerBounds();
    ctx.fillStyle = "rgb(0, 100, 255)";
    frame(performance.now());
}
// event handlers for our custom canvasLayer
canvasLayer.on("layer-mounted", function() {
    this._reset();
    this.animate();
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
    animateSmooth = map.getZoom() > smoothZoomLevel;
    canvasLayer.addTo(map);
    return map;
}

// start
const map = initiateLeaflet();

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

// pause animation when map is moving
map.on("movestart", function() {
    mapIsMoving = true;
})
map.on("zoomstart", function() {
    mapIsMoving = true;
})
map.on("zoomend", function() {
    refreshVehiclesOnScreen()
    mapIsMoving = false;
    animateSmooth = this.getZoom() > smoothZoomLevel;
    canvasLayer.animate();
})
// update vehiclesOnScreen when the map moves
map.on('moveend', function(e) {
    refreshVehiclesOnScreen()
    mapIsMoving = false;
    canvasLayer.animate();
});


// define stream source
const evtSource = new EventSource(window.location.origin + "/realtime");

// main event loop
evtSource.onmessage = (event) => {
    let data = JSON.parse(event.data);

    if (vehicleHM.has(data.vehicle_id)) {
        let vehicle = vehicleHM.get(data.vehicle_id);
        let onScreen = isOnScreen(vehicle, map);
        vehicle.updateData(data, onScreen);
        if (onScreen) {
            vehiclesOnScreen.set(vehicle.id, vehicle);
        } else if (vehiclesOnScreen.has(vehicle.id)) {
            vehiclesOnScreen.delete(vehicle.id);
        }
    } else {
        let vehicle = new Vehicle(data);
        vehicleHM.set(vehicle.id, vehicle);
        if (isOnScreen(vehicle, map)) {
            vehiclesOnScreen.set(vehicle.id, vehicle);
        }
    }
};
