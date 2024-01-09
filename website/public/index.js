// data store for reactive ui
document.addEventListener('alpine:init', () => {
    Alpine.store('sv', {
        displayText: "transitmap.io",
        m: {},
        delay: [],
        firstPredictedSequence: 1000,
        
        update(vehicle) {
            // console.log(vehicle);
            this.displayText = vehicle.displayText;
            this.m = vehicle.metadata ? vehicle.metadata : {};
            this.delay = vehicle.delay ? vehicle.delay : [];
            this.firstPredictedSequence = vehicle.firstPredictedSequence ? vehicle.firstPredictedSequence : 1000;
        }
    })
})

// hold references to all vehicles
const vehicleHM = new Map();

// holds references to vehicles that should be rendered
var vehiclesOnScreen = new Map();

// holds a reference to the currently selected vehicle
var selectedVehicle;
var userPosition;

var mapIsMoving = false; // animation is paused during map movements
const smoothZoomLevel = 10;
const clickableZoomLevel = 13;
var animateSmooth; // smooth animation only if zoomed in

// vehicle icons
const busIcon = new Image();
busIcon.src = "./images/bus_icon.png";
const trainIcon = new Image();
trainIcon.src = "./images/train_icon.png";
const tramIcon = new Image();
tramIcon.src = "./images/tram_icon.png";
const metroIcon = new Image();
metroIcon.src = "./images/metro_icon.png";
const ferryIcon = new Image();
ferryIcon.src = "./images/ferry_icon.png";
const taxiIcon = new Image();
taxiIcon.src = "./images/taxi_icon.png";
const otherIcon = new Image();
otherIcon.src = "./images/other_icon.png";
const iconsByColor = {
    "#FF7600": trainIcon,
    "#D61355": metroIcon,
    "#0078FF": busIcon,
    "#2BA714": tramIcon,
    "#0D1282": ferryIcon,
    "#FBCB0A": taxiIcon,
    "#8B8B8B": otherIcon,
}

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

            // draw vehicles
            let pointRadius = zoomToPointRadius(layer._map.getZoom());
            vehiclesOnScreen.forEach(function(vehicle, _, _) {
                let point;
                if (vehicle.animateUntil < timestamp) {
                    vehicle.animatedLatlng = null;
                    point = layer._map.latLngToContainerPoint(vehicle.realLatlng);
                    vehicle.containerPoint = point;
                } else {
                    let animationDuration = vehicle.animateUntil - vehicle.animationStart;
                    let remainingTime = vehicle.animateUntil - timestamp;
                    let percentDone = (animationDuration - remainingTime) / (animationDuration + 1);
                    let startPoint = layer._map.latLngToContainerPoint(vehicle.animationStartLatlng);
                    let endPoint = layer._map.latLngToContainerPoint(vehicle.realLatlng);
                    point = endPoint.multiplyBy(percentDone).add(startPoint.multiplyBy(1 - percentDone));
                    vehicle.containerPoint = point;
                    if (animateSmooth) {
                        vehicle.animatedLatlng = layer._map.containerPointToLatLng(point);
                    } else {
                        vehicle.animatedLatlng = null;
                    }
                }

                // draw
                if (pointRadius < 7) {
                    ctx.beginPath();
                    ctx.fillStyle = vehicle.color;
                    ctx.arc(point.x, point.y, pointRadius, 0, 2*Math.PI);
                    ctx.fill();
                } else {
                    ctx.drawImage(iconsByColor[vehicle.color], point.x-pointRadius, point.y-pointRadius, 2*pointRadius, 2*pointRadius);
                }
            });
            // highlight selected vehicle
            if (selectedVehicle && vehiclesOnScreen.has(selectedVehicle.id)) {
                // draw the vehicle again on top
                if (pointRadius < 7) {
                    ctx.beginPath();
                    ctx.fillStyle = selectedVehicle.color;
                    ctx.arc(selectedVehicle.containerPoint.x, selectedVehicle.containerPoint.y, pointRadius, 0, 2*Math.PI);
                    ctx.fill();
                } else {
                    ctx.drawImage(iconsByColor[selectedVehicle.color], selectedVehicle.containerPoint.x-pointRadius, selectedVehicle.containerPoint.y-pointRadius, 2*pointRadius, 2*pointRadius);
                }
                // draw highlight circle
                ctx.beginPath();
                ctx.strokeStyle = "red";
                ctx.lineWidth = 3
                ctx.arc(selectedVehicle.containerPoint.x, selectedVehicle.containerPoint.y, pointRadius+1, 0, 2*Math.PI);
                ctx.stroke();
                // draw textbox
                let textMetrics = ctx.measureText(selectedVehicle.displayText);
                let textHeight = textMetrics.actualBoundingBoxAscent + textMetrics.actualBoundingBoxDescent;
                let textWidth = textMetrics.actualBoundingBoxLeft + textMetrics.actualBoundingBoxRight;
                let textLeft = selectedVehicle.containerPoint.x + 3;
                let textBottom = selectedVehicle.containerPoint.y - pointRadius - 7 - textMetrics.actualBoundingBoxDescent;
                let textTop = textBottom - textMetrics.actualBoundingBoxAscent;
                ctx.beginPath();
                ctx.fillStyle = "white";
                ctx.rect(textLeft-2, textTop-2, textWidth+5, textHeight+5);
                ctx.fill();
                ctx.beginPath();
                ctx.strokeStyle = "black";
                ctx.lineWidth = 2
                ctx.rect(textLeft-2, textTop-2, textWidth+5, textHeight+5);
                ctx.stroke();
                ctx.fillStyle = "black";
                ctx.fillText(selectedVehicle.displayText, textLeft, textBottom);
            }
            // draw user location
            if (userPosition) {
                let point = layer._map.latLngToContainerPoint(userPosition.latlng);
                ctx.fillStyle = "#24b6ff";
                ctx.beginPath();
                ctx.arc(point.x, point.y, 6, 0, 2*Math.PI);
                ctx.fill();
                ctx.beginPath();
                ctx.globalAlpha = 0.3;
                ctx.arc(point.x, point.y, metersToPixels(userPosition.accuracy, map), 0, 2*Math.PI);
                ctx.fill();
                ctx.globalAlpha = 1.0;
            }
        }
        frameCounter++;
        window.requestAnimationFrame(frame);
    }

    // reset bounds and start the animation loop
    let { ctx } = layer.setFullLayerBounds();
    ctx.font = "20px arial"
    frame(performance.now());
}
// event handlers for our custom canvasLayer
canvasLayer.on("layer-mounted", function() {
    this._reset();
    this.animate();
});

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
        if (selectedVehicle.onTrip) {
            Alpine.store("sv").update(selectedVehicle);
        }
    }
});
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


// define stream source
const evtSource = new EventSource(window.location.origin + "/realtime");

// main event loop
evtSource.onmessage = (event) => {
    let data = JSON.parse(event.data);

    if (vehicleHM.has(data.id)) {
        let vehicle = vehicleHM.get(data.id);
        let onScreen = isOnScreen(vehicle, map);
        vehicle.updateData(data, onScreen);
        if (onScreen) {
            vehiclesOnScreen.set(vehicle.id, vehicle);
        } else if (vehiclesOnScreen.has(vehicle.id)) {
            vehiclesOnScreen.delete(vehicle.id);
        }
        // Trigger reactive update.
        if (vehicle.id === selectedVehicle?.id) {
            Alpine.store("sv").update(vehicle);
        }
    } else {
        let vehicle = new Vehicle(data);
        vehicleHM.set(vehicle.id, vehicle);
        if (isOnScreen(vehicle, map)) {
            vehiclesOnScreen.set(vehicle.id, vehicle);
        }
    }
};
